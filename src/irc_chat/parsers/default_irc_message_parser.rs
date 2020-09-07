use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::string::ToString;

use crate::irc_chat::chat_message_parser::IrcMessageParser;
use crate::irc_chat::commands::add_custom_text_command::add_custom_text_command;
use crate::irc_chat::commands::all_commands::all_commands;
use crate::irc_chat::commands::flipcoin::flipcoin;
use crate::irc_chat::commands::random_selection::random_selection;
use crate::irc_chat::commands::send_message_from_client_user_format;
use crate::irc_chat::commands::shoutout::shoutout;
use crate::irc_chat::commands::socials::socials;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::traits::message_parser::MessageParser;
use crate::irc_chat::twitch_message_type::TwitchIrcMessageType;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::logger::Logger;
use crate::save_data::default::custom_commands_save_data::CustomCommandsSaveData;
use crate::user::user_properties::UserLogin;
use crate::utilities::string_ext::{BeginsWith, Remove};

macro_rules! user_command_type {
    () => { fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger) };
}

#[derive(Clone, Default)]
pub struct DefaultMessageParser<TLogger>
    where TLogger: Logger + Clone {
    user_commands: HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger)>,
    user_commands_alternate_keywords: HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger)>
}

unsafe impl<TLogger> Send for DefaultMessageParser<TLogger>
    where TLogger: Logger + Clone {}

unsafe impl<TLogger> Sync for DefaultMessageParser<TLogger>
    where TLogger: Logger + Clone {}

impl<TLogger> MessageParser<TLogger> for DefaultMessageParser<TLogger>
    where TLogger: Logger + Clone {
    fn process_response(&self, context:&mut ResponseContext, logger:&TLogger) -> bool {

        let response_received = context.get_initial_response().clone();

        let mut deconstructing_response = response_received.clone();

        const TMI_TWITCH:&str = ":tmi.twitch.tv ";

        if response_received.begins_with(TMI_TWITCH) {

            deconstructing_response = deconstructing_response.remove_within(TMI_TWITCH);

            match &deconstructing_response[..3] {
                "001" => {/*Welcome, GLHF*/}
                "002" => {/*Your host is tmi.twitch.tv*/}
                "003" => {/*This server is rather new*/}
                "004" => {/*-*/}
                "372" => {/*You are in a maze of twisty passages, all alike.*/}
                "375" => {/*-*/}
                "376" => {/*>*/}
                "421" => {/* Unknown command */}
                _ => { println!("IRC parser Not aware of Twitch-code {0} for line: {1}", deconstructing_response[..3].to_string(), response_received); }
            }

        } else if response_received.begins_with("PING ") {
            context.add_response_to_reply_with(String::from("PONG :tmi.twitch.tv"));
        } else {

            if let Some(message) = self.decipher_response_message(context, logger) {
                match message {
                    TwitchIrcMessageType::Client => {
                        //println!("Client message...");
                    }
                    TwitchIrcMessageType::Message (message) => {
                        if self.try_execute_command(message, context, logger) {
                            return true;
                        }
                    }
                    TwitchIrcMessageType::JoiningChannel {joiner, channel} => {
                        println!("({0}'s channel): {1} has JOINED the channel!", channel.get_value(), joiner.get_value());
                    }
                    TwitchIrcMessageType::LeavingChannel {leaver, channel} => {
                        println!("({0}'s channel): {1} has LEFT the channel!", channel.get_value(), leaver.get_value());
                    }
                }
            } else {
                println!("IF THIS ISNT A USER MESSAGE.... WTF IS IT??")
            }
        }
        true
    }

}

impl<TLogger> IrcMessageParser<TLogger> for DefaultMessageParser<TLogger>
    where TLogger: Logger + Clone {

    fn get_user_commands(&self) -> HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger)> {
        self.user_commands.clone()
    }

    fn get_user_commands_including_alternates(&self) -> (HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger), RandomState>, HashMap<String, fn(Self, TwitchIrcUserMessage, Vec<String>, &mut ResponseContext, &TLogger), RandomState>) {
        (self.user_commands.clone(), self.user_commands_alternate_keywords.clone())
    }
}

