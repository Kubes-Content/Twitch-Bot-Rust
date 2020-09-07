use crate::irc_chat::chat_message_parser::IrcMessageParser;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;


pub fn all_commands<TParser,TLogger>(parser: TParser, message: TwitchIrcUserMessage, args: Vec<String>, context: &mut ResponseContext, logger: &TLogger)
where TParser : IrcMessageParser<TLogger>,
    TLogger: Logger {
    if args.len() > 0 {
        logger.write_line(String::from("Should we be triggering '!Commands' when arguments are given?"))
    }

    let commands = {
        let mut temp = String::new();

        for command in parser.get_user_commands().keys() {
            temp = format!("{0}!{1} ", temp, command);
        }

        // remove trailing whitespace
        if temp.len() > 0 {
            temp = temp[0..temp.len() - 1].to_string();
        }

        temp
    };

    println!("WARNING: All_commands is not including custom commands.");

    context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel(), format!("Commands: {}", commands)));
}
