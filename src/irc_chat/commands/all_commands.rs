use crate::irc_chat::commands::{
    get_user_commands_including_alternates, send_message_from_user_format, CommandFutureResult,
};
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::BotState;
use kubes_web_lib::web_socket::Session;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn all_commands(
    session: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
    Ok(Box::pin(async move {
        if args.len() > 0 {
            println!("Should we be triggering '!Commands' when arguments are given?");
        }

        let commands = {
            let mut temp = String::new();
            let primary_native_commands = get_user_commands_including_alternates().0;
            for command in primary_native_commands.keys() {
                temp = format!("{0}!{1} ", temp, command.get_value());
            }

            // remove trailing whitespace
            if temp.len() > 0 {
                temp = temp[0..temp.len() - 1].to_string();
            }

            temp
        };

        println!("WARNING: all_commands is not including custom commands.");

        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
                message.get_target_channel(),
                format!("Commands: {}", commands),
            ));

        Ok(())
    }))
}
