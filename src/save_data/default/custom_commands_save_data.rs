use std::collections::HashMap;

use crate::user::user_properties::{ChannelId};
use std::error::Error;
use kubes_std_lib::file::FileName;
use crate::irc_chat::commands::ChatCommandKey;
use kubes_std_lib::file;


cloneable_serializable!(CustomCommandsSaveData, HashMap<ChatCommandKey, String>, String::from("(CustomCommandSaveData)"));

impl CustomCommandsSaveData {

    pub fn load_or_default(channel: ChannelId) -> Result<Self, Box<dyn Error>> {
        let json = file::read(Self::get_filename(channel).get_value())?;
        Ok(serde_json::from_str(json.as_str())?)
    }

    pub fn save(self, channel: ChannelId) -> Result<(), Box<dyn Error>> {
        file::create(Self::get_filename(channel).get_value(), serde_json::to_string(&self)?)?;
        Ok(())
    }

    fn get_filename(channel: ChannelId) -> FileName {
        FileName::from(format!("{}_custom_commands.kubes", channel.get_value().get_value()))
    }

    // does not auto-save file
    pub fn add_command(&mut self, command: ChatCommandKey, body: String) -> String {
        match self.get_value().insert(command.clone(), body) {
            None => format!("Added !{} custom command.", command.get_value()),
            Some(_) => format!("Updated !{} custom command.", command.get_value()),
        }
    }

    pub fn get_commands(self) -> HashMap<ChatCommandKey, String> {
        self.get_value()
    }
}
