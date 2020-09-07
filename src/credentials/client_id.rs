use reqwest::header::HeaderValue;

use crate::web_requests::header::Header;

// use Header

#[derive(Clone)]
pub struct ClientId{
    pub value:String,
}

impl ClientId {

    pub fn new(new_id:String) -> ClientId {
        ClientId{ value : new_id }
    }

    pub fn get_header(&self) -> Header {
        const CLIENT_ID_KEY:&str = "Client-ID";
        Header::new(CLIENT_ID_KEY.to_string(), HeaderValue::from_str(self.value.as_str()).unwrap())
    }

    pub fn equals(&self, other:ClientId) -> bool { self.value == other.value }



    // get_hash_code

    pub fn to_string(&self) -> String {
        format!("(Client-ID: {0})", self.value)
    }

}

impl Eq for ClientId {
}
// == / !=
impl PartialEq for ClientId {
    fn eq(&self, other:&Self) -> bool {
        self.value == other.value
    }
}