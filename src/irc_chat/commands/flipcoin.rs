use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use rand::{Rng, thread_rng};
use std::future::Future;
use tokio::time::delay_for;
use std::time::Duration;


pub fn flipcoin<TLogger> (_parser:DefaultMessageParser<TLogger>, message:TwitchIrcUserMessage, args:Vec<String>, context:&mut ResponseContext, logger:&TLogger) -> Box<dyn Future<Output=()> + Unpin + Send>
    where TLogger: Logger {

    if args.len() > 0 { logger.write_line(String::from("Arguments were given to '!flipcoin', should we not trigger '!flipcoin'? ")); }

    let heads_or_tails = {
        if thread_rng().gen_bool(1.0 / 2.0) {
            String::from("Heads")
        } else {
            String::from("Tails")
        }
    };

    context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel(),
                                                                            format!("A coin somersaults into the air!\nIt lands {}!", heads_or_tails.to_lowercase())));

    Box::new(delay_for(Duration::from_millis(0)))

}