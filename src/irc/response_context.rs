use crate::user::user_properties::UserLogin;


pub struct ResponseContext {
    client_data:UserLogin,
    response_received:String,
    responses_to_send:Vec<String>
}

impl ResponseContext {
    pub fn new(client_data:UserLogin,
               response_received:String) -> ResponseContext {
        ResponseContext {
            client_data,
            response_received,
            responses_to_send: Vec::new()
        }
    }

    pub fn get_initial_response(&self) -> String {
        self.response_received.clone()
    }

    pub fn get_responses_to_reply_with(&self) -> Vec<String> {
        self.responses_to_send.clone()
    }

    pub fn add_response_to_reply_with(&mut self, reply:String) {
        self.responses_to_send.push(reply);
    }

    pub fn get_client_user(&self) -> UserLogin { self.client_data.clone() }
}