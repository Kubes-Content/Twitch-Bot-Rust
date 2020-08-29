extern crate futures;
extern crate tokio;

use reqwest::Client;
use std::io::{stdin, BufRead};
use crate::secrets::CLIENT_ID;
use crate::credentials::client_id::ClientId;
use std::error::Error;
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
pub mod utilities;
pub mod web_requests;

pub mod console_components;
pub mod logger;
pub mod browser;

pub mod secrets;

use user::oauth_token::OauthToken as UserOauthToken;
use crate::user::user_data::Data as UserData;
use crate::irc::chat_session::{IrcChatSession, TWITCH_IRC_URL};
use crate::logger::DefaultLogger;
use std::str::FromStr;
use crate::irc::default_message_parser::DefaultMessageParser;


static LOGGER:DefaultLogger = DefaultLogger {};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client_result = Client::builder().build();

    if client_result.is_err() {
        println!("{}", client_result.unwrap_err().to_string());

        panic!("DARF in main.rs");
    }

    let client = client_result.unwrap();
    let (token, user) = init_token_and_user(&client).await;


    let mut irc_parser = DefaultMessageParser::new(Default::default());
    let mut irc = IrcChatSession::new(user.get_login(), token, &mut irc_parser, &LOGGER, websocket::url::Url::from_str(TWITCH_IRC_URL).unwrap());
    let irc_future = irc.initialize(vec![user.get_login()]);
    irc_future.await;

    Ok(())
}


async fn init_token_and_user(client:&Client) -> (OauthToken, UserData) {
    println!("TEST AUTH");

    let b = browser::DefaultBrowser {};
    let components = console_components::ConsoleComponents::new(&LOGGER, &b);


    // get user token
    let token = UserOauthToken::request(client, ClientId::new(CLIENT_ID.to_string()), &b).await;


    // get logged in user's info
    let user_data = user::user_data::Data::get_from_bearer_token(client, token.clone(), components).await;


    (token, user_data)
}

//async fn request_user_oauth_token(client:&Client, browser: &mut dyn browser::Browser, client_id:&str) -> OauthToken { user::oauth_token::OauthToken::request(client, ClientId::new(client_id.to_string()), browser).await }

pub fn get_input_from_console(heading:&str) -> String {
    println!();
    println!("*****************************************************");
    println!("{}", heading);
    let stdin = stdin();
    let value = stdin.lock().lines().next().unwrap().unwrap();

    value
}