use crate::irc::response_context::ResponseContext;
use crate::logger::Logger;


pub trait MessageParser<TLogger>: Sync + 'static
    where TLogger: Logger{
    fn process_response(&self, context:&mut ResponseContext, logger:&TLogger) -> bool;
}