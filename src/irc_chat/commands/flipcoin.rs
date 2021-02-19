use crate::irc_chat::commands::{send_message_from_user_format, CommandFutureResult};
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::BotState;
use kubes_web_lib::web_socket::Session;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn flipcoin(
    session: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
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

    Ok(Box::pin(async move {
        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
                message.get_target_channel(),
                format!(
                    "A coin somersaults into the air!\nIt lands {}!",
                    heads_or_tails.to_lowercase()
                ),
            ));

        Ok(())
    }))
}
