use crate::json::crawler::crawl_json;
use crate::save_data::default::Serializable;
use crate::user::user_properties::UserId;

use colour;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Copy, Clone)]
pub struct UserRpgStats {
    experience: u32,
}

impl Serializable for UserRpgStats {
    fn to_json(self) -> String {
        serialize_root_object_wrapper!(serialize_field_wrapper!(
            "experience",
            (self.experience.to_string())
        ))
    }

    fn from_json(json: String) -> Self {
        let json_object = crawl_json(json.as_str());

        UserRpgStats {
            experience: json_object.get_u32_property_value("experience".to_string()),
        }
    }
}

impl UserRpgStats {
    fn get_filename(channel: UserId, user: UserId) -> String {
        format!(
            "user_rpg_stats{0}-{1}.kubes",
            channel.get_value(),
            user.get_value()
        )
    }

    pub fn load_or_default(channel: UserId, user: UserId) -> UserRpgStats {
        match File::open(Self::get_filename(channel, user)) {
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).unwrap();
                Self::from_json(buffer)
            }
            Err(e) => {
                red_ln!("Could not read save file for user possible Error: {}", e);
                UserRpgStats { experience: 0 }
            }
        }
    }

    pub fn save(&self, channel: UserId, user: UserId) {
        match File::create(Self::get_filename(channel, user)) {
            Ok(mut file) => match file.write(self.to_json().as_bytes()) {
                Ok(_) => {}
                Err(e) => panic!("Unable to save custom commands file. Error: {}", e),
            },
            Err(e) => panic!("Unable to save custom commands file. Error: {}", e),
        }
    }

    pub fn add_experience_points(&mut self, experience_to_add: u32) {
        self.experience += experience_to_add;
    }
}
