use reqwest::header::HeaderValue;

use crate::web_requests::Header;
use std::error::Error;

// use Header

#[derive(Clone, Hash)]
pub struct ClientId {
    pub value: String,
}

impl ClientId {
    pub fn new(new_id: String) -> ClientId {
        ClientId { value: new_id }
    }

    pub fn get_header(&self) -> Result<Header, Box<dyn Error>> {
        const CLIENT_ID_KEY: &str = "Client-ID";
        Ok(Header::new(
            CLIENT_ID_KEY.to_string(),
            HeaderValue::from_str(self.value.as_str())?,
        ))
    }

    // get_hash_code

    pub fn to_string(&self) -> String {
        format!("(Client-ID: {0})", self.value)
    }
}

impl Eq for ClientId {}
// == / !=
impl PartialEq for ClientId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
