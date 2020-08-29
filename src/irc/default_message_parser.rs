use crate::logger::Logger;
use crate::utilities::string_ext::{BeginsWith, Remove};
use std::collections::HashMap;
use crate::irc::twitch_user_message::TwitchIrcUserMessage;
use std::string::ToString;
use crate::irc::response_context::ResponseContext;
use crate::irc::traits::message_parser::IrcMessageParser;
use crate::irc::twitch_message_type::TwitchIrcMessageType;
use crate::user::user_properties::UserLogin;


pub struct DefaultMessageParser<'life> {
    user_commands:HashMap<String, &'life dyn Fn(TwitchIrcUserMessage, Vec<String>)>
}

impl<'life> IrcMessageParser for DefaultMessageParser<'life> {
    fn process_response(&mut self, context:&mut ResponseContext, logger:&dyn Logger) -> bool {

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
                _ => { println!("IRC parser Not aware of Twitch-code {0} for line: {1}", &deconstructing_response[..3], response_received); }
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
                        //println!("({0}'s channel): {1} said: \"{2}\"", channel.get_value(), speaker.get_value(), message);
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

impl<'life> DefaultMessageParser<'life> {
    pub fn get_commands_map() -> HashMap<String, &'life dyn Fn(TwitchIrcUserMessage, Vec<String>)> {
        let mut commands_map:HashMap<String, &'life dyn Fn(TwitchIrcUserMessage, Vec<String>)> = HashMap::new();
        commands_map.insert("STRING".to_string(), &DefaultMessageParser::test );

        commands_map
    }

    fn test(_msg:TwitchIrcUserMessage, _args:Vec<String>){}

    pub fn new(user_commands:HashMap<String, &'life dyn Fn(TwitchIrcUserMessage, Vec<String>)>) -> DefaultMessageParser<'life> {
        DefaultMessageParser { user_commands }
    }

    // decipher for any message returned to our IrcChatSession
    fn decipher_response_message(&self, context:&mut ResponseContext, logger:&dyn Logger) -> Option<TwitchIrcMessageType> {

        if ! context.get_initial_response().begins_with(":") { return None; }

        let deconstructing_response = context.get_initial_response()[1..].to_string(); // remove colon

        let mut first_username_split = deconstructing_response.split("!");

        let potential_username = first_username_split.next()?;
        let username_duplicate = first_username_split.next();//?.split("@").next()?;
        if username_duplicate == None {
            let mut client_username_split = deconstructing_response.split(".");
            if client_username_split.next()?.to_string() != context.get_client_user().get_value() { return None; }


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
                "353" => { logger.write_line("Is this message only when the client joins? or when anyone joins a channel?"); }
                "366" => { logger.write_line("End of names list.... (list only shows client's name atm) is this only after the client joins a channel?"); }
                _ => { logger.write_line("??? Client Message"); }
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

                    potential_message[1..].to_string()
                };

                Some(TwitchIrcMessageType::Message (TwitchIrcUserMessage::new (username, message, channel_name)))
            }
            "JOIN" => {
                Some(TwitchIrcMessageType::JoiningChannel { joiner: username, channel: channel_name })
            }
            _ => { None }
        }
    }

    fn try_execute_command(&mut self, message:TwitchIrcUserMessage, _context:&mut ResponseContext, _logger:&dyn Logger) -> bool {


        if message.get_message_body().chars().next().unwrap() != '!' { return false; }

        let message_body = message.get_message_body();
        let mut whitespace_split = message_body[1..].split(" ");


        let command = whitespace_split.next().unwrap();
        let mut command_args:Vec<String> = vec![];
        while let Some(arg) = whitespace_split.next() {
            command_args.push(arg.to_string());
        }

        if ! self.user_commands.contains_key(command) { return false; }

        if let Some(command_func) = self.user_commands.get(command) {
            command_func(message.clone(), command_args);

            return true;
        }

        false
    }
}
