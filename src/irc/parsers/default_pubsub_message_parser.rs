use crate::logger::Logger;
use std::collections::HashMap;
use crate::irc::response_context::ResponseContext;
use crate::irc::twitch_user_message::TwitchIrcUserMessage;
use crate::irc::traits::message_parser::MessageParser;


pub struct DefaultPubSubParser<TLogger: Logger + Clone>
{
    //_commands: HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, TLogger)>
}

impl<TLogger: Clone + Logger> MessageParser<TLogger> for DefaultPubSubParser<TLogger>
{
    fn process_response(&self, _context: &mut ResponseContext, _logger: &TLogger) -> bool {
        unimplemented!()
    }
}

impl<TLogger: Logger + Clone> DefaultPubSubParser<TLogger>
{
    pub fn new() -> DefaultPubSubParser<TLogger> {
        DefaultPubSubParser { }//_commands: Default::default() }
    }


    // init commands fn
}