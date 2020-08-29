use crate::oauth::token_data::TokenData;
use crate::credentials::client_id::ClientId;
use crate::oauth::has_oauth_signature::HasOauthSignature;
use crate::oauth::validation_token::ValidationToken;
use crate::oauth::signature::Signature;
use crate::credentials::bot_user_credentials::REDIRECT_URI;
use crate::credentials::access_scopes::get_all_scopes;
use crate::web_requests::header::Header as WebRequestHeader;
use reqwest::{Client, Response};
use crate::console_components::ConsoleComponents;
use crate::web_requests::twitch::request_data;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::debug::fail_safely;
use crate::json::crawler::crawl_json;
use crate::browser::Browser;
use crate::web_requests::{request, is_html};
use crate::get_input_from_console;

primitiveWrapper!(OauthToken, TokenData, "{}"); // may need to implement PartialEq on TokenData

impl HasOauthSignature for OauthToken {
    fn get_oauth_signature(&self) -> Signature {
        self.value.get_signature()
    }

    fn update_oauth(&mut self, validation_token: ValidationToken) {
        self.value.update(validation_token);
    }
}

impl OauthToken {
    pub async fn request(web_client:&Client, client_id:ClientId, _browser:&dyn Browser) -> OauthToken {
        let url_implicit_flow = format!("https://id.twitch.tv/oauth2/authorize?client_id={0}&redirect_uri={1}&response_type=token&scope={2}", client_id.value, REDIRECT_URI, get_all_scopes());

        println!("{}", url_implicit_flow);

        let headers = HeaderMap::new();

        let received_response = {
            let mut out_response:Option<Response> = None;
            let set_response = |response:Response| { out_response = Some(response); };
            request(web_client, url_implicit_flow.as_str(), headers, set_response).await;
            out_response.unwrap()
        };

        //let mut token = String::new();

        if ! is_html(&received_response) {

            println!("{}", received_response.text().await.unwrap());

            fail_safely("HTML EXPECTED");
        } else {

            open::that(received_response.url().as_str()).unwrap();

            /*// listen for oauth redirect
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 4096];
                stream.read(&mut buffer).unwrap();
                stream.
                let received_request = String::from_utf8_lossy(&buffer[..]).to_string();
                let get_token = |string:String| {
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

                    if uri.len() == 30 {
                        Ok(uri)
                    } else {
                        Err(ErrorKind::InvalidData)
                    }
                };

                let tmp = get_token(received_request);

                if ! tmp.is_err() {
                    token = tmp.unwrap();
                    break;
                }
            }*/

            // begin listening server // make sure redirect URI is localhost and port
        }

        let token = get_input_from_console("Upon authorizing your account, please post the URL of the page you are redirected to into the console to finalize authorization.");

        OauthToken::new(TokenData::from_str(token.as_str(), client_id.value))
    }

    pub fn get_client_id(&self) -> ClientId {
        self.value.get_client_id()
    }

    pub fn get_token_data_copy(&self) -> TokenData {
        self.value.clone()
    }

    pub fn get_oauth_oauth_header(&self) -> WebRequestHeader { self.get_oauth_signature().get_oauth_oauth_header() }

    pub fn get_oauth_bearer_header(&self) -> WebRequestHeader { self.get_oauth_signature().get_oauth_bearer_header() }

    pub async fn validate<'life>(&mut self, client:&Client, _components:ConsoleComponents<'life>) {
        const VALIDATION_URL:&str = "https://id.twitch.tv/oauth2/validate";
        const AUTHORIZATION_HEADER:&str = "Authorization";

        let mut header_map = HeaderMap::new();
        header_map.append(AUTHORIZATION_HEADER, HeaderValue::from_str(format!("OAuth {}", self.get_oauth_signature().to_string()).as_str()).unwrap());

        let on_string_received = |string:String| {

            let validation_token = ValidationToken::from_json(crawl_json(string.as_str()));

            self.update_oauth(validation_token);

            println!("User Oauth token updated via validation token!");
        };
        let on_html_received = |_:String| { fail_safely("Expecting JSON!") };
        request_data(client,
                     VALIDATION_URL,
                     header_map,
                     on_string_received,
                     on_html_received).await;
    }
}