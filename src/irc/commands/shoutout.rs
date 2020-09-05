use crate::irc::twitch_user_message::TwitchIrcUserMessage;
use crate::irc::response_context::ResponseContext;
use crate::irc::commands::send_message_from_client_user_format;
use crate::logger::Logger;
use crate::irc::traits::message_parser::IrcMessageParser;


pub fn shoutout<TParser,TLogger>(_parser:TParser, message:TwitchIrcUserMessage, args:Vec<String>, context:&mut ResponseContext, _logger:TLogger)
    where TParser: IrcMessageParser<TLogger>,
          TLogger: Logger {
    let shoutout_reply = {
        // check if name to shoutout is missing
        if args.len() == 0 || args[0].is_empty() || args[0] == "@"{
            format!("A name to shoutout was not given!\nYou screwed up, {}!", message.get_speaker().get_value())
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

    context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel(), shoutout_reply));
}