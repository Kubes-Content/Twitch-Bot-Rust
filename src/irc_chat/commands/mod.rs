use crate::irc_chat::parsers::default_irc_message_parser::UserNativeCommandsMap;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::user::user_properties::UserLogin;
use crate::BotState;
use kubes_web_lib::error::{SendError, SendResult};
use kubes_web_lib::web_socket::Session;
use std::sync::Arc;
use tokio::sync::Mutex;

mod add_custom_text_command;
mod all_commands;
mod blame;
mod flipcoin;
mod level;
mod lurk;
mod random_selection;
mod shoutout;
mod socials;

pub use add_custom_text_command::add_custom_text_command;
pub use all_commands::all_commands;
pub use blame::blame_random_user;
pub use flipcoin::flipcoin;
use futures::Future;
pub use level::get_user_level;
pub use lurk::enter_lurk;
pub use random_selection::random_selection;
pub use shoutout::shoutout;
pub use socials::socials;
use std::pin::Pin;

primitive_wrapper!(
    ChatCommandKey,
    String,
    "(ChatCommandKey: \"{}\")",
    serialize
);

pub type CommandContext = ResponseContext;

pub type CommandFutureResult = Result<
    Pin<Box<(dyn Future<Output = Result<(), Box<dyn SendError>>> + Send)>>,
    Box<dyn SendError>,
>;

pub type RereferenceableChatCommand = Arc<Mutex<ChatCommand>>;

pub struct ChatCommand {
    command: Box<
        dyn Fn(
                Arc<Mutex<Session<BotState>>>,
                TwitchIrcUserMessage,
                Vec<String>,
            ) -> CommandFutureResult
            + Send
            + Sync,
    >,
}

unsafe impl Send for ChatCommand {}
unsafe impl Sync for ChatCommand {}

impl ChatCommand {
    pub fn new(
        f: fn(
            Arc<Mutex<Session<BotState>>>,
            TwitchIrcUserMessage,
            Vec<String>,
        ) -> CommandFutureResult,
    ) -> ChatCommand {
        ChatCommand {
            command: Box::new(move |a, b, c| Ok(Box::pin(f(a, b, c)?))),
        }
    }

    pub async fn run(
        &self,
        session: Arc<Mutex<Session<BotState>>>,
        message: TwitchIrcUserMessage,
        args: Vec<String>,
    ) -> SendResult<()> {
        match (self.command)(session, message, args)?.await {
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

fn is_built_in_command(args: Vec<String>) -> bool {
    let (commands, alternate_commands) = get_user_commands_including_alternates();
    commands.contains_key(&ChatCommandKey::from(args[0].to_string()))
        || alternate_commands.contains_key(&ChatCommandKey::from(args[0].to_string()))
}
//
pub fn get_user_commands_including_alternates() -> (UserNativeCommandsMap, UserNativeCommandsMap) {
    // load commands

    let mut primary_commands_map = UserNativeCommandsMap::new();
    let mut alternate_keys_commands_map = UserNativeCommandsMap::new();
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "addcommand",
        vec!["newcommand"],
        ChatCommand::new(add_custom_text_command),
    );
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "commands",
        vec!["allcommands"],
        ChatCommand::new(all_commands),
    );
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "blame",
        vec!["scapegoat"],
        ChatCommand::new(blame_random_user),
    );
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "flipcoin",
        vec!["5050", "50-50"],
        ChatCommand::new(flipcoin),
    );
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "lurk",
        vec!["afk", "busy"],
        ChatCommand::new(enter_lurk),
    );
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "random",
        vec!["select"],
        ChatCommand::new(random_selection),
    );
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "shoutout",
        vec!["shout", "so"],
        ChatCommand::new(shoutout),
    );
    add_default_command(
        &mut primary_commands_map,
        &mut alternate_keys_commands_map,
        "socials",
        vec!["social"],
        ChatCommand::new(socials),
    );

    (primary_commands_map, alternate_keys_commands_map)
}

//
fn add_default_command(
    commands: &mut UserNativeCommandsMap,
    commands_alternate_keys: &mut UserNativeCommandsMap,
    primary_key: &str,
    alternate_keys: Vec<&str>,
    command_ref: ChatCommand,
) {
    let arc_cmd = Arc::new(Mutex::new(command_ref));

    if commands.contains_key(&ChatCommandKey::from(primary_key.to_string())) {
        println!(
            "WARNING: The irc-command key '{}' is used multiple times.",
            primary_key
        );
    }

    commands.insert(
        ChatCommandKey::from(primary_key.to_string()),
        arc_cmd.clone(),
    );

    for alt_key in alternate_keys {
        let key = ChatCommandKey::from(alt_key.to_string());
        if commands.contains_key(&key) {
            println!(
                "WARNING: The irc-command key '{}' is used multiple times.",
                alt_key
            );
        }

        commands_alternate_keys.insert(key, arc_cmd.clone());
    }
}
