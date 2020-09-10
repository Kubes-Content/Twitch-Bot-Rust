use reqwest::Client;
use reqwest::header::HeaderMap;

use crate::json::crawler::crawl_json;
use crate::json::crawler::json_object::JsonObject;
use crate::user::user_properties::UserLogin;
use crate::web_requests::request;


pub struct ChatterData {
    chatter_count:u32,
    broadcaster:Vec<UserLogin>,
    vips:Vec<UserLogin>,
    moderators:Vec<UserLogin>,
    staff:Vec<UserLogin>,
    admins:Vec<UserLogin>,
    global_moderators:Vec<UserLogin>,
    viewers:Vec<UserLogin> // this includes bots like Anotherttvviewer
}

impl ChatterData {
    fn new(chatter_count:u32, broadcaster:Vec<UserLogin>, vips:Vec<UserLogin>, moderators:Vec<UserLogin>, staff:Vec<UserLogin>, admins:Vec<UserLogin>, global_moderators:Vec<UserLogin>, viewers:Vec<UserLogin>) -> ChatterData {
        ChatterData{
            chatter_count,
            broadcaster,
            vips,
            moderators,
            staff,
            admins,
            global_moderators,
            viewers
        }
    }

    pub fn from_json(json_object:JsonObject) -> ChatterData {
        const CHATTER_COUNT_PROPERTY:&str = "chatter_count";
        const CHATTERS_PROPERTY:&str = "chatters";
        const BROADCASTER_PROPERTY:&str = "broadcaster";
        const VIPS_PROPERTY:&str = "vips";
        const MODS_PROPERTY:&str = "moderators";
        const STAFF_PROPERTY:&str = "staff";
        const ADMINS_PROPERTY:&str = "admins";
        const GLOBAL_MODS_PROPERTY:&str = "global_mods";
        const VIEWERS_PROPERTY:&str = "viewers";

        let chatter_count = json_object.get_u32_property_value(CHATTER_COUNT_PROPERTY.to_string());

        // shadowing
        let json_object = json_object.get_object_property(CHATTERS_PROPERTY.to_string());

        let broadcaster:Vec<UserLogin> = json_object.get_string_vector_property_value(BROADCASTER_PROPERTY.to_string()).into_iter().map(|s| UserLogin::new(s)).collect();
        let vips:Vec<UserLogin> = json_object.get_string_vector_property_value(VIPS_PROPERTY.to_string()).into_iter().map(|s| UserLogin::new(s)).collect();
        let moderators:Vec<UserLogin> = json_object.get_string_vector_property_value(MODS_PROPERTY.to_string()).into_iter().map(|s| UserLogin::new(s)).collect();
        let staff:Vec<UserLogin> = json_object.get_string_vector_property_value(STAFF_PROPERTY.to_string()).into_iter().map(|s| UserLogin::new(s)).collect();
        let admins:Vec<UserLogin> = json_object.get_string_vector_property_value(ADMINS_PROPERTY.to_string()).into_iter().map(|s| UserLogin::new(s)).collect();
        let global_mods:Vec<UserLogin> = json_object.get_string_vector_property_value(GLOBAL_MODS_PROPERTY.to_string()).into_iter().map(|s| UserLogin::new(s)).collect();
        let viewers:Vec<UserLogin> = json_object.get_string_vector_property_value(VIEWERS_PROPERTY.to_string()).into_iter().map(|s| UserLogin::new(s)).collect();

        ChatterData::new(chatter_count, broadcaster, vips, moderators, staff, admins, global_mods, viewers)
    }

    pub async fn from_channel(client:&Client, channel:UserLogin) -> ChatterData {
        let url = format!("https://tmi.twitch.tv/group/user/{}/chatters", channel.get_value());

        let response = request(client, url.as_str(), HeaderMap::new()).await;

        ChatterData::from_json(crawl_json(response.text().await.unwrap().as_str()))
    }

    pub fn get_mods(&self) -> Vec<UserLogin> {
        self.moderators.clone()
    }

    pub fn get_broadcaster(&self) -> Vec<UserLogin> {
        self.broadcaster.clone()
    }

    pub fn get_all_viewers(&self, include_broadcaster:bool, include_mods:bool) -> Vec<UserLogin> {

        let mut all_viewers:Vec<UserLogin> = Vec::new();

        let mut add_to_viewer_vector = |v:&Vec<UserLogin> | {
            for viewer in v {
                all_viewers.push(viewer.clone());
            }
        };

        if include_broadcaster {
            add_to_viewer_vector(&self.broadcaster);
        }

        if include_mods {
            add_to_viewer_vector(&self.moderators);
        }

        add_to_viewer_vector(&self.vips);
        add_to_viewer_vector(&self.staff);
        add_to_viewer_vector(&self.admins);
        add_to_viewer_vector(&self.global_moderators);
        add_to_viewer_vector(&self.viewers);

        all_viewers
    }
}