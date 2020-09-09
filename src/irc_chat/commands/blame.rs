use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use std::future::Future;
use std::time::Duration;
use tokio::time::delay_for;
use crate::irc_chat::channel_chatter_data::ChatterData;
use rand::Rng;
use crate::user::user_properties::UserLogin;
use futures::FutureExt;
use std::sync::{Mutex, MutexGuard, TryLockError};


pub fn blame_random_user<TLogger>(_parser:DefaultMessageParser<TLogger>, message:TwitchIrcUserMessage, args:Vec<String>, context:tokio::sync::Mutex<ResponseContext>, logger:&TLogger) -> Box<dyn Future<Output=()> + Unpin + Send>
    where TLogger: Logger {

    if args.len() > 0 { logger.write_line(String::from("Arguments were given to '!blame', should we not trigger '!blame'? ")); }

    // DEBUG
    Box::new(Box::pin(async move {

        match context.try_lock() {
            Ok(mut context_mutex) => {
                let client_login = context_mutex.get_client_user().get_login();
                context_mutex.add_response_to_reply_with(blame_random_user_async(client_login).await);
            },
            Err(_) => {},
        }

    }))
}

async fn blame_random_user_async(client_login:UserLogin) -> String {
    let reqwest_client:reqwest::Client = reqwest::Client::builder().build().unwrap();
    let chatter_data = ChatterData::from_channel(&reqwest_client, client_login).await;

    let chatters = chatter_data.get_all_viewers(true,true);

    let index = rand::thread_rng().gen_range(0, chatters.len());

    format!("{} is clearly the issue.", chatters[index].get_value())
}