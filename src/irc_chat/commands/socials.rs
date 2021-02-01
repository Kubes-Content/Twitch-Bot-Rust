use crate::irc_chat::commands::{send_message_from_client_user_format, CommandFuture};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use std::sync::Arc;

pub fn socials(
    _parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
) -> CommandFuture {
    if args.len() > 0 {
        println!("Arguments were given to '!flipcoin', should we not trigger '!flipcoin'? ");
    }

    match context_mutex.try_lock() {
        Ok(mut context) => {
            context.add_response_to_reply_with(send_message_from_client_user_format(
                message.get_target_channel(),
                String::from(
                    "Socials\n\
                Patreon: https://patreon.com/KubesContent/\n\
                Twitter: https://twitter.com/ContentKubes/\n\
                Discord: https://discord.gg/cB4Pyzk/\n\
                Instagram: https://www.instagram.com/kubes_content/\n\
                ArtStation: https://www.artstation.com/kubes",
                ),
            ));
        }
        Err(e) => {
            panic!("Error! : {}", e)
        }
    }

    Ok(Box::pin(async { Ok(()) }))
}
