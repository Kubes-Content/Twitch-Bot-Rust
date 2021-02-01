use crate::irc_chat::channel_chatter_data::ChatterData;
use crate::irc_chat::commands::{send_message_from_client_user_format, CommandFuture};
use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::send_error::{get_result, get_result_dyn};
use crate::user::user_properties::UserLogin;
use rand::Rng;
use std::error::Error;
use std::sync::Arc;

pub fn blame_random_user(
    _parser: DefaultMessageParser,
    _message: TwitchIrcUserMessage,
    args: Vec<String>,
    context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
) -> CommandFuture {
    if args.len() > 0 {
        println!("Arguments were given to '!blame', should we not trigger '!blame'? ");
    }

    return Ok(Box::pin(async move {
        let mut context = get_result(context_mutex.try_lock())?;
        context.add_response_to_reply_with(format!("THIS IS A TEST MESSAGE"));

        let client_login = context.get_client_user_data().get_login();
        context.add_response_to_reply_with(send_message_from_client_user_format(
            client_login.clone(),
            get_result_dyn(blame_random_user_async(client_login).await)?,
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
