use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;

use crate::user::oauth_token::OauthToken;
use crate::user::user_properties::{
    UserBroadcasterType, UserDescription, UserDisplayName, UserEmail, UserId, UserLogin,
    UserOfflineImageUrlFormat, UserProfileImageUrlFormat, UserType, UserViewCount,
};
use crate::web_requests::twitch::{request_data, TwitchRequestResponse};
use kubes_std_lib::logging::Logger;
use kubes_web_lib::error::send_error;
use kubes_web_lib::json::crawler::{
    crawl_json, json_object::JsonObject, json_property_key::JsonPropertyKey,
    json_property_value::JsonPropertyValue, property_type::PropertyType,
};
use std::error::Error;

const GET_USERS_URL: &str = "https://api.twitch.tv/helix/users";

#[derive(Clone)]
pub struct UserData {
    id: UserId,
    login: UserLogin,
    display_name: UserDisplayName,
    user_type: UserType,
    broadcaster_type: UserBroadcasterType,
    description: UserDescription,
    profile_image_url_format: UserProfileImageUrlFormat,
    offline_image_url_format: UserOfflineImageUrlFormat,
    view_count: UserViewCount,
    email: UserEmail,
}

impl UserData {
    fn new(
        new_id: UserId,
        new_login: UserLogin,
        new_display_name: UserDisplayName,
        new_type: UserType,
        new_broadcaster_type: UserBroadcasterType,
        new_description: UserDescription,
        new_profile_image: UserProfileImageUrlFormat,
        new_offline_image: UserOfflineImageUrlFormat,
        new_view_count: UserViewCount,
        new_email: UserEmail,
    ) -> UserData {
        UserData {
            id: new_id,
            login: new_login,
            display_name: new_display_name,
            user_type: new_type,
            broadcaster_type: new_broadcaster_type,
            description: new_description,
            profile_image_url_format: new_profile_image,
            offline_image_url_format: new_offline_image,
            view_count: new_view_count,
            email: new_email,
        }
    }

    pub fn from_json<TLogger>(json_data_object: JsonObject, _logger: &TLogger) -> Vec<UserData>
    where
        TLogger: Logger,
    {
        const PROPERTY_NAME_USER_ID: &str = "id";
        const PROPERTY_NAME_LOGIN: &str = "login";
        const PROPERTY_NAME_DISPLAY_NAME: &str = "display_name";
        const PROPERTY_NAME_TYPE: &str = "type";
        const PROPERTY_NAME_BROADCASTER_TYPE: &str = "broadcaster_type";
        const PROPERTY_NAME_DESCRIPTION: &str = "description";
        const PROPERTY_NAME_PROFILE_IMAGE: &str = "profile_image_url";
        const PROPERTY_NAME_OFFLINE_IMAGE: &str = "offline_image_url";
        const PROPERTY_NAME_VIEW_COUNT: &str = "view_count";
        const PROPERTY_NAME_EMAIL: &str = "email";

        let mut user_data: Vec<UserData> = Vec::new();

        let json_object_array = json_data_object.get_non_empty_object_array_vector_property(
            JsonPropertyKey::new("data".to_string(), PropertyType::Invalid),
        );

        for json_object in json_object_array {
            let user_id =
                UserId::from(json_object.get_u32_property_value(PROPERTY_NAME_USER_ID.to_string()));
            let user_login = UserLogin::from(
                json_object.get_string_property_value(PROPERTY_NAME_LOGIN.to_string()),
            );
            let user_display_name = UserDisplayName::from(
                json_object.get_string_property_value(PROPERTY_NAME_DISPLAY_NAME.to_string()),
            );
            let user_type = UserType::new_from_string(
                json_object.get_string_property_value(PROPERTY_NAME_TYPE.to_string()),
            );
            let user_broadcaster_type = UserBroadcasterType::new_from_string(
                json_object.get_string_property_value(PROPERTY_NAME_BROADCASTER_TYPE.to_string()),
            );
            let user_description = UserDescription::from(
                json_object.get_string_property_value(PROPERTY_NAME_DESCRIPTION.to_string()),
            );
            let user_profile_url = UserProfileImageUrlFormat::from(
                json_object.get_string_property_value(PROPERTY_NAME_PROFILE_IMAGE.to_string()),
            );
            let user_offline_url = UserOfflineImageUrlFormat::from(
                json_object.get_string_property_value(PROPERTY_NAME_OFFLINE_IMAGE.to_string()),
            );
            let user_view_count = UserViewCount::from(
                json_object.get_u32_property_value(PROPERTY_NAME_VIEW_COUNT.to_string()),
            );
            //
            // try get email // otherwise blank
            let user_email_string: String = {
                let mut out_email_property: JsonPropertyValue = Default::default();
                // REPLACE WITH try_use_property
                if json_object.try_get_property_value_copy(
                    JsonPropertyKey::new(PROPERTY_NAME_EMAIL.to_string(), PropertyType::Invalid),
                    &mut out_email_property,
                ) {
                    out_email_property.get_string_value()
                } else {
                    let debug = format!(
                        "Did not acquire user {}'s email address!",
                        user_display_name.to_string()
                    );
                    println!("{}", debug);
                    String::from("")
                }
            };
            let user_email = UserEmail::from(user_email_string);

            user_data.push(UserData::new(
                user_id,
                user_login,
                user_display_name,
                user_type,
                user_broadcaster_type,
                user_description,
                user_profile_url,
                user_offline_url,
                user_view_count,
                user_email,
            ));
        }

        user_data
    }

