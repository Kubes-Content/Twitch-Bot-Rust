use super::reqwest::{Client, Response};
use super::reqwest::header::HeaderMap;
use crate::WebRequests::{request, get_reqwest_response_text, is_html};
use crate::Debug::fail_safely;


pub async fn request_data<StringReceivedFunc, HtmlReceivedFunc>(client:&Client, url_string:&str, headers:HeaderMap, on_string_received_callback:StringReceivedFunc, on_html_received_callback:HtmlReceivedFunc)
    where StringReceivedFunc: FnOnce(String),
          HtmlReceivedFunc: FnOnce(String) {

    // Get response
    let response:Response = {
        let mut out_response:Option<Response> = None;
        let on_response = | response:Response | {
            out_response = Some(response);
        };
        request(client, url_string, headers, on_response).await;
        out_response.unwrap()
    };

    let is_html = is_html(&response);

    let response_text = get_reqwest_response_text(response).await;

    // act on response
    if is_html {
        on_html_received_callback(response_text)
    } else {
        on_string_received_callback(response_text)
    }
}
