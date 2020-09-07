use crate::user::user_properties::UserLogin;


#[derive(Clone)]
pub struct TwitchIrcUserMessage {
    speaker:UserLogin,
    text:String,
    channel:UserLogin
}

impl TwitchIrcUserMessage {
    pub fn new(speaker:UserLogin,
               text:String,
               channel:UserLogin) -> TwitchIrcUserMessage {
        TwitchIrcUserMessage { speaker, text, channel }
    }

    pub fn get_message_body(&self) -> String { self.text.clone() }

    pub fn get_target_channel(&self) -> UserLogin { self.channel.clone() }

    pub fn get_speaker(&self) -> UserLogin { self.speaker.clone() }
}