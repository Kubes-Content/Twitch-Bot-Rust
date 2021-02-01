use crate::irc_chat::response_context::ResponseContext;
use crate::send_error::SendError;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait MessageParser: Send + Sync + 'static {
    async fn process_response(
        &self,
        context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> Result<(), Box<dyn SendError>>;
}
