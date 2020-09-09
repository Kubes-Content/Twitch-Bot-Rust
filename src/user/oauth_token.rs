use std::io::{ErrorKind, Read};
use std::net::TcpListener;
use std::ops::Add;

use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::credentials::access_scopes::get_all_scopes;
use crate::credentials::bot_user_credentials::REDIRECT_URI;
use crate::credentials::client_id::ClientId;
use crate::credentials::client_secret::ClientSecret;
use crate::json::crawler::crawl_json;
use crate::logger::Logger;
use crate::oauth::has_oauth_signature::HasOauthSignature;
use crate::oauth::signature::Signature;
use crate::oauth::token_data::TokenData;
use crate::oauth::validation_token::ValidationToken;
use crate::web_requests::{is_html, is_json, post_request, request};
use crate::web_requests::header::Header as WebRequestHeader;
use crate::web_requests::twitch::{request_data, TwitchRequestResponse};

primitive_wrapper!(OauthToken, TokenData, "{}");

impl Default for OauthToken {
    fn default() -> Self {
        OauthToken::new(TokenData::from_str("", "".to_string()))
    }
}


impl HasOauthSignature for OauthToken {
    fn get_oauth_signature(&self) -> Signature {
        self.value.get_signature()
    }

    fn update_oauth(&mut self, validation_token: ValidationToken) {
        self.value.update(validation_token);
    }
}


impl OauthToken {
    pub async fn request<TLogger>(web_client: &Client, client_id: ClientId, client_secret:ClientSecret, logger:&TLogger) -> OauthToken
        where TLogger: Logger {
        let url_authoritive_flow = format!("https://id.twitch.tv/oauth2/authorize?client_id={0}&redirect_uri={1}&response_type=code&scope={2}", client_id.value, REDIRECT_URI, get_all_scopes());

        println!("{}", url_authoritive_flow);

        let headers = HeaderMap::new();

        let authorization_response = request(web_client, url_authoritive_flow.as_str(), headers.clone()).await;

        let authorization = {
            let mut authorization = String::new();
            if !is_html(&authorization_response, logger) {
                println!("{}", authorization_response.text().await.unwrap());

                panic!("HTML EXPECTED");
            } else {
                open::that(authorization_response.url().as_str()).unwrap();

                // listen for oauth redirect
                let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
                for stream in listener.incoming() {
                    let mut stream = stream.unwrap();
                    let mut buffer = [0; 4096];
                    stream.read(&mut buffer).unwrap();
                    let received_request = String::from_utf8_lossy(&buffer[..]).to_string();
                    let get_authorization_code = |string: String| {
                        println!("Request: {}", string);

                        let mut chars = string.chars();

                        while let Some(character) = chars.next() {
                            if character == '/' {
                                break;
                            }
                        }

                        let mut uri = String::new();

                        while let Some(character) = chars.next() {
                            if character == '&' || character == ' ' {
                                break;
                            }
                            uri = uri.add(character.to_string().as_str());
                        }

                        if uri.len() == 36 && uri[0..6].to_string() == "?code=" {
                            Ok(uri[6..].to_string())
                        } else {
                            Err(ErrorKind::InvalidData)
                        }
                    };

                    match get_authorization_code(received_request) {
                        Ok(authorization_temp) => {
                            authorization = authorization_temp;
                            break;
                        }
                        Err(_) => {},
                    }
                }
            }
            authorization
        };

        let url_request_access_token = format!("https://id.twitch.tv/oauth2/token?client_id={0}&client_secret={1}&code={2}&grant_type=authorization_code&redirect_uri={3}", client_id.value, client_secret.value, authorization, REDIRECT_URI);

        let token_response = post_request(web_client, url_request_access_token.as_str() , headers.clone()).await;

        let token = {

            if ! is_json(&token_response, logger) {
                panic!("EXPECTED JSON");
            }

            TokenData::from_json(crawl_json(token_response.text().await.unwrap().as_str()), client_id)
        };


        //let token = get_input_from_console("Upon authorizing your account, please post the URL of the page you are redirected to into the console to finalize authorization.");


        OauthToken::new(token)
//        OauthToken::new(TokenData::from_str(authorization.as_str(), client_id.value))
    }

    pub fn get_client_id(&self) -> ClientId {
        self.value.get_client_id()
    }

    pub fn get_token_data_copy(&self) -> TokenData {
        self.value.clone()
    }

    pub fn get_oauth_oauth_header(&self) -> WebRequestHeader { self.get_oauth_signature().get_oauth_oauth_header() }

    pub fn get_oauth_bearer_header(&self) -> WebRequestHeader { self.get_oauth_signature().get_oauth_bearer_header() }

    pub async fn validate<TLogger>(&mut self, client: &Client, logger:&TLogger)
        where TLogger: Logger {
        const VALIDATION_URL: &str = "https://id.twitch.tv/oauth2/validate";
        const AUTHORIZATION_HEADER: &str = "Authorization";

        let mut header_map = HeaderMap::new();
        header_map.append(AUTHORIZATION_HEADER, HeaderValue::from_str(format!("OAuth {}", self.get_oauth_signature().to_string()).as_str()).unwrap());

        match request_data(client,
                     VALIDATION_URL,
                     header_map,
                     logger).await {
            TwitchRequestResponse::Json { response_text } => {
                let validation_token = ValidationToken::from_json(crawl_json(response_text.as_str()));

                self.update_oauth(validation_token);

                logger.write_line("User Oauth token updated via validation token!".to_string());
            }
            _ => {
                panic!("Expecting JSON!");
            },
        };
    }
}