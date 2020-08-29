use crate::logger::Logger;
use crate::irc::response_context::ResponseContext;


pub trait IrcMessageParser {
    fn process_response(&mut self, context:&mut ResponseContext, logger:&dyn Logger) -> bool;
}