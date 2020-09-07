// use callbacks instead of passing in junk
extern crate reqwest;

use crate::logger::Logger;

use self::reqwest::{Client, Error, RequestBuilder, Response};
use self::reqwest::header::HeaderMap;


pub mod header;
pub mod twitch;


pub async fn post_request(client:&reqwest::Client, url_string:&str, headers:HeaderMap) -> Response {
   for _attempt in 1..4 {
      match submit_request_builder(client, client.post(url_string.clone()), headers.clone(), 3).await {
         Ok(response) => {
            return response;
         },
         Err(e) => {
            println!("Post-request error: {}", e)
         },
      }
   }
   panic!("POST-REQUEST FAILED AFTER 3 ATTEMPTS")
}

pub async fn request(client:&reqwest::Client, url_string:&str, headers:HeaderMap) -> Response {
   for _attempt in 1..4 {
      match submit_request_builder(client, client.get(url_string.clone()), headers.clone(), 3).await {
         Ok(response) => {
            return response;
         },
         Err(e) => {
            println!("Get-request error: {}", e)

         },
      }
   }
   panic!("GET-REQUEST FAILED AFTER 3 ATTEMPTS")
}

async fn submit_request_builder(client:&Client, request_builder:RequestBuilder, headers:HeaderMap, attempts:u8) -> Result<Response,Error> {
   submit_request(client, request_builder.headers(headers), attempts).await
}

async fn submit_request(client:&Client, request_builder:RequestBuilder, attempts:u8) -> Result<Response,Error> {
   match request_builder.build() {
      Ok(request) => {
         client.execute(request).await
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