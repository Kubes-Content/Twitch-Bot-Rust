use crate::web_requests::Header;
use kubes_web_lib::web_request::new_header;
use std::error::Error;

// use Header

#[derive(Clone, Hash)]
pub struct ClientId {
    pub value: String,
}

impl ClientId {
    const HEADER_KEY: &'static str = "Client-ID";

    pub const fn new(new_id: String) -> ClientId {
        ClientId { value: new_id }
    }

    pub fn get_header(&self) -> Result<Header, Box<dyn Error>> {
        Ok(new_header(ClientId::HEADER_KEY, self.value.clone())?)
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
