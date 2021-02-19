use crate::irc_chat::channel_chatter_data::ChatterData;
use crate::irc_chat::commands::{send_message_from_user_format, CommandFutureResult};
//use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::user::user_properties::UserLogin;
use crate::BotState;
use kubes_web_lib::error::send_result;
use kubes_web_lib::web_socket::Session;
use rand::Rng;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn blame_random_user(
    session_mutex: Arc<Mutex<Session<BotState>>>,
    _message: TwitchIrcUserMessage,
    args: Vec<String>,
) -> CommandFutureResult {
    if args.len() > 0 {
        println!("Arguments were given to '!blame', should we not trigger '!blame'? ");
    }

    return Ok(Box::pin(async move {
        let mut session = session_mutex.lock().await;
        session.send_string("THIS IS A TEST MESSAGE");

        let client_login = session.state.irc_channel.owner_data.get_login();
        session.send_string(send_message_from_user_format(
            client_login.clone(),
            send_result::from_dyn(blame_random_user_async(client_login).await)?,
        ));
        Ok(())
    }));
}

async fn blame_random_user_async(client_login: UserLogin) -> Result<String, Box<dyn Error>> {
    let reqwest_client: reqwest::Client = reqwest::Client::builder().build().unwrap();
    let chatter_data = ChatterData::from_channel(&reqwest_client, client_login).await?;

    let chatters = chatter_data.get_all_viewers(true, true);

    let index = rand::thread_rng().gen_range(0, chatters.len());

    Ok(format!(
        "{} is clearly the issue.",
        chatters[index].get_value()
    ))
}
