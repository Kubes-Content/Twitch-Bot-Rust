use std::ops::Range;

use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::{
    response_context::ResponseContext, twitch_user_message::TwitchIrcUserMessage,
};
use crate::logger::Logger;
use crate::save_data::default::custom_commands_save_data::CustomCommandsSaveData;
use std::future::Future;
use std::sync::Arc;
use tokio::io::Error;
use tokio::time::{delay_for, Duration};

pub fn add_custom_text_command<TLogger>(
    parser: DefaultMessageParser<TLogger>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    _logger: &TLogger,
) -> Box<dyn Future<Output = ()> + Unpin + Send>
where
    TLogger: Logger,
{
    let channel_id = {
        match context_mutex.try_lock() {
            Ok(context) => {
                let client_data = context.get_client_user_data();
                if message.get_target_channel() == client_data.get_login() {
                    client_data.get_user_id()
                } else {
                    _logger.write_line("Failed to add custom command as the target channel is not the client user's.".to_string());
                    return Box::new(delay_for(Duration::from_millis(0)));
                }
            }
            Err(e) => panic!("Error! : {}", e),
        }
    };

    let return_message =
    // check if at least a command name and a single word to display for that command is present
    if args.len() < 2 {

        "WARNING: Did not receive a command name and the text to display when that command is fired.".to_string()

    } else if is_built_in_command(parser, args.clone()) {

        format!("WARNING: Cannot create custom command. \"!{}\" is already a built-in command.", args[0].clone())

    } else {

        let command_text = {

            let mut temp = String::new();

            let range = Range { start: 1, end: args.len() };
            for index in range {
                temp = format!("{0}{1} ", temp, args[index]);
            }

            // remove final space
            temp[0..temp.len()-1].to_string()
        };

        // add command to temp data

        let mut custom_commands_save_data = match CustomCommandsSaveData::load_or_default(channel_id.clone()) {
            Ok(commands_data) => commands_data,
            Err(e) => panic!("Error getting custom commands. Error: {}", e)
        };
        let result = custom_commands_save_data.add_command(args[0].clone(), command_text);

        // save temp data
        custom_commands_save_data.save(channel_id.clone());

        result
    };
    match context_mutex.try_lock() {
        Ok(mut context) => {
            context.add_response_to_reply_with(send_message_from_client_user_format(
                message.get_target_channel(),
                return_message,
            ));
        }
        Err(e) => panic!("Error! : {}", e),
    }

    Box::new(delay_for(Duration::from_millis(0)))
}

fn is_built_in_command<TLogger>(parser: DefaultMessageParser<TLogger>, args: Vec<String>) -> bool
where
    TLogger: Logger,
{
    let (commands, alternate_commands) = parser.get_user_commands_including_alternates();

    commands.contains_key(args[0].as_str()) || alternate_commands.contains_key(args[0].as_str())
}
