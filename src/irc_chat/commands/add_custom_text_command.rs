use std::ops::Range;

use crate::irc_chat::{response_context::ResponseContext,
                 twitch_user_message::TwitchIrcUserMessage};
use crate::irc_chat::chat_message_parser::IrcMessageParser;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::logger::Logger;
use crate::save_data::default::custom_commands_save_data::CustomCommandsSaveData;


pub fn add_custom_text_command<TParser, TLogger>(parser: TParser, message: TwitchIrcUserMessage, args: Vec<String>, context: &mut ResponseContext, _logger: &TLogger)
    where TParser : IrcMessageParser<TLogger>,
          TLogger: Logger {

    let channel_id = {
        if message.get_target_channel() != context.get_client_user().get_login() {
            _logger.write_line("Failed to add custom command as the target channel is not the client user's.".to_string());
            return;
        }

        context.get_client_user().get_user_id()
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

        let mut custom_commands_save_data = CustomCommandsSaveData::load_or_default(channel_id.clone());
        let result = custom_commands_save_data.add_command(args[0].clone(), command_text);

        // save temp data
        custom_commands_save_data.save(channel_id.clone());

        result
    };

    send_message_from_client_user_format(message.get_target_channel(), return_message);
}

fn is_built_in_command<TParser,TLogger>(parser:TParser, args: Vec<String>) -> bool
    where TParser: IrcMessageParser<TLogger>,
          TLogger: Logger {

    let (commands, alternate_commands) = parser.get_user_commands_including_alternates();

    commands.contains_key(args[0].as_str())
        || alternate_commands.contains_key(args[0].as_str())
}