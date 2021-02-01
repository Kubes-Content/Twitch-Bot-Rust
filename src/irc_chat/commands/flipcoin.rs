use crate::irc_chat::commands::{send_message_from_client_user_format, CommandFuture};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub fn flipcoin(
    _parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
) -> CommandFuture {
    if args.len() > 0 {
        println!("Arguments were given to '!flipcoin', should we not trigger '!flipcoin'? ");
    }

    let heads_or_tails = {
        if thread_rng().gen_bool(1.0 / 2.0) {
            String::from("Heads")
        } else {
            String::from("Tails")
        }
    };

    match context_mutex.try_lock() {
        Ok(mut context) => {
            context.add_response_to_reply_with(send_message_from_client_user_format(
                message.get_target_channel(),
                format!(
                    "A coin somersaults into the air!\nIt lands {}!",
                    heads_or_tails.to_lowercase()
                ),
            ));
        }
        Err(e) => {
            panic!("Error! : {}", e)
        }
    }

    Ok(Box::pin(async { Ok(()) }))
}
