use crate::irc_chat::commands::{
    send_message_from_user_format, ChatCommandKey, CommandContext, CommandFutureResult,
};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::save_data::default::custom_commands_save_data::CustomCommandsSaveData;
use crate::user::user_properties::ChannelId;
use kubes_web_lib::error::send_result;

pub fn add_custom_text_command(
    parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: CommandContext,
) -> CommandFutureResult {
    let channel_id = {
        let context = send_result::from(context_mutex.try_lock())?;
        let client_data = context.parser.channel.clone();
        if message.get_target_channel() == client_data.get_login() {
            ChannelId::from(client_data.get_user_id())
        } else {
            println!(
                "Failed to add custom command as the target channel is not the client user's."
            );
            return Ok(Box::pin(async { Ok(()) }));
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

            for index in 1..args.len() {
                temp = format!("{0}{1} ", temp, args[index]);
            }

            temp[0..temp.len()-1 /* remove final space */ ].to_string()
        };

        // add command to temp data

        let mut custom_commands_save_data = send_result::from_dyn(CustomCommandsSaveData::load_or_default(channel_id.clone()))?;
        let result = custom_commands_save_data.add_command(ChatCommandKey::from(args[0].clone()), command_text);

        // save temp data
        send_result::from_dyn(custom_commands_save_data.save(channel_id.clone()))?;

        result
    };

    let mut context = send_result::from(context_mutex.try_lock())?;
    context.add_response_to_reply_with(send_message_from_user_format(
        message.get_target_channel(),
        return_message,
    ));

    return Ok(Box::pin(async { Ok(()) }));
}

fn is_built_in_command(parser: DefaultMessageParser, args: Vec<String>) -> bool {
    let (commands, alternate_commands) = parser.get_user_commands_including_alternates();
    commands.contains_key(&ChatCommandKey::from(args[0].to_string()))
        || alternate_commands.contains_key(&ChatCommandKey::from(args[0].to_string()))
}
