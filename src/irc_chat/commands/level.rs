use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::{DefaultLogger, Logger};
use crate::save_data::default::user_rpg_stats::UserRpgStats;
use crate::user::user_data::Data as UserData;
use reqwest::header::HeaderMap;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::Error;
use tokio::time::delay_for;

pub fn get_user_level<TLogger>(
    _parser: DefaultMessageParser<TLogger>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    logger: &TLogger,
) -> Box<dyn Future<Output = ()> + Unpin + Send>
where
    TLogger: Logger,
{
    if args.len() > 0 {
        logger.write_line(String::from(
            "Arguments were given to '!level', should we not trigger '!level'? ",
        ));
    }

    Box::new(Box::pin(async move {
        let reqwest_client = reqwest::Client::builder().build().unwrap();

        let sender_id = UserData::get_from_username(
            &reqwest_client,
            message.get_speaker(),
            &DefaultLogger {},
            HeaderMap::new(),
        )
        .await
        .get_user_id();

        println!("Do we need an oauth to get a User's ID?");

        match context_mutex.try_lock() {
            Ok(mut context) => {
                let sender_stats = match UserRpgStats::load_or_default(
                    context.get_client_user_data().get_user_id(),
                    sender_id,
                ) {
                    Ok(stats) => stats,
                    Err(e) => {
                        red!("Error getting user stats! Error: {}", e);
                        return;
                    }
                };

                context.add_response_to_reply_with(send_message_from_client_user_format(
                    message.get_target_channel(),
                    format!(
                        "{0}'s current level is {1}.",
                        message.get_speaker().get_value(),
                        sender_stats.get_current_level()
                    ),
                ));
            }
            Err(e) => panic!("Error! : {}", e),
        }
    }))
}
