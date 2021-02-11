use crate::irc_chat::commands::{
    send_message_from_user_format, ChatCommandKey, CommandContext, CommandFutureResult,
};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use kubes_std_lib::random::random_in_range_once;
use kubes_web_lib::error::send_result;

pub fn random_selection(
    parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: CommandContext,
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
                    parser
                        .user_commands
                        .get(&ChatCommandKey::from("flipcoin".to_string())),
                )?
                .clone();

                return Ok(Box::pin(async move {
                    flipcoin_func
                        .lock()
                        .await
                        .run(parser, message, vec![], context_mutex)
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

    send_result::from(context_mutex.try_lock())?.add_response_to_reply_with(
        send_message_from_user_format(message.get_target_channel(), reply_to_send),
    );

    Ok(Box::pin(async { Ok(()) }))
}
