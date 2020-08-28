use crate::WebRequests::header::Header as WebRequestHeader;
use reqwest::header::HeaderValue;

primitiveWrapper!(Signature, String, "{}");

const AUTHORIZATION_HEADER_NAME:&str = "Authorization";

impl Signature {

    pub fn get_oauth_oauth_header(&self) -> WebRequestHeader {
        WebRequestHeader::new(AUTHORIZATION_HEADER_NAME.to_string(), HeaderValue::from_str(format!("OAuth {}", self.to_string()).as_str()).unwrap())
    }

    pub fn get_oauth_bearer_header(&self) -> WebRequestHeader {
        WebRequestHeader::new(AUTHORIZATION_HEADER_NAME.to_string(), HeaderValue::from_str(format!("Bearer {}", self.to_string()).as_str()).unwrap())
    }
}