use crate::json::crawler::crawl_json;
use crate::save_data::default::Serializable;
use crate::user::user_properties::UserId;

use crate::json::crawler::json_object::JsonObject;
use colour;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Copy, Clone)]
pub struct UserRpgStats {
    experience: u32,
}

impl Serializable for UserRpgStats {
    fn to_json(&self) -> String {
        serialize_root_object_wrapper!(serialize_field_wrapper!(
            "experience",
            (self.experience.to_string())
        ))
    }

    fn from_json(json: String) -> Result<Self, Box<dyn Error>> {
        let json_object = match crawl_json(json.as_str()) {
            Ok(j) => j,
            Err(e) => return Err(e),
        };

        Ok(UserRpgStats {
            experience: json_object.get_u32_property_value("experience".to_string()),
        })
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

    pub fn load_or_default(channel: UserId, user: UserId) -> Result<Self, Box<dyn Error>> {
        match File::open(Self::get_filename(channel, user)) {
            Ok(mut file) => {
                let mut buffer = String::new();
                match file.read_to_string(&mut buffer) {
                    Ok(_) => UserRpgStats::from_json(buffer),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => {
                red_ln!("Could not open save file for user. Potential Error: {}\n UPDATE THIS TO BE AWARE OF THE USUAL Err()-case", e);
                // find out what the error for a missing file is (that should be OK, otherwise Err())
                Ok(UserRpgStats { experience: 0 })
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

    pub fn get_current_level(&self) -> u32 {
        const EXPONENT: f32 = 1.2;
        const MULTIPLIER: f32 = 3.0;
        let mut level: u32 = 1;

        while self.experience > ((level as f32).powf(EXPONENT) * MULTIPLIER) as u32 {
            level += 1;
        }

        level
    }
}
