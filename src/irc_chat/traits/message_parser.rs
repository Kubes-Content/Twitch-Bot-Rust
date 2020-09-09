use async_trait::async_trait;
use crate::irc_chat::response_context::ResponseContext;
use crate::logger::Logger;

#[async_trait]
pub trait MessageParser<TLogger>: Send + Sync + 'static
    where TLogger: Logger{
    async fn process_response(&self, context:&mut ResponseContext, logger:&TLogger) -> bool;
}