extern crate futures;
extern crate tokio;

use reqwest::Client;
use std::io::{stdin, BufRead};
use crate::secrets::{CLIENT_ID, CLIENT_SECRET};
use crate::credentials::client_id::ClientId;
use crate::user::oauth_token::OauthToken;


#[macro_use]
pub mod macros {
    #[macro_export] macro_rules! primitiveWrapper {
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

pub mod irc;
pub mod credentials;
pub mod json;
pub mod debug;
pub mod user;
pub mod oauth;
pub mod save_data;
pub mod utilities;
pub mod web_requests;

pub mod logger;

pub mod secrets;

use user::oauth_token::OauthToken as UserOauthToken;
use crate::user::user_data::Data as UserData;
use crate::irc::chat_session::{IrcChatSession, TWITCH_IRC_URL};
use crate::logger::{DefaultLogger, Logger};
use std::str::FromStr;
use crate::irc::default_irc_message_parser::DefaultMessageParser;
use std::sync::{Arc, Mutex};
use crate::credentials::client_secret::ClientSecret;
use crate::irc::channel_chatter_data::ChatterData;
use crate::user::user_properties::UserLogin;
use tokio::time::{delay_for, Duration};
use std::thread::sleep;
use websocket::{ClientBuilder, OwnedMessage, WebSocketError};
use websocket::url::Url;
use websocket::futures::{IntoFuture, Async, Stream, Sink};
use std::ops::{Deref, DerefMut};
use tokio::macros::support::Pin;
use websocket::client::r#async::{Framed, TcpStream};
use websocket::header::Headers;
use websocket::websocket_base::codec::ws::MessageCodec;
use websocket::ws::dataframe::DataFrame;
use crate::oauth::token_data::TokenData;
use chrono::Local;
use crate::oauth::signature::Signature;


#[tokio::main]
async fn main() {



    // custom commands test
    /*let mut test_custom_commands:HashMap<String,String> = HashMap::new();
    test_custom_commands.insert("commandName_here".to_string(), "Command text that says stuff!!".to_string());
    test_custom_commands.insert("commandName2_here".to_string(), "Command2 text that says stuff too!!".to_string());

    let test_custom_commands_save_data = CustomCommandsSaveData::new(test_custom_commands);

    let channel = UserLogin::new("kubesvoxel".to_string());

    test_custom_commands_save_data.save(channel.clone());

    let test_custom_commands = CustomCommandsSaveData::load_or_default(channel.clone());*/




    // params
    let irc_url = {
        match websocket::url::Url::from_str(TWITCH_IRC_URL) {
            Ok(irc_url) => {
                irc_url
            }
            Err(..) => {
                panic!("Invalid IRC URL in main.rs!")
            }
        }
    };
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


    // chatter_data test
    /*let chatter_data = ChatterData::from_channel(&client, UserLogin::new("wormjuicedev".to_string())).await;
    let all_viewers = chatter_data.get_all_viewers(true, true);*/

    let (token,
        user) = init_token_and_user(&client, DefaultLogger {}).await;
    // create IRC
    let irc = IrcChatSession::new(user.clone(), token, DefaultMessageParser::new(), DefaultLogger {}, irc_url);
    //
    // run IRC
    let arc = Arc::new(Mutex::new(irc));
    IrcChatSession::initialize(arc.clone(), vec![user.get_login()]).await;
    //IrcChatSession::debug_start_async(arc.clone(), vec![user.get_login()]).await;

    loop {
        sleep(Duration::from_millis(9999));
        //delay_for(Duration::from_millis(1)).await;
    }

    panic!("FORCE PANIC ON CLOSE!!!");
}

async fn init_token_and_user<TLogger>(client:&Client, logger:TLogger) -> (OauthToken, UserData)
    where TLogger: Logger {
    println!("TEST AUTH");


    // get user token
    let token = UserOauthToken::request(client, ClientId::new(CLIENT_ID.to_string()), ClientSecret::new(CLIENT_SECRET.to_string())).await;


    // get logged in user's info
    let user_data = user::user_data::Data::get_from_bearer_token(client, token.clone(), logger).await;


    (token, user_data)
}

pub fn get_input_from_console(heading:&str) -> String {
    println!();
    println!("*****************************************************");
    println!("{}", heading);
    let stdin = stdin();
    let value = stdin.lock().lines().next().unwrap().unwrap();

    value
}


async fn concurrent_test() {

    tokio::task::spawn( async move {
        loop {
            println!("1");
            delay_for(Duration::from_millis(10)).await;
        }
    });
    tokio::task::spawn( async move {
        loop {
            println!("2");
            delay_for(Duration::from_millis(10)).await;
        }
    });

    delay_for(Duration::from_millis(10)).await;
}

// test thread and task mixing? (thread blocking)

async fn test_async_websocket(url:&Url){
    let mut client_future = ClientBuilder::from_url(url).async_connect_insecure();
        let delay = 2;

    match client_future.poll() {
        Ok( mut a ) => {
            match a {
                Async::Ready((mut c,h)) => {
                    let stream = c.poll();
                    match stream {
                        Ok(stream_async) => {
                            match stream_async {
                                Async::Ready(potential_message) => {
                                    match potential_message {
                                        None => {

                                        },
                                        Some(message) => {
                                            let message_text = String::from_utf8_lossy(&message.take_payload()).to_string();
                                            println!("")
                                        },
                                    }
                                },
                                Async::NotReady => {},
                            }
                        }
                        Err(_) => {},
                    }
                },
                Async::NotReady => {
                    delay_for(Duration::from_millis(delay)).await;
                },
            }
        },
        Err(e ) => {
            println!("ERROR: {}", e);
        },
    }


}