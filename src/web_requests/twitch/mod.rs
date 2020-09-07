use crate::logger::Logger;
use crate::web_requests::{is_html, is_json, request};

use super::reqwest::{Client, Response};
use super::reqwest::header::HeaderMap;


pub enum TwitchRequestResponse {
    Other { response_text:String },
    Html { response_text:String },
    Json { response_text:String }
}


pub async fn request_data<TLogger>(client:&Client, url_string:&str, headers:HeaderMap, logger:&TLogger) -> TwitchRequestResponse
    where TLogger:Logger {

    // Get response
    let response:Response = request(client, url_string, headers).await;

    let is_html = is_html(&response, logger);
    let is_json = is_json(&response, logger);

    let response_text = match response.text().await {
        Ok(text) => { text },
        Err(e) => { panic!("Could not get response from request! ERROR: {}", e) },
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
