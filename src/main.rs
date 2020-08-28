extern crate futures;
extern crate tokio;

use reqwest::Client;
use std::io::{stdin, BufRead};
use crate::Secrets::CLIENT_ID;
use crate::Credentials::ClientId::ClientId;
use futures::executor::block_on;
use std::error::Error;
use crate::User::OAuthToken::OauthToken;


#[macro_use]
pub mod Macros {
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

pub mod IRC;
pub mod Credentials;
pub mod JSON;
pub mod Debug;
pub mod User;
pub mod OAuth;
pub mod Utilities;
pub mod WebRequests;

pub mod ConsoleComponents;
pub mod Logger;
pub mod Browser;

pub mod Secrets;

use User::OAuthToken::OauthToken as UserOauthToken;
use crate::User::UserData::Data as UserData;
use crate::IRC::ChatSession::{IrcChatSession, TWITCH_IRC_URL};
use crate::User::UserProperties::UserLogin;
use crate::Logger::DefaultLogger;
use std::str::FromStr;
use futures::Future;
use crate::IRC::MessageParser::{IrcMessageParser, DefaultMessageParser};
use crate::OAuth::TokenData::TokenData;


static LOGGER:DefaultLogger = DefaultLogger {};


#[tokio::main]
async fn main() -> Result<(), Box<Error>> {

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

    let mut b = Browser::DefaultBrowser {};
    let components = ConsoleComponents::ConsoleComponents::new(&LOGGER, &b);


    // get user token
    let token = UserOauthToken::request(client, ClientId::new(CLIENT_ID.to_string()), &b).await;


    // get logged in user's info
    let userData = User::UserData::Data::get_from_bearer_token(client, token.clone(), components).await;


    (token, userData)
}

async fn request_user_oauth_token(client:&Client, browser: &mut dyn Browser::Browser, client_id:&str) -> OauthToken {
    User::OAuthToken::OauthToken::request(client, ClientId::new(client_id.to_string()), browser).await
}

pub fn get_input_from_console(heading:&str) -> String {
    println!();
    println!("*****************************************************");
    println!("{}", heading);
    let stdin = stdin();
    let value = stdin.lock().lines().next().unwrap().unwrap();

    value
}