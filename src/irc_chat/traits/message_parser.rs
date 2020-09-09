use async_trait::async_trait;
use crate::irc_chat::response_context::ResponseContext;
use crate::logger::Logger;
use std::sync::Arc;


#[async_trait]
pub trait MessageParser<TLogger>: Send + Sync + 'static
    where TLogger: Logger{
    async fn process_response(&self, context_mutex:Arc<tokio::sync::Mutex<ResponseContext>>, logger:&TLogger) -> bool;
}