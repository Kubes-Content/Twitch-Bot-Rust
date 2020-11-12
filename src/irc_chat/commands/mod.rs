use crate::user::user_properties::UserLogin;

pub mod add_custom_text_command;
pub mod all_commands;
pub mod blame;
pub mod flipcoin;
pub mod level;
pub mod lurk;
pub mod random_selection;
pub mod shoutout;
pub mod socials;

pub fn send_message_from_client_user_format(channel: UserLogin, message: String) -> String {
    format!("PRIVMSG #{0} :{1}", channel.get_value(), message)
}
