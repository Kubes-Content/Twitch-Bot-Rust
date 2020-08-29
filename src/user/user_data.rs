use crate::user::user_properties::{UserId, UserLogin, UserDisplayName, UserType, UserBroadcasterType, UserDescription, UserProfileImageUrlFormat, UserOfflineImageUrlFormat, UserViewCount, UserEmail};
use crate::json::crawler::json_object::JsonObject;
use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::property_type::PropertyType;
use crate::user::oauth_token::OauthToken;
use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use crate::web_requests::{request, get_reqwest_response_text, is_html};
use crate::json::crawler::crawl_json;
use crate::logger::Logger;
use crate::json::crawler::json_property_value::JsonPropertyValue;
use crate::console_components::ConsoleComponents;
use std::str::FromStr;
use crate::{get_input_from_console};


const GET_USERS_URL: &str = "https://api.twitch.tv/helix/users";


pub struct Data {
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


impl Data {
    fn new(new_id: UserId, new_login: UserLogin, new_display_name: UserDisplayName, new_type: UserType, new_broadcaster_type: UserBroadcasterType, new_description: UserDescription, new_profile_image: UserProfileImageUrlFormat, new_offline_image: UserOfflineImageUrlFormat, new_view_count: UserViewCount, new_email: UserEmail) -> Data {
        Data {
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

    pub fn from_json(json_data_object: JsonObject, _logger: &dyn Logger) -> Data {
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

        let json_object_array = json_data_object.get_non_empty_object_array_vector_property(JsonPropertyKey::new("data".to_string(), PropertyType::Invalid));
        let json_object = json_object_array[0].clone();

        let user_id = UserId::new(json_object.get_u32_property_value(PROPERTY_NAME_USER_ID.to_string()));
        let user_login = UserLogin::new(json_object.get_string_property_value(PROPERTY_NAME_LOGIN.to_string()));
        let user_display_name = UserDisplayName::new(json_object.get_string_property_value(PROPERTY_NAME_DISPLAY_NAME.to_string()));
        let user_type = UserType::new_from_string(json_object.get_string_property_value(PROPERTY_NAME_TYPE.to_string()));
        let user_broadcaster_type = UserBroadcasterType::new_from_string(json_object.get_string_property_value(PROPERTY_NAME_BROADCASTER_TYPE.to_string()));
        let user_description = UserDescription::new(json_object.get_string_property_value(PROPERTY_NAME_DESCRIPTION.to_string()));
        let user_profile_url = UserProfileImageUrlFormat::new(json_object.get_string_property_value(PROPERTY_NAME_PROFILE_IMAGE.to_string()));
        let user_offline_url = UserOfflineImageUrlFormat::new(json_object.get_string_property_value(PROPERTY_NAME_OFFLINE_IMAGE.to_string()));
        let user_view_count = UserViewCount::new(json_object.get_u32_property_value(PROPERTY_NAME_VIEW_COUNT.to_string()));
        //
        // try get email // otherwise blank
        let user_email_string: String = {
            let mut out_email_property:JsonPropertyValue = Default::default();
            // REPLACE WITH try_use_property
            if json_object.try_get_property_value_copy(JsonPropertyKey::new(PROPERTY_NAME_EMAIL.to_string(), PropertyType::Invalid), &mut out_email_property) {
                out_email_property.get_string_value()
            } else {
                let debug = format!("Did not acquire user {}'s email address!", user_display_name.to_string());
                println!("{}", debug);
                String::from("")
            }
        };
        let user_email = UserEmail::new(user_email_string);

        Data::new(user_id, user_login, user_display_name, user_type, user_broadcaster_type, user_description, user_profile_url, user_offline_url, user_view_count, user_email)
    }

    async fn get_from_url<'life>(client:&Client, url: &str, components: ConsoleComponents<'life>, web_request_headers: HeaderMap) -> Data {

        let response = {
            let mut out_response:Option<Response> = None;
            let get_response = |new_response:Response| { out_response = Some(new_response); };
            request(client, url, web_request_headers, get_response).await;
            out_response.unwrap()
        };

        let response_text = {
            if ! is_html(&response) {
                get_reqwest_response_text(response).await
            } else {
                open::that(url).unwrap();

                get_input_from_console("Upon authorizing your account, please post the URL of the page you are redirected to into the console to finalize authorization.")
            }
        };

        let json_object = crawl_json(response_text.as_str());

        Data::from_json(json_object, components.logger)

    }

    /*pub async fn request<'life>(client:&Client, components:ConsoleComponents<'life>) -> Result<Data, Error> {
        let implicit_flow_url:String = format!("https://id.twitch.tv/oauth2/authorize?client_id={0}&redirect_uri={1}&response_type=token&scope={2}", CLIENT_ID, REDIRECT_URI, get_all_scopes());

        request_user_oauth_token()

    }*/

    pub async fn get_from_bearer_token<'life>(client:&Client, bearer_token:OauthToken, components:ConsoleComponents<'life>) -> Data {

        let mut header_map = HeaderMap::new();
        let client_header = bearer_token.get_client_id().get_header();
        let header_name = HeaderName::from_str(client_header.get_name().as_str()).unwrap();
        header_map.append(header_name, HeaderValue::from_str(bearer_token.get_client_id().value.as_str()).unwrap());
        let bearer_header = bearer_token.get_oauth_bearer_header();
        let header_name = HeaderName::from_str(bearer_header.get_name().as_str()).unwrap();
        header_map.append(header_name, bearer_header.get_value());

        Data::get_from_url(client, GET_USERS_URL, components, header_map).await
    }

    pub async fn get_from_username<'life>(client:&Client, user_login:UserLogin, components:ConsoleComponents<'life>, headers:HeaderMap) -> Data {
        let url = format!("{0}?login={1}", GET_USERS_URL, user_login.get_value());

        Data::get_from_url(client, url.as_str(), components, headers).await
    }

    pub fn get_login(&self) -> UserLogin {
        self.login.clone()
    }
}