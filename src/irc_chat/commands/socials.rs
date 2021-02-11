use crate::irc_chat::commands::{
    send_message_from_user_format, CommandContext, CommandFutureResult,
};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use kubes_web_lib::error::send_result;

pub fn socials(
    _parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: CommandContext,
) -> CommandFutureResult {
    if args.len() > 0 {
        println!("Arguments were given to '!flipcoin', should we not trigger '!flipcoin'? ");
    }

    send_result::from(context_mutex.try_lock())?.add_response_to_reply_with(
        send_message_from_user_format(
            message.get_target_channel(),
            "Socials\n\
                Patreon: https://patreon.com/KubesContent/\n\
                Twitter: https://twitter.com/ContentKubes/\n\
                Discord: https://discord.gg/cB4Pyzk/\n\
                Instagram: https://www.instagram.com/kubes_content/\n\
                ArtStation: https://www.artstation.com/kubes",
        ),
    );

    Ok(Box::pin(async { Ok(()) }))
}
