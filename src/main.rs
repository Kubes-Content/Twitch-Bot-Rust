extern crate async_trait;
extern crate futures;
extern crate tokio;

use credentials::client_id::ClientId;
use irc_chat::{channel_chatter_data::ChatterData,
               parsers::{default_irc_message_parser::DefaultMessageParser,
                         pubsub::default_message_parser::DefaultPubSubParser},
               web_socket_session::WebSocketSession};
use logger::{DefaultLogger, Logger};
use oauth::has_oauth_signature::HasOauthSignature;
use reqwest::Client;
use secrets::{CLIENT_ID, CLIENT_SECRET};
use std::{convert::TryFrom,
          str::FromStr,
          sync::{Arc,
                 Mutex},
          thread::sleep,
          time::Instant};
use tokio::time::Duration;
use user::{oauth_token::OauthToken as UserOauthToken,
           user_data::Data as UserData,
           user_properties::UserLogin};
use websocket::{stream::sync::{TcpStream,
                               TlsStream},
                url::Url};


#[macro_use]
pub mod macros {
    #[macro_export] macro_rules! primitive_wrapper {
        ($type_name:ident, $wrapped_type:ty, $string_format:expr) => {
            #[derive(Clone)]
            pub struct $type_name {
                value:$wrapped_type
            }

            impl ToString for $type_name {
                fn to_string(&self) -> String {
                    format!($string_format, self.value.to_string())
                }
            }

            impl PartialEq for $type_name {
                fn eq(&self, other:&$type_name) -> bool {
                    self.value == other.value
                }
            }

            impl $type_name {
                pub fn new(new_value:$wrapped_type) -> $type_name {
                    $type_name { value : new_value }
                }

                pub fn get_value(&self) -> $wrapped_type {
                    self.value.clone()
                }
            }
        };
    }
}

pub mod credentials;
pub mod debug;
pub mod irc_chat;
pub mod json;
pub mod logger;
pub mod oauth;
pub mod save_data;
pub mod secrets;
pub mod user;
pub mod utilities;
pub mod web_requests;

const TICK_RATE:u64 = 1000;

#[tokio::main]
async fn main() {

    let (token,
        user) = init_token_and_user(&DefaultLogger {}).await;

    start_chat_session(token.clone(), user.clone()).await;

    start_pubsub_session(token.clone(), user.clone()).await;

    tick(token, user, TICK_RATE).await;
}


async fn start_pubsub_session(token: UserOauthToken, user: UserData) {
    let pubsub_url = Url::from_str("wss://pubsub-edge.twitch.tv").unwrap();
// create PubSub-WebSocket
    let pubsub_session = WebSocketSession::new(user.clone(), token.clone(), DefaultPubSubParser::new(), DefaultLogger {}, pubsub_url);
    let pubsub_arc = Arc::new(tokio::sync::Mutex::new(pubsub_session));

    let temp_channel_id = user.clone().get_user_id().get_value();
    let temp_oauth = token.clone().get_oauth_signature().get_value();
    //
    let on_pubsub_start = move |session: &mut WebSocketSession<DefaultPubSubParser, DefaultLogger>, listener: &mut websocket::sync::Client<TlsStream<TcpStream>>| {
        session.send_string(listener, format!("{{\
        \"type\": \"LISTEN\",\
        \"nonce\": \"333\",\
        \"data\": {{\
        \"topics\": [
        \"channel-points-channel-v1.{0}\"
        ],\
        \"auth_token\": \"{1}\"\
        }}
        }}", temp_channel_id, temp_oauth));
    };
    //
    WebSocketSession::initialize(pubsub_arc, on_pubsub_start).await;
}


async fn start_chat_session(token: UserOauthToken, user: UserData) {

    let chat_url = websocket::url::Url::from_str("wss://irc-ws.chat.twitch.tv:443").unwrap();
// create Chat-IRC
    let chat_session =
        WebSocketSession::new(user.clone(), token.clone(), DefaultMessageParser::new(), DefaultLogger {}, chat_url);
//
// run Chat-IRC
    let chat_irc_arc = Arc::new(tokio::sync::Mutex::new(chat_session));
    let token_temp = token.clone();
    let user_temp = user.clone();
    let on_chat_start = move |session: &mut WebSocketSession<DefaultMessageParser<DefaultLogger>, DefaultLogger>, listener: &mut websocket::sync::Client<TlsStream<TcpStream>>| {

        // authenticate user (login)
        session.send_string(listener, format!("PASS oauth:{}", token_temp.clone().get_oauth_signature().get_value()));
        session.send_string(listener, format!("NICK {}", user_temp.clone().get_login().get_value()));
        session.send_string(listener, format!("JOIN #{}", user_temp.get_login().get_value()));
    };
    WebSocketSession::initialize(chat_irc_arc.clone(), on_chat_start).await;
}


async fn init_token_and_user<TLogger>(logger:&TLogger) -> (UserOauthToken, UserData)
    where TLogger: Logger {


    let client = {
        match Client::builder().build() {
            Ok(client) => {
                client
            }
            Err(..) => {
                panic!("Error creating client in main.rs!")
            }
        }
    };

    // get user token
    let token = UserOauthToken::request(&client, ClientId::new(CLIENT_ID.to_string()), CLIENT_SECRET, logger).await;


    // get logged in user's info
    let user_data = user::user_data::Data::get_from_bearer_token(&client, token.clone(), logger).await;


    (token, user_data)
}


async fn tick(_token: UserOauthToken, user: UserData, tick_rate:u64) {

    let reqwest_client = reqwest::Client::builder().build().unwrap();
    let client_user = user.get_login();
    sleep(Duration::from_millis(1000));
    loop {
        let before_tick_instant = Instant::now();

        tick_routine(&reqwest_client, client_user.clone()).await;

        let tick_elapsed_time = {
            match u64::try_from(before_tick_instant.elapsed().as_millis()) {
                Ok(new64) => { if new64 >= tick_rate { tick_rate - 1 } else { new64 } },
                Err(_) => { tick_rate - 1 },
            }
        };

        sleep(Duration::from_millis(tick_rate - tick_elapsed_time));
    }
}
//
async fn tick_routine(reqwest_client:&reqwest::Client, client_user:UserLogin) {

}

/*pub fn get_input_from_console(heading:&str) -> String {
    println!();
    println!("*****************************************************");
    println!("{}", heading);
    let stdin = stdin();
    let value = stdin.lock().lines().next().unwrap().unwrap();

    value
}*/