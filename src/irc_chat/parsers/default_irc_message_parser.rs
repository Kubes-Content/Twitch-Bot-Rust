use crate::irc_chat::commands::{ChatCommand, RereferenceableChatCommand};
use crate::send_error::{get_option, get_result, get_result_dyn, KubesError, SendError};
use crate::{
    irc_chat::{
        commands::{
            add_custom_text_command::add_custom_text_command, all_commands::all_commands,
            blame::blame_random_user, flipcoin::flipcoin, lurk::enter_lurk,
            random_selection::random_selection, send_message_from_client_user_format,
            shoutout::shoutout, socials::socials,
        },
        response_context::ResponseContext,
        traits::message_parser::MessageParser,
        twitch_message_type::TwitchIrcMessageType,
        twitch_user_message::TwitchIrcUserMessage,
    },
    save_data::default::custom_commands_save_data::CustomCommandsSaveData,
    user::user_properties::UserLogin,
};
use async_trait::async_trait;
use kubes_std_lib::text::impl_to_string::{begins_with, remove_within};
use std::{collections::HashMap, string::ToString, sync::Arc};
use tokio::sync::Mutex;

pub type UserCommandsMap = HashMap<String, RereferenceableChatCommand>;

#[derive(Clone, Default)]
pub struct DefaultMessageParser {
    pub user_commands: UserCommandsMap,
    pub user_commands_alternate_keywords: UserCommandsMap,
}

unsafe impl Send for DefaultMessageParser {}

unsafe impl Sync for DefaultMessageParser {}

