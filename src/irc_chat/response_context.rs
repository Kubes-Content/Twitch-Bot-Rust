use crate::user::user_data::Data as UserData;

/*macro_rules! user_command_type {
    () => { &'life dyn Fn(TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &dyn Logger) };
}

macro_rules! user_commands_map {
    () => { HashMap<String, user_command_type!() > };
}*/

pub struct ResponseContext {
    client_data: UserData,
    response_received: String,
    responses_to_send: Vec<String>,
    //user_commands:user_commands_map!()
}

//unsafe impl Send for ResponseContext {}

impl ResponseContext {
    pub fn new(
        client_data: UserData,
        response_received: String,
        //user_commands:user_commands_map!()
    ) -> ResponseContext {
        ResponseContext {
            client_data,
            response_received,
            responses_to_send: Vec::new(),
            //user_commands
        }
    }

    pub fn get_initial_response(&self) -> String {
        self.response_received.clone()
    }

    pub fn get_responses_to_reply_with(&self) -> Vec<String> {
        self.responses_to_send.clone()
    }

    pub fn add_response_to_reply_with(&mut self, reply: String) {
        self.responses_to_send.push(reply);
    }

    pub fn get_client_user_data(&self) -> UserData {
        self.client_data.clone()
    }
}
