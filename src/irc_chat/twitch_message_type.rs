use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::user::user_properties::UserLogin;

pub enum TwitchIrcMessageType {
    Client,
    Message(TwitchIrcUserMessage),
    JoiningChannel {
        joiner: UserLogin,
        channel: UserLogin,
    },
    LeavingChannel {
        leaver: UserLogin,
        channel: UserLogin,
    },
}

unsafe impl Send for TwitchIrcMessageType {}
