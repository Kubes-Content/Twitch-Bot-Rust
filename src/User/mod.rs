use crate::irc_chat::channel_chatter_data::ChatterData;
use crate::user::user_properties::UserLogin;

pub mod oauth_token;
pub mod user_data;
pub mod user_properties;

pub async fn is_admin_or_mod(target: UserLogin, channel: UserLogin) -> bool {
    if target == channel {
        return true;
    }

    let reqwest_client = reqwest::Client::builder().build().unwrap();
    let chatter_data = ChatterData::from_channel(&reqwest_client, channel).await;
    let mods = chatter_data.get_mods();

    mods.contains(&target)
}
