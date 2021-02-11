use crate::irc_chat::commands::{
    send_message_from_user_format, CommandContext, CommandFutureResult,
};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::user::is_admin_or_mod;
use kubes_web_lib::error::send_result;

pub fn shoutout(
    _parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: CommandContext,
) -> CommandFutureResult {
    return Ok(Box::pin(async move {
        if !is_admin_or_mod(message.get_speaker(), message.get_target_channel())
            .await
            .unwrap()
        {
            return Ok(());
        }

        let shoutout_reply = {
            // check if name to shoutout is missing
            if args.len() == 0 || args[0].is_empty() || args[0] == "@" {
                format!(
                    "A name to shoutout was not given!\nYou screwed up, {}!",
                    message.get_speaker().get_value()
                )
            } else {
                let name_to_shoutout = {
                    let dirty_name = args[0].clone();
                    if dirty_name.chars().next().unwrap() == '@' {
                        dirty_name[1..].to_string()
                    } else {
                        dirty_name
                    }
                };

                format!("Go check out {0}!\nhttps://twitch.tv/{0}", name_to_shoutout)
            }
        };

        // TODO check if user is a mod or channel owner, to allow shoutout to trigger

        let mut context = send_result::from(context_mutex.try_lock())?;
        context.add_response_to_reply_with(send_message_from_user_format(
            message.get_target_channel(),
            shoutout_reply,
        ));
        Ok(())
    }));
}
