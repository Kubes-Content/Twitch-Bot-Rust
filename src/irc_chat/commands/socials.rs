use crate::irc_chat::commands::{
    send_message_from_user_format, CommandContext, CommandFutureResult,
};
//use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::BotState;
use kubes_web_lib::error::send_result;
use kubes_web_lib::web_socket::Session;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn socials(
    session: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
    if args.len() > 0 {
        println!("Arguments were given to '!flipcoin', should we not trigger '!flipcoin'? ");
    }

    Ok(Box::pin(async move {
        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
                message.get_target_channel(),
                "Socials\n\
                    Patreon: https://patreon.com/KubesContent/\n\
                    Twitter: https://twitter.com/ContentKubes/\n\
                    Discord: https://discord.gg/cB4Pyzk/\n\
                    Instagram: https://www.instagram.com/kubes_content/\n\
                    ArtStation: https://www.artstation.com/kubes",
            ));
        Ok(())
    }))
}
