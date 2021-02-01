use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::save_data::default::user_rpg_stats::UserRpgStats;
use crate::send_error::{get_result, get_result_dyn, to_error, SendError};
use crate::user::user_data::UserData;
use kubes_std_lib::logging::{DefaultLogger, Logger};
use reqwest::header::HeaderMap;
use std::future::Future;
use std::sync::Arc;

pub fn get_user_level(
    _parser: DefaultMessageParser,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    logger: &impl Logger,
) -> Box<dyn Future<Output = Result<(), Box<dyn SendError>>> + Unpin + Send> {
    if args.len() > 0 {
        logger.write_line(String::from(
            "Arguments were given to '!level', should we not trigger '!level'? ",
        ));
    }

    Box::new(Box::pin(async move {
        let reqwest_client = get_result(reqwest::Client::builder().build())?;

        let sender_id = get_result_dyn(
            UserData::get_from_username(
                &reqwest_client,
                message.get_speaker(),
                &DefaultLogger {},
                HeaderMap::new(),
            )
            .await,
        )?
        .get_user_id();

        println!("Do we need an oauth to get a User's ID?");

        let mut context = get_result(context_mutex.try_lock())?;
        let sender_stats = match UserRpgStats::load_or_default(
            context.get_client_user_data().get_user_id(),
            sender_id,
        ) {
            Ok(stats) => stats,
            Err(e) => {
                return Err(to_error(e));
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

        return Ok(());
    }))
}
