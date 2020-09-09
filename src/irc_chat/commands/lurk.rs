use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use std::future::Future;
use tokio::time::delay_for;
use std::time::Duration;


pub fn enter_lurk<TLogger>(_parser:DefaultMessageParser<TLogger>, message:TwitchIrcUserMessage, args:Vec<String>, context:&mut ResponseContext, logger:&TLogger) -> Box<dyn Future<Output=()> + Unpin + Send>
    where TLogger: Logger {

    if args.len() > 0 { logger.write_line(String::from("Arguments were given to '!lurk', should we not trigger '!lurk'? ")); }



    context.add_response_to_reply_with(send_message_from_client_user_format(
        message.get_target_channel(),
        format!("{} bursts into smoke and disperses into the darkness. They will be missed.", message.get_speaker().get_value())));

    Box::new(delay_for(Duration::from_millis(0)))

}