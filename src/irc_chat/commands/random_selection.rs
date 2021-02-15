use crate::irc_chat::commands::{
    get_user_commands_including_alternates, send_message_from_user_format, ChatCommandKey,
    CommandContext, CommandFutureResult,
};
//use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::BotState;
use kubes_std_lib::random::random_in_range_once;
use kubes_web_lib::error::send_result;
use kubes_web_lib::web_socket::Session;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn random_selection(
    session: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
    let reply_to_send = {
        let mut temp = String::new();

        // validate that there are enough arguments
        if args.len() < 2 {
            temp = String::from("Not enough arguments given to '!random'.");
        }

        // check if heads/tails
        if temp == "" && args.len() == 2 {
            let mut heads = false;
            let mut tails = false;

            let mut heads_tails_check = |s: String| match s.to_lowercase().as_str() {
                "heads" => heads = true,
                "tails" => tails = true,
                _ => {}
            };

            heads_tails_check(args[0].clone());
            heads_tails_check(args[1].clone());

            if heads && tails {
                let flipcoin_func = send_result::from_option(
                    get_user_commands_including_alternates()
                        .0
                        .get(&ChatCommandKey::from("flipcoin".to_string())),
                )?
                .clone();

                return Ok(Box::pin(async move {
                    flipcoin_func
                        .lock()
                        .await
                        .run(session, message, vec![])
                        .await?;
                    Ok(())
                }));
            }
        }

        // pick random
        if temp == "" {
            temp = args[random_in_range_once(0..(args.len() as u32)) as usize].clone();
        }

        temp
    };

    Ok(Box::pin(async move {
        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
                message.get_target_channel(),
                reply_to_send,
            ));

        Ok(())
    }))
}
