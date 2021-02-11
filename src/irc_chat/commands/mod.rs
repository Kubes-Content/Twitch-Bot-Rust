use crate::irc_chat::parsers::default_irc_message_parser::DefaultMessageParser;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::user::user_properties::UserLogin;
use futures::future::BoxFuture;
use kubes_web_lib::error::{SendError, SendResult};
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

primitive_wrapper!(
    ChatCommandKey,
    String,
    "(ChatCommandKey: \"{}\")",
    serialize
);

pub type CommandContext<'l> = Arc<tokio::sync::Mutex<ResponseContext<'l, DefaultMessageParser>>>;

pub type CommandFutureResult<'l> =
    Result<BoxFuture<'l, Result<(), Box<dyn SendError>>>, Box<dyn SendError>>;

pub type RereferenceableChatCommand = Arc<Mutex<ChatCommand>>;

pub struct ChatCommand {
    command: Box<
        dyn for<'f> Fn(
                DefaultMessageParser,
                TwitchIrcUserMessage,
                Vec<String>,
                Arc<Mutex<ResponseContext<'f, DefaultMessageParser>>>,
            ) -> CommandFutureResult
            + Send
            + Sync
            + 'static,
    >,
}

unsafe impl Send for ChatCommand {}
unsafe impl Sync for ChatCommand {}

impl ChatCommand {
    pub fn new(
        f: for<'a> fn(
            DefaultMessageParser,
            TwitchIrcUserMessage,
            Vec<String>,
            Arc<Mutex<ResponseContext<'a, DefaultMessageParser>>>,
        ) -> CommandFutureResult,
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
        context: Arc<Mutex<ResponseContext<'_, DefaultMessageParser>>>,
    ) -> SendResult<()> {
        match (self.command)(parser, message, args, context)?.await {
            Ok(_) => {}
            Err(e) => println!("Error executing command: {}", e.to_string()),
        };
        Ok(())
    }
}

pub fn send_message_from_user_format(channel: UserLogin, message: impl ToString) -> String {
    format!(
        "PRIVMSG #{0} :{1}",
        channel.get_value(),
        message.to_string()
    )
}
