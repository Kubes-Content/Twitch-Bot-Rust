use super::reqwest::header::HeaderMap;
use super::reqwest::{Client, Response};
use crate::web_requests::WEB_REQUEST_ATTEMPTS;
use crate::Logger;
use kubes_web_lib::web_request::{is_html, is_json, request, RequestType};
use url::Url;

pub enum TwitchRequestResponse {
    Other { response_text: String },
    Html { response_text: String },
    Json { response_text: String },
}

pub async fn request_data<TLogger: Logger>(
    client: &Client,
    url_string: &str,
    headers: HeaderMap,
    logger: &TLogger,
) -> TwitchRequestResponse {
    // Get response
    let response: Response = request(
        client,
        Url::parse(url_string).unwrap(),
        RequestType::Get,
        headers,
        WEB_REQUEST_ATTEMPTS,
    )
    .await
    .unwrap();

    let is_html = is_html(&response, logger).unwrap();
    let is_json = is_json(&response, logger).unwrap();

    let response_text = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            panic!("Could not get response from request! ERROR: {}", e)
        }
    };

    // act on response
    if is_html {
        return TwitchRequestResponse::Html { response_text };
    }

    if is_json {
        return TwitchRequestResponse::Json { response_text };
    }

    TwitchRequestResponse::Other { response_text }
}