    pub async fn get_from_bearer_token<TLogger>(
        client: &Client,
        bearer_token: OauthToken,
        logger: &TLogger,
    ) -> Result<UserData, Box<dyn Error>>
    where
        TLogger: Logger,
    {
        let mut header_map = HeaderMap::new();
        let client_header = bearer_token.get_client_id().get_header()?;
        let header_name = HeaderName::from_str(client_header.key.as_str())?;
        header_map.append(
            header_name,
            HeaderValue::from_str(bearer_token.get_client_id().value.as_str())?,
        );
        let bearer_header = bearer_token.get_oauth_bearer_header();
        let header_name = HeaderName::from_str(bearer_header.key.as_str())?;
        header_map.append(header_name, bearer_header.value);

        Ok(UserData::get_from_url(client, GET_USERS_URL, header_map, logger).await?[0].clone())
    }

    pub async fn get_from_username<TLogger: Logger>(
        client: &Client,
        user_login: UserLogin,
        logger: &TLogger,
        headers: HeaderMap,
    ) -> Result<UserData, Box<dyn Error>> {
        let url = format!("{0}?login={1}", GET_USERS_URL, user_login.get_value());

        Ok(UserData::get_from_url(client, url.as_str(), headers, logger).await?[0].clone())
    }

    async fn get_from_url<TLogger>(
        client: &Client,
        url: &str,
        web_request_headers: HeaderMap,
        logger: &TLogger,
    ) -> Result<Vec<UserData>, Box<dyn Error>>
    where
        TLogger: Logger,
    {
        let response_text =  // JSON expected
        match request_data(client, url, web_request_headers, logger).await {

            TwitchRequestResponse::Json { response_text } => response_text,
            _ => return Err(Box::new(send_error::new("JSON EXPECTED")))
        };

        Ok(UserData::from_json(
            crawl_json(response_text.as_str())?,
            logger,
        ))
    }

    pub async fn get_from_usernames<TLogger>(
        client: &Client,
        user_logins: Vec<UserLogin>,
        logger: &TLogger,
        headers: HeaderMap,
    ) -> Result<Vec<UserData>, Box<dyn Error>>
    where
        TLogger: Logger,
    {
        if user_logins.len() == 0 {
            return Ok(Vec::new());
        }

        let url = {
            let mut temp = format!("{0}?", GET_USERS_URL);

            for login in user_logins {
                temp = format!("{0}login={1}&", temp, login.get_value());
            }

            temp[0..temp.len() - 1].to_string() // remove final ampersand
        };

        Self::get_from_url(client, url.as_str(), headers, logger).await
    }

    pub fn get_login(&self) -> UserLogin {
        self.login.clone()
    }

    pub fn get_user_id(&self) -> UserId {
        self.id.clone()
    }
}
