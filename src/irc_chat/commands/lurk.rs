use crate::irc_chat::commands::{send_message_from_client_user_format, CommandFuture};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use std::sync::Arc;

pub fn enter_lurk(
    _parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
) -> CommandFuture {
    if args.len() > 0 {
        println!("Arguments were given to '!lurk', should we not trigger '!lurk'? ");
    }

    let mut context = context_mutex.try_lock().expect("Error in enter_lurk!");
    context.add_response_to_reply_with(send_message_from_client_user_format(
        message.get_target_channel(),
        format!(
            "{} bursts into smoke and disperses into the darkness. They will be missed.",
            message.get_speaker().get_value()
        ),
    ));

    Ok(Box::pin(async { Ok(()) }))
}
