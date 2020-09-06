use crate::irc::response_context::ResponseContext;
use crate::irc::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use crate::irc::commands::send_message_from_client_user_format;
use rand::{Rng, thread_rng};
use crate::irc::chat_message_parser::IrcMessageParser;


pub fn flipcoin<TParser,TLogger> (_parser:TParser, message:TwitchIrcUserMessage, args:Vec<String>, context:&mut ResponseContext, logger:&TLogger)
    where TParser: IrcMessageParser<TLogger>,
        TLogger: Logger {

    if args.len() > 0 { logger.write_line(String::from("Arguments were given to '!flipcoin', should we not trigger '!flipcoin'? ")); }

    let heads_or_tails = {
        if thread_rng().gen_bool(1.0 / 2.0) {
            String::from("Heads")
        } else {
            String::from("Tails")
        }
    };

    context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel(),
                                                                            format!("A coin somersaults into the air!\nIt lands {}!", heads_or_tails.to_lowercase())))
}