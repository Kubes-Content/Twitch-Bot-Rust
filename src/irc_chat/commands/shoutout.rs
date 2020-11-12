use crate::irc_chat::channel_chatter_data::ChatterData;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use crate::user::is_admin_or_mod;
use std::future::Future;
use std::sync::Arc;

pub fn shoutout<TLogger>(
    _parser: DefaultMessageParser<TLogger>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    _logger: &TLogger,
) -> Box<dyn Future<Output = ()> + Unpin + Send>
where
    TLogger: Logger,
{
    Box::new(Box::pin(async move {
        if !is_admin_or_mod(message.get_speaker(), message.get_target_channel()).await {
            return;
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

        match context_mutex.try_lock() {
            Ok(mut context) => {
                context.add_response_to_reply_with(send_message_from_client_user_format(
                    message.get_target_channel(),
                    shoutout_reply,
                ));
            }
            Err(e) => panic!("Error! : {}", e),
        }
    }))
}
