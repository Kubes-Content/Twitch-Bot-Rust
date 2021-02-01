use crate::irc_chat::commands::{send_message_from_client_user_format, CommandFuture};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use std::sync::Arc;

pub fn all_commands(
    parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
) -> CommandFuture {
    if args.len() > 0 {
        println!("Should we be triggering '!Commands' when arguments are given?");
    }

    let commands = {
        let mut temp = String::new();

        for command in parser.user_commands.keys() {
            temp = format!("{0}!{1} ", temp, command);
        }

        // remove trailing whitespace
        if temp.len() > 0 {
            temp = temp[0..temp.len() - 1].to_string();
        }

        temp
    };

    println!("WARNING: All_commands is not including custom commands.");

    let mut context = context_mutex.try_lock().expect("Error!");
    context.add_response_to_reply_with(send_message_from_client_user_format(
        message.get_target_channel(),
        format!("Commands: {}", commands),
    ));

    Ok(Box::pin(async { Ok(()) }))
}
