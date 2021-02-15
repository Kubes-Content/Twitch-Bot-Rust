use crate::irc_chat::commands::{send_message_from_user_format, CommandFutureResult};
//use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::BotState;
use kubes_web_lib::web_socket::Session;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn enter_lurk(
    session: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
    if args.len() > 0 {
        println!("Arguments were given to '!lurk', should we not trigger '!lurk'? ");
    }

    Ok(Box::pin(async move {
        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
                message.get_target_channel(),
                format!(
                    "{} bursts into smoke and disperses into the darkness. They will be missed.",
                    message.get_speaker().get_value()
                ),
            ));
        Ok(())
    }))
}
