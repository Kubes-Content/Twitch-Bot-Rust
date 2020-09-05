use crate::irc::twitch_user_message::TwitchIrcUserMessage;
use crate::irc::response_context::ResponseContext;
use crate::irc::commands::send_message_from_client_user_format;
use crate::logger::Logger;
use crate::irc::traits::message_parser::IrcMessageParser;


pub fn socials<TParser,TLogger>(_parser:TParser, message:TwitchIrcUserMessage, args:Vec<String>, context:&mut ResponseContext, logger:TLogger)
    where TParser: IrcMessageParser<TLogger>,
          TLogger: Logger {

    if args.len() > 0 { logger.write_line(String::from("Arguments were given to '!flipcoin', should we not trigger '!flipcoin'? ")); }



    context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel(), String::from(
        "Patreon: https://patreon.com/KubesContent/\
        Twitter: https://twitter.com/ContentKubes/\
        Discord: https://discord.gg/cB4Pyzk/\
        Instagram: https://www.instagram.com/kubes_content/\
        ArtStation: https://www.artstation.com/kubes")));
}