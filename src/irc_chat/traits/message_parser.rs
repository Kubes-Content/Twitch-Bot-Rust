use crate::irc_chat::response_context::ResponseContext;
use async_trait::async_trait;
use kubes_web_lib::error::SendError;
use std::sync::Arc;

#[async_trait]
pub trait MessageParser<TSelf: MessageParser<TSelf>>: Send + Sync + 'static {
    async fn process_response(
        &self,
        context_mutex: Arc<tokio::sync::Mutex<ResponseContext<'_, TSelf>>>,
    ) -> Result<(), Box<dyn SendError>>;
}
