use std::{
    error::Error,
    io::{ErrorKind, Read},
    net::TcpListener,
    ops::Add,
};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

use crate::{
    credentials::{
        access_scopes::get_all_scopes, bot_user_credentials::REDIRECT_URI, client_id::ClientId,
        client_secret::ClientSecret,
    },
    oauth::{
        has_oauth_signature::HasOauthSignature, token_data::TokenData,
        validation_token::ValidationToken,
    },
    web_requests::{
        twitch::{request_data, TwitchRequestResponse},
        Header as WebRequestHeader, WEB_REQUEST_ATTEMPTS,
    },
};
use kubes_std_lib::logging::Logger;
use kubes_web_lib::{
    error::send_error,
    json::crawler::crawl_json,
    oauth::Signature,
    web_request::{is_html, is_json, request, RequestType},
};
use url::Url;

primitive_wrapper!(OauthToken, TokenData, "{}");

impl Default for OauthToken {
    fn default() -> Self {
        OauthToken::from(TokenData::from_str("", "".to_string()))
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
    pub async fn request<TLogger: Logger>(
        web_client: &Client,
        client_id: ClientId,
        client_secret: ClientSecret,
        logger: &TLogger,
    ) -> Result<OauthToken, Box<dyn Error>> {
        let url_authoritive_flow = format!("https://id.twitch.tv/oauth2/authorize?client_id={0}&redirect_uri={1}&response_type=code&scope={2}", client_id.value, REDIRECT_URI, get_all_scopes());

        println!("{}", url_authoritive_flow);

        let headers = HeaderMap::new();

        let url = Url::parse(url_authoritive_flow.as_str())?;

        let authorization_response = request(
            web_client,
            url,
            RequestType::Get,
            headers.clone(),
            WEB_REQUEST_ATTEMPTS,
        )
        .await?;

        let authorization = {
            let mut authorization = String::new();
            if !is_html(&authorization_response, logger)? {
                println!("{}", authorization_response.text().await?);

                panic!("HTML EXPECTED");
            } else {
                open::that(authorization_response.url().as_str())?;

                // listen for oauth redirect
                let listener = TcpListener::bind("127.0.0.1:7878")?;
                for stream in listener.incoming() {
                    let mut stream = stream?;
                    let mut buffer = [0; 4096];
                    stream.read(&mut buffer)?;
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
                        Err(_) => {}
                    }
                }
            }
            authorization
        };

        let url_request_access_token = format!("https://id.twitch.tv/oauth2/token?client_id={0}&client_secret={1}&code={2}&grant_type=authorization_code&redirect_uri={3}", client_id.value, client_secret.value, authorization, REDIRECT_URI);

        let token_response = request(
            web_client,
            Url::parse(url_request_access_token.as_str())?,
            RequestType::Post {
                body: String::from(""),
            },
            headers.clone(),
            WEB_REQUEST_ATTEMPTS,
        )
        .await
        .unwrap();

        let token = {
            if !is_json(&token_response, logger)? {
                panic!("EXPECTED JSON!");
            }

            match crawl_json(token_response.text().await?.as_str()) {
                Ok(token_json) => TokenData::from_json(token_json, client_id),
                Err(e) => {
                    panic!("Failed to retrieve oauth token. Error: {}", e)
                }
            }
        };

        Ok(OauthToken::from(token))
    }

    pub fn get_client_id(&self) -> ClientId {
        self.value.get_client_id()
    }

    pub fn get_token_data_copy(&self) -> TokenData {
        self.value.clone()
    }

    pub fn get_oauth_oauth_header(&self) -> WebRequestHeader {
        self.get_oauth_signature().get_oauth_oauth_header()
    }

    pub fn get_oauth_bearer_header(&self) -> WebRequestHeader {
        self.get_oauth_signature().get_oauth_bearer_header()
    }

    pub async fn validate<TLogger>(&mut self, client: &Client, logger: &TLogger)
    where
        TLogger: Logger,
    {
        const VALIDATION_URL: &str = "https://id.twitch.tv/oauth2/validate";
        const AUTHORIZATION_HEADER: &str = "Authorization";

        let mut header_map = HeaderMap::new();
        header_map.append(
            AUTHORIZATION_HEADER,
            HeaderValue::from_str(
                format!("OAuth {}", self.get_oauth_signature().to_string()).as_str(),
            )
            .unwrap(),
        );

        match request_data(client, VALIDATION_URL, header_map, logger).await {
            TwitchRequestResponse::Json { response_text } => {
                let validation_token = match crawl_json(response_text.as_str()) {
                    Ok(j) => ValidationToken::from_json(j),
                    Err(_) => panic!("Failed to parse validation token!"),
                };

                self.update_oauth(validation_token);

                logger.write_line("User Oauth token updated via validation token!".to_string());
            }
            _ => {
                panic!("Expecting JSON!");
            }
        };
    }
}
