extern crate async_trait;
#[macro_use]
extern crate colour;
#[macro_use]
extern crate kubes_std_lib;
extern crate tokio;

use crate::{main_tick_data::TickData, user::user_properties::ChannelId};
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
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, error::Error, str::FromStr, sync::Arc, time::Instant};
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
pub mod user;
pub mod web_requests;

const TICK_RATE: u64 = 1000;

fn get_pubsub_secure_url() -> Url {
    Url::from_str("wss://pubsub-edge.twitch.tv").unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (token, user) = init_token_and_user(&DefaultLogger {}).await?;

    start_chat_session(token.clone(), user.clone()).await?;

    start_pubsub_session(token.clone(), user.clone()).await?;

    tick(token, user, TICK_RATE).await
}

#[derive(Deserialize, Serialize)]
struct ClientToServerRequest {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<String>,
    data: ClientToServerRequestData,
}

#[derive(Deserialize, Serialize)]
struct ClientToServerRequestData {
    topics: Vec<String>,
    auth_token: String,
}

/// Interacts with Twitch Events such as subs and channel points spending
async fn start_pubsub_session(token: UserOauthToken, user: UserData) -> Result<(), Box<dyn Error>> {
    // create PubSub-WebSocket
    let pubsub_arc = {
        let pubsub_session = WebSocketSession::new(
            DefaultPubSubParser::new(ChannelId::from(user.get_user_id()), token.clone()),
            get_pubsub_secure_url(),
        );
        Arc::new(tokio::sync::Mutex::new(pubsub_session))
    };

    let start_request = serde_json::to_string(&ClientToServerRequest {
        r#type: "LISTEN".to_string(),
        nonce: Some("333".to_string()),
        data: ClientToServerRequestData {
            topics: vec![format!(
                "channel-points-channel-v1.{}",
                user.clone().get_user_id().get_value()
            )],
            auth_token: token.clone().get_oauth_signature().get_value(),
        },
    })?;

    //
    let on_pubsub_start =
        move |session: &mut WebSocketSession<DefaultPubSubParser>,
              listener: &mut websocket::sync::Client<TlsStream<TcpStream>>| {
            session.send_string(listener, start_request);
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
        DefaultMessageParser::new(user.clone(), token.clone()),
        chat_url,
    );
    //
    // run Chat-IRC
    let chat_irc_arc = Arc::new(tokio::sync::Mutex::new(chat_session));
    let token_temp = token.clone();
    let user_temp = user.clone();
    let on_chat_start =
        move |session: &mut WebSocketSession<DefaultMessageParser>,
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
    WebSocketSession::initialize(chat_irc_arc, on_chat_start).await;
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
    let token = UserOauthToken::request(&client, CLIENT_ID.clone(), CLIENT_SECRET, logger).await?;

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
    // Headers
    let mut header_map = HeaderMap::new();
    // Client Header
    let client_header = bearer_token.get_client_id().get_header()?;
    let header_name = HeaderName::from_str(client_header.key.as_str())?;
    header_map.append(
        header_name,
        HeaderValue::from_str(bearer_token.get_client_id().value.as_str())?,
    );
    // Bearer Header
    let bearer_header = bearer_token.get_oauth_bearer_header();
    let header_name = HeaderName::from_str(bearer_header.key.as_str())?;
    header_map.append(header_name, bearer_header.value);

    //
    let chatter_data = ChatterData::from_channel(&reqwest_client, client_user.get_login()).await?;
    let viewers = chatter_data.get_all_viewers(true, true);
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
