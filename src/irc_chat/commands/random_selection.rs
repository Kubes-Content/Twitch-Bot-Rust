use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use rand::{Rng, thread_rng};
use std::future::Future;
use tokio::time::delay_for;
use std::time::Duration;


pub fn random_selection<TLogger>(parser:DefaultMessageParser<TLogger>, message:TwitchIrcUserMessage, args:Vec<String>, context:&mut ResponseContext, logger:&TLogger) -> Box<dyn Future<Output=()> + Unpin + Send>
    where TLogger: Logger {
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

            let mut heads_tails_check = |s: String| {
                match s.to_lowercase().as_str() {
                    "heads" => { heads = true }
                    "tails" => { tails = true }
                    _ => { }
                }
            };

            heads_tails_check(args[0].clone());
            heads_tails_check(args[1].clone());

            if heads && tails {
                if let Some(flipcoin_func) = parser.get_user_commands().get("flipcoin") {
                    return (*flipcoin_func)(parser, message, vec![], context, logger);
                } else {
                    temp = String::from("Use \"!flipcoin\" instead.");
                }
            }
        }

        // pick random
        if temp == "" {
            temp = args[thread_rng().gen_range(0, args.len())].clone();
        }

        temp
    };

    context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel(), reply_to_send));

    Box::new(delay_for(Duration::from_millis(0)))
}