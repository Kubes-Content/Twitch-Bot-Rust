use super::reqwest::header::HeaderValue;


pub struct Header {
    name:String,
    value:HeaderValue
}

impl Header {
    pub fn new(new_name:String, new_value:HeaderValue) -> Header {
        Header { name: new_name, value: new_value }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_value(&self) -> HeaderValue {
        self.value.clone()
    }
}