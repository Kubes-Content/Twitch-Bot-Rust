use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use crate::{json::crawler::crawl_json};
use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::json_property_value::JsonPropertyValue;
use crate::save_data::default::Serializable;
use crate::user::user_properties::UserId;


#[derive(Default)]
pub struct CustomCommandsSaveData {
    //user_data:HashMap<UserLogin, DefaultUserSaveData>
                            // cmd // chat text sent by bot
    custom_commands:HashMap<String,String>
}

impl Serializable for CustomCommandsSaveData {
    fn to_json(self) -> String {
        serialize_root_object_wrapper!(
            serialize_object_wrapper!("custom_commands",
                {
                    let mut all_entries = String::new();

                    // add each command as a field
                    for (command, text) in self.custom_commands {
                        all_entries = format!("{0}{1},", all_entries, serialize_field_wrapper!(command, text));
                    }

                    // remove last comma
                    if all_entries.len() > 0 {
                        all_entries = all_entries[0..all_entries.len()-1].to_string();
                    }

                    println!("{}", all_entries);

                    all_entries
                }
            )
        )
    }

    fn from_json(json: String) -> Self {
        let json = crawl_json(json.as_str());

        let commands_hashmap_json = json.get_object_property("custom_commands".to_string());

        let custom_commands = {
            let mut hashmap:HashMap<String,String> = HashMap::new();
            //
            let add_command = |key:JsonPropertyKey, value:JsonPropertyValue | {
                hashmap.insert(key.get_value(), value.get_string_value());
            };
            commands_hashmap_json.use_all_key_value_pairs(add_command);

            hashmap
        };

        CustomCommandsSaveData::new(custom_commands)
    }
}

impl CustomCommandsSaveData {
    fn new(custom_commands:HashMap<String,String>) -> CustomCommandsSaveData {
        CustomCommandsSaveData { custom_commands }
    }

    pub fn load_or_default(channel:UserId) -> CustomCommandsSaveData {
        match File::open(Self::get_filename(channel)) {
            Ok(mut file) => {
                let mut json = String::new();
                match file.read_to_string(&mut json) {
                    Ok(_) => {
                        Self::from_json(json)
                    },
                    Err(_) => {
                        Default::default()
                    }
                }
            }
            Err(..) => {
                Default::default()
            }
        }
    }

    pub fn save(self, channel:UserId) {
        match File::create(Self::get_filename(channel)) {
            Ok(mut file) => {
                match file.write(self.to_json().as_bytes()) {
                    Ok(_) => {},
                    Err(e) => {
                        panic!("Unable to save custom commands file. Error: {}", e)
                    },
                }
            },
            Err(e) => {
                panic!("Unable to save custom commands file. Error: {}", e)
            },
        }
    }

    fn get_filename(channel:UserId) -> String {
        format!("{}_custom_commands.kubes", channel.get_value())
    }

    // does not auto-save file
    pub fn add_command(&mut self, command:String, body:String) -> String {
        match self.custom_commands.insert(command.clone(), body) {
            None => { format!("Added !{} custom command.", command) },
            Some(_) => { format!("Updated !{} custom command.", command) },
        }
    }

    pub fn get_commands(self) -> HashMap<String,String> {
        self.custom_commands
    }
}

