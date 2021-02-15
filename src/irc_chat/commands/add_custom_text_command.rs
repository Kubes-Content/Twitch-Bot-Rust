use crate::irc_chat::commands::{
    is_built_in_command, send_message_from_user_format, ChatCommand, ChatCommandKey,
    CommandContext, CommandFutureResult,
};
//use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::commands::all_commands::all_commands;
use crate::irc_chat::commands::blame::blame_random_user;
use crate::irc_chat::commands::flipcoin::flipcoin;
use crate::irc_chat::commands::lurk::enter_lurk;
use crate::irc_chat::commands::random_selection::random_selection;
use crate::irc_chat::commands::shoutout::shoutout;
use crate::irc_chat::commands::socials::socials;
use crate::irc_chat::parsers::default_irc_message_parser::UserNativeCommandsMap;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::save_data::default::custom_commands_save_data::CustomCommandsSaveData;
use crate::user::user_properties::{ChannelData, ChannelId};
use crate::BotState;
use kubes_web_lib::error::send_result;
use kubes_web_lib::web_socket::Session;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn add_custom_text_command(
    session: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
    return Ok(Box::pin(async move {
        let channel_id = {
            let client_data = session.lock().await.state.irc_channel.owner_data.clone();
            if message.get_target_channel() == client_data.get_login() {
                ChannelId::from(client_data.get_user_id())
            } else {
                println!(
                    "Failed to add custom command as the target channel is not the client user's."
                );
                return Ok(());
            }
        };

        let return_message =
    // check if at least a command name and a single word to display for that command is present
    if args.len() < 2 {

        "WARNING: Did not receive a command name and the text to display when that command is fired.".to_string()

    } else if is_built_in_command(args.clone()) {

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

        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
                message.get_target_channel(),
                return_message,
            ));

        Ok(())
    }));
}
