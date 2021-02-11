use crate::user::user_properties::UserId;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct UserRpgStats {
    experience: u32,
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
        let mut json = String::new();

        File::open(Self::get_filename(channel, user))?.read_to_string(&mut json)?;

        Ok(serde_json::from_str(json.as_str())?)
    }

    pub fn save(&self, channel: UserId, user: UserId) -> Result<(), Box<dyn Error>> {

        File::create(Self::get_filename(channel, user))?.write(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn add_experience_points(&mut self, experience_to_add: u32) {
        self.experience += experience_to_add;
    }

    pub fn get_current_level(&self) -> u32 {
        Self::get_level_from_experience(self.experience)
    }

    pub fn get_level_from_experience(experience: u32) -> u32 {
        const EXPONENT: f32 = 1.2;
        const MULTIPLIER: f32 = 3.0;
        let mut level: u32 = 1;

        while experience >= ((level as f32).powf(EXPONENT) * MULTIPLIER) as u32 {
            level += 1;
        }

        level
    }
}
