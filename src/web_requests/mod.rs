// use callbacks instead of passing in junk
extern crate reqwest;

pub mod header;
pub mod twitch;

use self::reqwest::header::{HeaderMap};
use self::reqwest::{Response, RequestBuilder, Client};
use crate::logger::Logger;


pub async fn post_request(client:&reqwest::Client, url_string:&str, headers:HeaderMap) -> Response {
   submit_request(client, client.post(url_string), headers).await
}

pub async fn request(client:&reqwest::Client, url_string:&str, headers:HeaderMap) -> Response {
   submit_request(client, client.get(url_string), headers).await
}

async fn submit_request(client:&Client, request_builder:RequestBuilder, headers:HeaderMap) -> Response {
   let request_result = request_builder.headers(headers).build();

   match request_result {
      Ok(request) => {
         match client.execute(request).await {
            Ok(response) => {
               response
            },
            Err(e) => {
               panic!("WEB RESPONSE NOT RECEIVED! Error: {}", e)
            }
         }
      },
      Err(_) => {
         panic!("WEB REQUEST FAILED!")
      }
   }
}

pub fn is_html<TLogger> (target:&Response, logger:&TLogger) -> bool
   where TLogger : Logger {
   content_type_is_x("text/html", target, logger)
}

pub fn is_json<TLogger> (target:&Response, logger:&TLogger) -> bool
   where TLogger : Logger {
   content_type_is_x("application/json", target, logger)
}

pub fn is_text<TLogger> (target:&Response, logger:&TLogger) -> bool
   where TLogger : Logger {
   content_type_is_x("text/plain", target, logger)
}

fn content_type_is_x<TLogger> (target_type:&str, target:&Response, logger:&TLogger) -> bool
   where TLogger: Logger {
   let headers = target.headers().clone();
   let content_type_header_option = headers.get("content-type");

   match content_type_header_option {

      None => {

         logger.write_line("Couldnt get content-type from web response".to_string());
         false
      },
      Some(content_type_header) => {

         match content_type_header.to_str() {

            Ok(content_type) => {

               let mut content_type = content_type.to_string();
               if content_type.len() != target_type.len() { // remove excess text (not sure why it will sometimes give everything after the retrieved header)
                  content_type = content_type[0..target_type.len()].to_string();
               }

               content_type == target_type.to_string()
            },
            Err(e) => {

               logger.write_line(format!("WARNING: Could not convert header value to string. Error: {}", e));
               false
            },
         }
      },
   }
}