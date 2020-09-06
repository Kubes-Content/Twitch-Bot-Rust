#[derive(Copy, Clone)]
pub struct ClientSecret {
    pub value:&'static str,
}

impl ClientSecret {

    pub const fn new(new_secret:&'static str) -> ClientSecret {
        ClientSecret{ value : new_secret }
    }

    pub fn equals(&self, other:ClientSecret) -> bool {
        self.value == other.value
    }

    // get_hash_code

}

// == / !=
impl PartialEq for ClientSecret {
    fn eq(&self, other:&Self) -> bool {
        self.value == other.value
    }
}