use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::send_error::SendError;
use crate::user::user_properties::UserLogin;
use futures::future::BoxFuture;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod add_custom_text_command;
pub mod all_commands;
pub mod blame;
pub mod flipcoin;
pub mod level;
pub mod lurk;
pub mod random_selection;
pub mod shoutout;
pub mod socials;

pub type CommandFuture =
    Result<BoxFuture<'static, Result<(), Box<dyn SendError>>>, Box<dyn SendError>>;

pub type RereferenceableChatCommand = Arc<Mutex<ChatCommand>>;

pub struct ChatCommand {
    command: Box<
        dyn Fn(
                DefaultMessageParser,
                TwitchIrcUserMessage,
                Vec<String>,
                Arc<tokio::sync::Mutex<ResponseContext>>,
            ) -> CommandFuture
            + Send
            + Sync
            + 'static,
    >,
}

//unsafe impl<x> Send for Box<x> {}

unsafe impl Send for ChatCommand {}
unsafe impl Sync for ChatCommand {}

impl ChatCommand {
    pub fn new(
        f: for<'a> fn(
            DefaultMessageParser,
            TwitchIrcUserMessage,
            Vec<String>,
            Arc<tokio::sync::Mutex<ResponseContext>>,
        ) -> CommandFuture,
    ) -> ChatCommand {
        ChatCommand {
            command: Box::new(move |a, b, c, d| Ok(Box::pin(f(a, b, c, d)?))),
        }
    }

    pub async fn run(
        &self,
        parser: DefaultMessageParser,
        message: TwitchIrcUserMessage,
        args: Vec<String>,
        context: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> Result<(), Box<dyn SendError>> {
        match (self.command)(parser, message, args, context)?.await {
            Ok(_) => {}
            Err(e) => println!("Error: {}", e.to_string()),
        };
        Ok(())
    }
}

pub fn send_message_from_client_user_format(channel: UserLogin, message: String) -> String {
    format!("PRIVMSG #{0} :{1}", channel.get_value(), message)
}
