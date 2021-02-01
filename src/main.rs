extern crate async_trait;
#[macro_use]
extern crate colour;
#[macro_use]
extern crate kubes_std_lib;
extern crate tokio;

use crate::main_tick_data::TickData;
use credentials::client_id::ClientId;
use irc_chat::{
    channel_chatter_data::ChatterData,
    parsers::{
        default_irc_message_parser::DefaultMessageParser,
        pubsub::default_message_parser::DefaultPubSubParser,
    },
    web_socket_session::WebSocketSession,
};
pub use kubes_std_lib::logging::{DefaultLogger, Logger};
pub use kubes_web_lib::web_request::{is_html, is_json};
use oauth::has_oauth_signature::HasOauthSignature;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use secrets::{CLIENT_ID, CLIENT_SECRET};
use std::error::Error;
use std::{convert::TryFrom, str::FromStr, sync::Arc, time::Instant};
use tokio::time::{delay_for as sleep, Duration};
use user::{oauth_token::OauthToken as UserOauthToken, user_data::UserData};
use websocket::{
    stream::sync::{TcpStream, TlsStream},
    url::Url,
};

pub mod credentials;
pub mod irc_chat;
pub mod main_tick_data;
pub mod oauth;
pub mod save_data;
pub mod secrets;
mod send_error;
pub mod user;
pub mod web_requests;

const TICK_RATE: u64 = 1000;

#[tokio::main]
async fn main() {
    let (token, user) = match init_token_and_user(&DefaultLogger {}).await {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    match start_chat_session(token.clone(), user.clone()).await {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    };

    match start_pubsub_session(token.clone(), user.clone()).await {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    };

    match tick(token, user, TICK_RATE).await {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    };
}

/// Interacts with Twitch Events such as subs and channel points spending
async fn start_pubsub_session(token: UserOauthToken, user: UserData) -> Result<(), Box<dyn Error>> {
    let pubsub_url = Url::from_str("wss://pubsub-edge.twitch.tv")?;
    // create PubSub-WebSocket
    let pubsub_session = WebSocketSession::new(
        user.clone(),
        token.clone(),
        DefaultPubSubParser::new(),
        DefaultLogger {},
        pubsub_url,
    );
    let pubsub_arc = Arc::new(tokio::sync::Mutex::new(pubsub_session));

    let temp_channel_id = user.clone().get_user_id().get_value();
    let temp_oauth = token.clone().get_oauth_signature().get_value();
    //
    let on_pubsub_start =
        move |session: &mut WebSocketSession<DefaultPubSubParser, DefaultLogger>,
              listener: &mut websocket::sync::Client<TlsStream<TcpStream>>| {
            session.send_string(
                listener,
                format!(
                    "{{\
        \"type\": \"LISTEN\",\
        \"nonce\": \"333\",\
        \"data\": {{\
        \"topics\": [
        \"channel-points-channel-v1.{0}\"
        ],\
        \"auth_token\": \"{1}\"\
        }}
        }}",
                    temp_channel_id, temp_oauth
                ),
            );
        };
    //
    WebSocketSession::initialize(pubsub_arc, on_pubsub_start).await;
    Ok(())
}

/// Interacts with Twitch IRC chat in a designated channel
async fn start_chat_session(token: UserOauthToken, user: UserData) -> Result<(), Box<dyn Error>> {
    let chat_url = websocket::url::Url::from_str("wss://irc-ws.chat.twitch.tv:443")?;
    // create Chat-IRC
    let chat_session = WebSocketSession::new(
        user.clone(),
        token.clone(),
        DefaultMessageParser::new(),
        DefaultLogger {},
        chat_url,
    );
    //
    // run Chat-IRC
    let chat_irc_arc = Arc::new(tokio::sync::Mutex::new(chat_session));
    let token_temp = token.clone();
    let user_temp = user.clone();
    let on_chat_start =
        move |session: &mut WebSocketSession<DefaultMessageParser, DefaultLogger>,
              listener: &mut websocket::sync::Client<TlsStream<TcpStream>>| {
            // authenticate user (login)
            session.send_string(
                listener,
                format!(
                    "PASS oauth:{}",
                    token_temp.clone().get_oauth_signature().get_value()
                ),
            );
            session.send_string(
                listener,
                format!("NICK {}", user_temp.clone().get_login().get_value()),
            );
            session.send_string(
                listener,
                format!("JOIN #{}", user_temp.get_login().get_value()),
            );
        };
    WebSocketSession::initialize(chat_irc_arc.clone(), on_chat_start).await;
    Ok(())
}

/// Get OAuth token and the logged in client_user's info
async fn init_token_and_user<TLogger>(
    logger: &TLogger,
) -> Result<(UserOauthToken, UserData), Box<dyn Error>>
where
    TLogger: Logger,
{
    let client = Client::builder().build()?;

    // get user token
    let token = UserOauthToken::request(
        &client,
        ClientId::new(CLIENT_ID.to_string()),
        CLIENT_SECRET,
        logger,
    )
    .await?;

    // get logged in user's info
    let user_data =
        user::user_data::UserData::get_from_bearer_token(&client, token.clone(), logger).await?;

    Ok((token, user_data))
}

async fn tick(token: UserOauthToken, user: UserData, tick_rate: u64) -> Result<(), Box<dyn Error>> {
    let reqwest_client = reqwest::Client::builder().build()?;
    let mut tick_data = Default::default();

    sleep(Duration::from_millis(tick_rate)).await; // don't trigger before WebSockets start

    loop {
        let before_tick_instant = Instant::now();

        tick_routine(&reqwest_client, user.clone(), token.clone(), &mut tick_data).await?;

        let tick_elapsed_time = {
            match u64::try_from(before_tick_instant.elapsed().as_millis()) {
                Ok(new64) => {
                    if new64 >= tick_rate {
                        tick_rate - 1
                    } else {
                        new64
                    }
                }
                Err(_) => tick_rate - 1,
            }
        };
        sleep(Duration::from_millis(tick_rate - tick_elapsed_time)).await;
    }
}
//
async fn tick_routine(
    reqwest_client: &reqwest::Client,
    client_user: UserData,
    bearer_token: UserOauthToken,
    tick_data: &mut TickData,
) -> Result<(), Box<dyn Error>> {
    let chatter_data = ChatterData::from_channel(&reqwest_client, client_user.get_login()).await?;

    let viewers = chatter_data.get_all_viewers(true, true);

    let mut header_map = HeaderMap::new();
    let client_header = bearer_token.get_client_id().get_header()?;
    let header_name = HeaderName::from_str(client_header.key.as_str())?;
    header_map.append(
        header_name,
        HeaderValue::from_str(bearer_token.get_client_id().value.as_str())?,
    );
    let bearer_header = bearer_token.get_oauth_bearer_header();
    let header_name = HeaderName::from_str(bearer_header.key.as_str())?;
    header_map.append(header_name, bearer_header.value);

    let viewer_datas =
        UserData::get_from_usernames(reqwest_client, viewers, &DefaultLogger {}, header_map)
            .await?;

    tick_data.tick_on_users(
        client_user.get_user_id(),
        viewer_datas
            .iter()
            .map(|d| d.clone().get_user_id())
            .collect(),
    )?;
    Ok(())
}