#[async_trait]
impl MessageParser for DefaultMessageParser {
    async fn process_response(
        &self,
        context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> Result<(), Box<dyn SendError>> {
        let response_received = get_result(context_mutex.try_lock())?
            .get_initial_response()
            .clone();

        let mut deconstructing_response = response_received.clone();

        const TMI_TWITCH: &str = ":tmi.twitch.tv ";

        if begins_with(&response_received, TMI_TWITCH) {
            deconstructing_response = remove_within(&deconstructing_response, TMI_TWITCH);

            match &deconstructing_response[..3] {
                "001" => { /*Welcome, GLHF*/ }
                "002" => { /*Your host is tmi.twitch.tv*/ }
                "003" => { /*This server is rather new*/ }
                "004" => { /*-*/ }
                "372" => { /*You are in a maze of twisty passages, all alike.*/ }
                "375" => { /*-*/ }
                "376" => { /*>*/ }
                "421" => { /* Unknown command */ }
                _ => {
                    println!(
                        "IRC parser Not aware of Twitch-code {0} for line: {1}",
                        deconstructing_response[..3].to_string(),
                        response_received
                    );
                }
            }
        } else if begins_with(&response_received, "PING ") {
            get_result(context_mutex.try_lock())?
                .add_response_to_reply_with(String::from("PONG :tmi.twitch.tv"));
        } else {
            let z = self.decipher_response_message(context_mutex.clone()).await;
            match z {
                Err(e) => println!(
                    "IF THIS ISNT A USER MESSAGE.... WTF IS IT?? {}",
                    e.to_string()
                ),
                Ok(message) => {
                    match message {
                        TwitchIrcMessageType::Client => {
                            //println!("Client message...");
                        }
                        TwitchIrcMessageType::Message(message) => {

                            if begins_with(&message.get_message_body(), "!") {
                                let a =
                                    self.try_execute_command(message.clone(), context_mutex.clone());
                                a.await?;
                            } else {
                                println!("{}'s channel: {} said {}", message.get_target_channel().get_value(), message.get_speaker().get_value(), message.get_message_body());
                            }
                        }
                        TwitchIrcMessageType::JoiningChannel { joiner, channel } => {
                            println!(
                                "({0}'s channel): {1} has JOINED the channel!",
                                channel.get_value(),
                                joiner.get_value()
                            );
                        }
                        TwitchIrcMessageType::LeavingChannel { leaver, channel } => {
                            println!(
                                "({0}'s channel): {1} has LEFT the channel!",
                                channel.get_value(),
                                leaver.get_value()
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl DefaultMessageParser {
    pub fn get_user_command(&self, key: &str) -> Option<&RereferenceableChatCommand> {
        self.user_commands.get(key)
    }

    pub async fn run_user_command(
        &self,
        key: &str,
        message: TwitchIrcUserMessage,
        args: Vec<String>,
        context: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> Result<(), Box<dyn SendError>> {
        match self.user_commands.get(key) {
            None => println!("Could not find command '{}'", key),
            Some(command) => {
                command
                    .lock()
                    .await
                    .run(self.clone(), message, args, context)
                    .await?
            }
        };
        Ok(())
    }

    //pub fn get_user_commands(&self) -> HashMap<String, user_command_type!()> { self.user_commands.clone() }

    pub fn get_user_commands_including_alternates(&self) -> (UserCommandsMap, UserCommandsMap) {
        (
            self.user_commands.clone(),
            self.user_commands_alternate_keywords.clone(),
        )
    }

    pub fn new() -> DefaultMessageParser {
        let mut new = DefaultMessageParser {
            user_commands: Default::default(),
            user_commands_alternate_keywords: Default::default(),
        };
        new.init_default_commands();
        new
    }

    pub fn init_default_commands(&mut self) {
        self.add_command(
            "addcommand",
            vec!["newcommand"],
            ChatCommand::new(add_custom_text_command),
        );
        self.add_command(
            "commands",
            vec!["allcommands"],
            ChatCommand::new(all_commands),
        );
        self.add_command(
            "blame",
            vec!["scapegoat"],
            ChatCommand::new(blame_random_user),
        );
        self.add_command(
            "flipcoin",
            vec!["5050", "50-50"],
            ChatCommand::new(flipcoin),
        );
        self.add_command("lurk", vec!["afk", "busy"], ChatCommand::new(enter_lurk));
        self.add_command("random", vec!["select"], ChatCommand::new(random_selection));
        self.add_command("shoutout", vec!["shout", "so"], ChatCommand::new(shoutout));
        self.add_command("socials", vec!["social"], ChatCommand::new(socials));
    }

    pub fn add_command(
        &mut self,
        primary_key: &str,
        alternate_keys: Vec<&str>,
        command_ref: ChatCommand,
    ) {
        let arc_cmd = Arc::new(Mutex::new(command_ref));

        if self.user_commands.contains_key(primary_key) {
            println!(
                "WARNING: The irc-command key '{}' is used multiple times.",
                primary_key
            );
        }

        self.user_commands
            .insert(primary_key.to_string(), arc_cmd.clone());

        for alt_key in alternate_keys {
            if self.user_commands.contains_key(alt_key) {
                println!(
                    "WARNING: The irc-command key '{}' is used multiple times.",
                    alt_key
                );
            }

            self.user_commands_alternate_keywords
                .insert(alt_key.to_string(), arc_cmd.clone());
        }
    }

    // decipher for any message returned to our IrcChatSession
    async fn decipher_response_message(
        &self,
        context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> Result<TwitchIrcMessageType, Box<dyn SendError>> {
        let deconstructing_response;

        {
            let context = get_result(context_mutex.try_lock())?;

            if !begins_with(&context.get_initial_response(), ":") {
                return Err(Box::new(KubesError {
                    error: "".to_string(),
                }));
            }

            deconstructing_response = {
                let initial = context.get_initial_response()[1..].to_string();
                let mut temp = String::new();

                // remove duplicate whitespace
                let mut previous_character = ' ';
                for character in initial.chars() {
                    if character == ' ' && previous_character == ' ' {
                        continue;
                    }

                    temp = format!("{0}{1}", temp, character);
                    previous_character = character;
                }

                temp
            };
        }

        let mut first_username_split = deconstructing_response.split("!");

        let potential_username = {
            match first_username_split.next() {
                None => {
                    return Err(Box::new(KubesError {
                        error: "".to_string(),
                    }))
                }
                Some(r) => r,
            }
        };
        let username_duplicate = first_username_split.next(); //?.split("@").next()?;
        if username_duplicate == None {
            let mut client_username_split = deconstructing_response.split(".");

            {
                let context = get_result(context_mutex.try_lock())?;
                if get_option(client_username_split.next(), "NoneError".to_string())?.to_string()
                    != context.get_client_user_data().get_login().get_value()
                {
                    return Err(Box::new(KubesError {
                        error: "".to_string(),
                    }));
                }
            };

            get_option(client_username_split.next(), "NoneError".to_string())?; // tmi
            get_option(client_username_split.next(), "NoneError".to_string())?; // twitch

            // there SHOULDNT be any more periods....

            let response_after_client_name = {
                let temp = get_option(client_username_split.next(), "NoneError".to_string())?;
                if temp.len() < 4 {
                    return Err(Box::new(KubesError {
                        error: "".to_string(),
                    }));
                }

                temp[3..].to_string() // remove 'tv '
            };

            let mut response_whitespace_split = response_after_client_name.split(" ");

            println!(
                "{}",
                match get_option(response_whitespace_split.next(), "NoneError".to_string())? {
                    "353" => {
                        "Is this message only when the client joins? or when anyone joins a channel?"
                    }
                    "366" => {
                        "End of names list.... (list only shows client's name atm) is this only after the client joins a channel?"
                    }
                    _ => {
                        "??? Client Message"
                    }
                }
            );

            return Ok(TwitchIrcMessageType::Client);
        }

        let username_duplicate = get_option(
            get_option(username_duplicate, "NoneError".to_string())?
                .split("@")
                .next(),
            "NoneError".to_string(),
        )?;
        // check if not a usual message (could begin with [client_user].tmi.twitch.tv)
        if potential_username != username_duplicate {
            return Err(Box::new(KubesError {
                error: "".to_string(),
            }));
        }

        let username = UserLogin::new(potential_username.to_string());

        let mut whitespace_split = deconstructing_response.split(" ");
        get_option(whitespace_split.next(), "NoneError".to_string())?;
        let message_type = get_option(whitespace_split.next(), "NoneError".to_string())?;

        let channel_name = {
            let dirty_channel_name = get_option(whitespace_split.next(), "NoneError".to_string())?;
            if dirty_channel_name.len() < 2 {
                return Err(Box::new(KubesError {
                    error: "".to_string(),
                }));
            }
            UserLogin::new(dirty_channel_name[1..].to_string()) // remove pound symbol
        };

        match message_type {
            "PRIVMSG" => {
                let message = {
                    let mut potential_message =
                        get_option(whitespace_split.next(), "NoneError".to_string())?.to_string();
                    if potential_message.len() < 2 {
                        return Err(Box::new(KubesError {
                            error: "Twitch PRIVMSG has an empty body or an invalid format.".to_string(),
                        }));
                    }

                    while let Some(next_word) = whitespace_split.next() {
                        potential_message = format!("{0} {1}", potential_message, next_word);
                    }

                    potential_message[1..].to_string() // remove first space
                };

                Ok(TwitchIrcMessageType::Message(TwitchIrcUserMessage::new(
                    username,
                    message,
                    channel_name,
                )))
            }
            "JOIN" => Ok(TwitchIrcMessageType::JoiningChannel {
                joiner: username,
                channel: channel_name,
            }),
            _ => {
                println!("Could not register IRC message type: {}", message_type);
                return Err(Box::new(KubesError {
                    error: "".to_string(),
                }));
            }
        }
    }

    async fn try_execute_command(
        &self,
        message: TwitchIrcUserMessage,
        context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> Result<(), Box<dyn SendError>> {
        let channel_id;
        let command: String;
        let command_args;

        {
            let context = get_result(context_mutex.try_lock())?;
            channel_id = {
                if message.get_target_channel() != context.get_client_user_data().get_login() {
                    return Err(Box::new(KubesError {
                        error: "".to_string(),
                    }));
                }
                context.get_client_user_data().get_user_id()
            };
        }

        if get_option(
            message.get_message_body().chars().next(),
            "NoneError".to_string(),
        )? != '!'
            || message.get_message_body().len() == 1
        {
            return Err(Box::new(KubesError {
                error: "Twitch - invalid message.".to_string(),
            }));
        }

        let message_body = message.get_message_body();

        // for retrieving command and args
        let mut whitespace_split = message_body[1..].split(" ");

        let string = get_option(whitespace_split.next(), "NoneError".to_string())?.to_lowercase(); // temp to maintain lifetime
        command = string;

        command_args = {
            let mut temp = vec![];
            while let Some(arg) = whitespace_split.next() {
                temp.push(arg.to_string());
            }
            temp
        };

        // try to trigger command
        if let Some(command_func) = self.user_commands.get(command.as_str()) {
            let command_func = command_func.clone();
            command_func
                .lock()
                .await
                .run(
                    DefaultMessageParser::clone(&self),
                    message.clone(),
                    command_args,
                    context_mutex,
                )
                .await?;

            println!(
                "{0} triggered !{1}.",
                message.get_speaker().get_value(),
                command
            );

            return Ok(());
        } else if let Some(command_func) =
            self.user_commands_alternate_keywords.get(command.as_str())
        {
            command_func
                .lock()
                .await
                .run(self.clone(), message.clone(), command_args, context_mutex)
                .await?;

            println!(
                "{0} triggered !{1}.",
                message.get_speaker().get_value(),
                command
            );

            return Ok(());
        }

        // try to trigger custom command
        let custom_commands =
            get_result_dyn(CustomCommandsSaveData::load_or_default(channel_id))?.get_commands();

        if !(custom_commands.contains_key(command.as_str())) {
            return Err(Box::new(KubesError {
                error: "".to_string(),
            }));
        }

        {
            let mut context = get_result(context_mutex.try_lock())?;
            context.add_response_to_reply_with(send_message_from_client_user_format(
                message.get_target_channel().clone(),
                get_option(
                    custom_commands.get(command.as_str()),
                    "NoneError".to_string(),
                )?
                .clone(),
            ));
        }

        println!(
            "{0} triggered !{1}.",
            message.get_speaker().get_value(),
            command
        );

        Ok(())
    }
}