impl<TLogger> DefaultMessageParser<TLogger>
    where TLogger: Logger + Clone {

    pub fn new() ->DefaultMessageParser<TLogger> {
        let mut new = DefaultMessageParser { user_commands: Default::default(), user_commands_alternate_keywords: Default::default() };
        new.init_default_commands();
        new
    }

    pub fn init_default_commands(&mut self) {

        self.add_command("addcommand", vec!["newcommand"], add_custom_text_command);
        self.add_command("commands", vec!["allcommands"], all_commands);
        self.add_command("flipcoin", vec!["5050", "50-50"], flipcoin);
        self.add_command("random", vec!["select"], random_selection);
        self.add_command("shoutout", vec!["so"], shoutout);
        self.add_command("socials", vec!["social"], socials)
    }

    pub fn add_command(&mut self, primary_key:&str, alternate_keys:Vec<&str>, command_ref:user_command_type!()) {
        self.user_commands.insert(primary_key.to_string(), command_ref);

        for alt_key in alternate_keys {
            self.user_commands_alternate_keywords.insert(alt_key.to_string(), command_ref);
        }
    }

    // decipher for any message returned to our IrcChatSession
    fn decipher_response_message(&self, context:&mut ResponseContext, logger:&TLogger) -> Option<TwitchIrcMessageType>{

        if ! context.get_initial_response().begins_with(":") { return None; }

        let deconstructing_response = {
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


        let mut first_username_split = deconstructing_response.split("!");

        let potential_username = first_username_split.next()?;
        let username_duplicate = first_username_split.next();//?.split("@").next()?;
        if username_duplicate == None {
            let mut client_username_split = deconstructing_response.split(".");
            if client_username_split.next()?.to_string() != context.get_client_user().get_login().get_value() { return None; }


            client_username_split.next()?; // tmi
            client_username_split.next()?; // twitch

            // there SHOULDNT be any more periods....

            let response_after_client_name = {
                let temp = client_username_split.next()?;
                if temp.len() < 4 { return None; }

                temp[3..].to_string() // remove 'tv '
            };

            let mut response_whitespace_split = response_after_client_name.split(" ");

            match response_whitespace_split.next()? {
                "353" => { logger.write_line(String::from("Is this message only when the client joins? or when anyone joins a channel?")); }
                "366" => { logger.write_line(String::from("End of names list.... (list only shows client's name atm) is this only after the client joins a channel?")); }
                _ => { logger.write_line(String::from("??? Client Message")); }
            };
            return Some(TwitchIrcMessageType::Client);
        }

        let username_duplicate = username_duplicate.unwrap().split("@").next()?;
        // check if not a usual message (could begin with [client_user].tmi.twitch.tv)
        if potential_username != username_duplicate { return None; }

        let username = UserLogin::new(potential_username.to_string());


        let mut whitespace_split = deconstructing_response.split(" ");
        whitespace_split.next()?;
        let message_type = whitespace_split.next()?;

        let channel_name = {
            let dirty_channel_name = whitespace_split.next()?;
            if dirty_channel_name.len() < 2 { return None; }
            UserLogin::new(dirty_channel_name[1..].to_string()) // remove pound symbol
        };

        match message_type {
            "PRIVMSG" => {
                let message = {
                    let mut potential_message = whitespace_split.next()?.to_string();
                    if potential_message.len() < 2 { return None; }

                    while let Some(next_word) = whitespace_split.next() {
                        potential_message = format!("{0} {1}", potential_message, next_word);
                    }

                    potential_message[1..].to_string() // remove first space
                };

                Some(TwitchIrcMessageType::Message (TwitchIrcUserMessage::new (username, message, channel_name)))
            }
            "JOIN" => {
                Some(TwitchIrcMessageType::JoiningChannel { joiner: username, channel: channel_name })
            }
            _ => {
                logger.write_line(format!("Could not register IRC message type: {}", message_type));
                None
            }
        }
    }

    fn try_execute_command(&self, message:TwitchIrcUserMessage, context:&mut ResponseContext, logger:&TLogger) -> bool {

        let channel_id = {
            if message.get_target_channel() != context.get_client_user().get_login() {
                logger.write_line("TRYING TO EXECUTE COMMAND IN SOMEONE ELSE'S CHANNEL.".to_string());
                return false;
            }
            context.get_client_user().get_user_id()
        };

        if message.get_message_body().chars().next().unwrap() != '!' || message.get_message_body().len() == 1 { return false; }

        let message_body = message.get_message_body();

        // for retrieving command and args
        let mut whitespace_split = message_body[1..].split(" ");

        let string = whitespace_split.next().unwrap().to_lowercase(); // temp to maintain lifetime
        let command = string.as_str();

        let command_args:Vec<String> = {
            let mut temp = vec![];
            while let Some(arg) = whitespace_split.next() {
                temp.push(arg.to_string());
            }
            temp
        };

        // try to trigger command
        if let Some(command_func) = self.user_commands.clone().get(command) {
            command_func(DefaultMessageParser::clone(&self), message.clone(), command_args, context, logger);

            println!("{0} triggered !{1}.", message.get_speaker().get_value(), command);

            return true;
        } else if let Some(command_func) = self.user_commands_alternate_keywords.clone().get(command) {
            command_func(self.clone(), message.clone(), command_args, context, logger);

            println!("{0} triggered !{1}.", message.get_speaker().get_value(), command);

            return true;
        }

        // try to trigger custom command
        let custom_commands = CustomCommandsSaveData::load_or_default(channel_id).get_commands();
        if custom_commands.contains_key(command) {

            context.add_response_to_reply_with(send_message_from_client_user_format(message.get_target_channel().clone(), custom_commands.get(command).unwrap().clone()));

            println!("{0} triggered !{1}.", message.get_speaker().get_value(), command);

            true
        } else {
            false
        }

    }
}

