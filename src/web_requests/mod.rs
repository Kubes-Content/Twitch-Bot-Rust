// use callbacks instead of passing in junk
extern crate reqwest;

pub mod header;
pub mod twitch;

use crate::debug::fail_safely;
use self::reqwest::header::HeaderMap;
use self::reqwest::{Response, Error};
use std::ops::{Deref, DerefMut};
use futures::Future;


pub async fn post_request<Callback>(client:&reqwest::Client, url_string:&str, headers:HeaderMap, on_response_received_callback:Callback)
   where Callback: FnOnce(Response) {
   let request_builder = client.post(url_string);

   let request_result = request_builder.headers(headers.clone()).build();
   if request_result.is_err() { fail_safely("WEB REQUEST FAILED!"); }


   let request = request_result.unwrap();

   let response_result = client.execute(request).await;
   if response_result.is_err() { println!("WEB RESPONSE NOT RECEIVED!"); }

   on_response_received_callback(response_result.unwrap());
}

pub async fn request<Callback>(client:&reqwest::Client, url_string:&str, headers:HeaderMap, on_response_received_callback:Callback)
   where Callback: FnOnce(Response)
{
   let request_builder = client.get(url_string);

   let request_result = request_builder.headers(headers).build();
   if request_result.is_err() { fail_safely("WEB REQUEST FAILED!"); }


   let request = request_result.unwrap();

   let response_result = client.execute(request).await;

   match response_result {
      Ok(response) => {
         on_response_received_callback(response);
      },
      Err(e) => {
         fail_safely(format!("WEB RESPONSE NOT RECEIVED! Error: {}", e).as_str());
      }
   }

}

pub async fn get_reqwest_response_text(response:Response) -> String {
   let response_text_result = response.text().await;
   if response_text_result.is_err() {
      fail_safely(format!("Received error from web response: {}", (response_text_result.unwrap_err().to_string())).as_str());
      String::new()
   } else {
      response_text_result.unwrap()
   }
}

pub fn is_html (target:&Response) -> bool {
   let headers = target.headers().clone();
   let x = headers.get("content-type");
   if x == None {
      println!("Couldnt get content-type from web response")
   }else {
      println!("{}", x.unwrap().to_str().unwrap())
   }
   (x.unwrap().to_str().unwrap() == "text/html") == true
}

pub fn is_json (target:&Response) -> bool {
   let headers = target.headers().clone();
   let x = headers.get("content-type");
   if x == None {
      println!("Couldnt get content-type from web response")
   }else {
      println!("{}", x.unwrap().to_str().unwrap())
   }
   (x.unwrap().to_str().unwrap() == "application/json") == true
}

pub fn is_text (target:&Response) -> bool {
   let headers = target.headers().clone();
   let x = headers.get("content-type");
   if x == None {
      println!("Couldnt get content-type from web response")
   }else {
      println!("{}", x.unwrap().to_str().unwrap())
   }
   (x.unwrap().to_str().unwrap() == "text/plain") == true
}
