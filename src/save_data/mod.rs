use crate::{save_data::default::custom_commands_save_data::CustomCommandsSaveData,
            user::user_properties::UserLogin};
use std::{fs::File,
          io::Read};
use tokio::io::Error;


pub mod default;

fn get_irc_save_filename(channel:UserLogin) -> String {
    format!("{}_irc_save_data.kubes", channel.get_value())
}

fn get_all_irc_data(channel:UserLogin) -> Result<CustomCommandsSaveData, Error> {
    let mut irc_data_file = File::open(get_irc_save_filename(channel))?;

    let _irc_data_json = {
        let mut string = String::new();
        irc_data_file.read_to_string(&mut string)?;
        string
    };

    unimplemented!()
    //default_save_data::from_json(irc_data_json);
}