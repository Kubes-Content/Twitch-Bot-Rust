use std::collections::HashMap;

use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::traits::message_parser::MessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;


pub trait IrcMessageParser<TLogger> : MessageParser<TLogger>
    where TLogger: Logger {
    fn get_user_commands(&self) -> HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger)>;

    fn get_user_commands_including_alternates(&self) -> (HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger)>, HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger)>);
}