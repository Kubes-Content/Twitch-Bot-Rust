use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use std::future::Future;
use tokio::time::{delay_for, Duration};
use std::sync::Arc;


pub fn all_commands<TLogger>(parser: DefaultMessageParser<TLogger>, message: TwitchIrcUserMessage, args: Vec<String>, context_mutex:Arc<tokio::sync::Mutex<ResponseContext>>, logger: &TLogger) -> Box<dyn Future<Output=()> + Unpin + Send>
where TLogger: Logger {
    if args.len() > 0 {
        logger.write_line(String::from("Should we be triggering '!Commands' when arguments are given?"))
    }

    let commands = {
        let mut temp = String::new();

        for command in parser.get_user_commands().keys() {
            temp = format!("{0}!{1} ", temp, command);
        }

        // remove trailing whitespace
        if temp.len() > 0 {
            temp = temp[0..temp.len() - 1].to_string();
        }

        temp
    };

    println!("WARNING: All_commands is not including custom commands.");

    match context_mutex.try_lock() {
        Ok(mut context) => {
            context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel(), format!("Commands: {}", commands)));
        }
        Err(e) => { panic!("Error! : {}", e) }
    }

    Box::new(delay_for(Duration::from_millis(0)))
}

