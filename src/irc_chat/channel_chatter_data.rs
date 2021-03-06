use reqwest::header::HeaderMap;
use reqwest::Client;

use crate::user::user_properties::UserLogin;
use crate::web_requests::WEB_REQUEST_ATTEMPTS;
use kubes_web_lib::json::crawler::{crawl_json, json_object::JsonObject};
use kubes_web_lib::web_request::{request, RequestType};
use url::Url;
use std::error::Error;


#[allow(dead_code)]
pub struct ChatterData {
    chatter_count: u32,
    broadcaster: Vec<UserLogin>,
    vips: Vec<UserLogin>,
    moderators: Vec<UserLogin>,
    staff: Vec<UserLogin>,
    admins: Vec<UserLogin>,
    global_moderators: Vec<UserLogin>,
    viewers: Vec<UserLogin>, // this includes bots like Anotherttvviewer
}

impl ChatterData {
    fn new(
        chatter_count: u32,
        broadcaster: Vec<UserLogin>,
        vips: Vec<UserLogin>,
        moderators: Vec<UserLogin>,
        staff: Vec<UserLogin>,
        admins: Vec<UserLogin>,
        global_moderators: Vec<UserLogin>,
        viewers: Vec<UserLogin>,
    ) -> ChatterData {
        ChatterData {
            chatter_count,
            broadcaster,
            vips,
            moderators,
            staff,
            admins,
            global_moderators,
            viewers,
        }
    }

    pub fn from_json(json_object: JsonObject) -> ChatterData {
        const CHATTER_COUNT_PROPERTY: &str = "chatter_count";
        const CHATTERS_PROPERTY: &str = "chatters";
        const BROADCASTER_PROPERTY: &str = "broadcaster";
        const VIPS_PROPERTY: &str = "vips";
        const MODS_PROPERTY: &str = "moderators";
        const STAFF_PROPERTY: &str = "staff";
        const ADMINS_PROPERTY: &str = "admins";
        const GLOBAL_MODS_PROPERTY: &str = "global_mods";
        const VIEWERS_PROPERTY: &str = "viewers";

        let chatter_count = json_object.get_u32_property_value(CHATTER_COUNT_PROPERTY.to_string());

        // shadowing
        let json_object = json_object.get_object_property(CHATTERS_PROPERTY.to_string());

        let get_user_logins = |j_o: &JsonObject, key: &str| -> Vec<UserLogin> {
            j_o.get_string_vector_property_value(key.to_string())
                .into_iter()
                .map(|s| UserLogin::from(s))
                .collect()
        };
        let broadcasters = get_user_logins(&json_object, BROADCASTER_PROPERTY);
        let vips = get_user_logins(&json_object, VIPS_PROPERTY);
        let moderators = get_user_logins(&json_object, MODS_PROPERTY);
        let staff = get_user_logins(&json_object, STAFF_PROPERTY);
        let admins = get_user_logins(&json_object, ADMINS_PROPERTY);
        let global_mods = get_user_logins(&json_object, GLOBAL_MODS_PROPERTY);
        let viewers = get_user_logins(&json_object, VIEWERS_PROPERTY);

        ChatterData::new(
            chatter_count,
            broadcasters,
            vips,
            moderators,
            staff,
            admins,
            global_mods,
            viewers,
        )
    }

    pub async fn from_channel(client: &Client, channel: UserLogin) -> Result<ChatterData, Box<dyn Error>> {
        let url = format!(
            "https://tmi.twitch.tv/group/user/{}/chatters",
            channel.get_value()
        );

        let response = request(
            client,
            Url::parse(url.as_str())?,
            RequestType::Get,
            HeaderMap::new(),
            WEB_REQUEST_ATTEMPTS,
        )
        .await?;

        Ok(ChatterData::from_json(crawl_json(response.text().await?.as_str())?))
    }

    pub fn get_mods(&self) -> Vec<UserLogin> {
        self.moderators.clone()
    }

    pub fn get_broadcaster(&self) -> Vec<UserLogin> {
        self.broadcaster.clone()
    }

    pub fn get_all_viewers(&self, include_broadcaster: bool, include_mods: bool) -> Vec<UserLogin> {
        let mut all_viewers: Vec<UserLogin> = Vec::new();

        let mut add_to_viewer_vector = |v: &Vec<UserLogin>| {
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
