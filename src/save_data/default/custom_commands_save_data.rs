use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use crate::user::user_properties::UserId;
use std::error::Error;
use serde::{ Deserialize, Serialize };


#[derive(Default, Deserialize, Serialize)]
pub struct CustomCommandsSaveData {
    //user_data:HashMap<UserLogin, DefaultUserSaveData>
    // cmd // chat text sent by bot
    custom_commands: HashMap<String, String>,
}

impl CustomCommandsSaveData {
    fn new(custom_commands: HashMap<String, String>) -> CustomCommandsSaveData {
        CustomCommandsSaveData { custom_commands }
    }

    pub fn load_or_default(channel: UserId) -> Result<Self, Box<dyn Error>> {
        let mut json = String::new();
        let mut file = File::open(Self::get_filename(channel))?;
        file.read_to_string(&mut json)?;
        Ok(serde_json::from_str(json.as_str())?)
    }

    pub fn save(self, channel: UserId) -> Result<(), Box<dyn Error>> {
        File::create(Self::get_filename(channel))?.write(serde_json::to_string(&self)?.as_bytes())?;
        Ok(())
    }

    fn get_filename(channel: UserId) -> String {
        format!("{}_custom_commands.kubes", channel.get_value())
    }

    // does not auto-save file
    pub fn add_command(&mut self, command: String, body: String) -> String {
        match self.custom_commands.insert(command.clone(), body) {
            None => format!("Added !{} custom command.", command),
            Some(_) => format!("Updated !{} custom command.", command),
        }
    }

    pub fn get_commands(self) -> HashMap<String, String> {
        self.custom_commands
    }
}
