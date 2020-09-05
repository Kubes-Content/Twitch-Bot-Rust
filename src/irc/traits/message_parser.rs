use crate::logger::Logger;
use crate::irc::response_context::ResponseContext;
use std::collections::HashMap;
use crate::irc::twitch_user_message::TwitchIrcUserMessage;


pub trait IrcMessageParser<TLogger>
    where TLogger: Logger {
    fn process_response(&self, context:&mut ResponseContext, logger:TLogger) -> bool;

    fn get_user_commands(&self) -> HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, TLogger)>;

    fn get_user_commands_including_alternates(&self) -> (HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, TLogger)>, HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, TLogger)>);
}