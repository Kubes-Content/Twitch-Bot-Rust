use crate::irc_chat::commands::{
    send_message_from_user_format, CommandContext, CommandFutureResult,
};
//use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::save_data::default::user_rpg_stats::UserRpgStats;
use crate::user::user_data::UserData;
use crate::BotState;
use kubes_std_lib::logging::DefaultLogger;
use kubes_web_lib::error::send_result;
use kubes_web_lib::web_socket::Session;
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn get_user_level(
    session: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
    if args.len() > 0 {
        println!("Arguments were given to '!level', should we not trigger '!level'? ");
    }

    Ok(Box::pin(async move {
        let reqwest_client = send_result::from(Client::builder().build())?;

        let sender_id = send_result::from_dyn(
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

        let sender_stats = send_result::from_dyn(UserRpgStats::load_or_default(
            session
                .lock()
                .await
                .state
                .irc_channel
                .owner_data
                .get_user_id(),
            sender_id,
        ))?;

        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
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
