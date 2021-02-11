use std::str::FromStr;

use url::Url;

use crate::irc_chat::parsers::pubsub::event::channel_points_event::reward_signature::{
    ImageUrlSignature, RewardSignature,
};
use crate::user::user_properties::{UserId, UserLogin};
use kubes_web_lib::json::crawler::json_object::JsonObject;

pub mod reward_signature;

#[allow(dead_code)]
pub struct ChannelPointsEvent {
    timestamp: String,
    id: String,
    user: (UserId, UserLogin, String),
    channel_id: UserId,
    redeemed_at: String,
    reward: RewardSignature,
    user_input: String, // Optional
    fulfilled: bool,
}

impl ChannelPointsEvent {
    pub fn from_json(event_object: JsonObject) -> ChannelPointsEvent {
        let timestamp = event_object.get_string_property_value("timestamp".to_string());

        let redemption_object = event_object.get_object_property("redemption".to_string());
        let redemption_id = redemption_object.get_string_property_value("id".to_string());

        let redemption_user_object = redemption_object.get_object_property("user".to_string());
        let redemption_user_id =
            UserId::from(redemption_user_object.get_u32_property_value("id".to_string()));
        let redemption_user_login =
            UserLogin::from(redemption_user_object.get_string_property_value("login".to_string()));
        let redemption_user_display_name =
            redemption_user_object.get_string_property_value("display_name".to_string());

        // redemption object
        let channel_id =
            UserId::from(redemption_object.get_u32_property_value("channel_id".to_string()));
        let redeemed_at = redemption_object.get_string_property_value("redeemed_at".to_string());

        let reward_object = redemption_object.get_object_property("reward".to_string());
        let reward_id = reward_object.get_string_property_value("id".to_string());
        let reward_channel_id =
            UserId::from(reward_object.get_u32_property_value("channel_id".to_string()));
        let reward_title = reward_object.get_string_property_value("title".to_string());
        let reward_prompt = reward_object.get_string_property_value("prompt".to_string());
        let reward_cost = reward_object.get_u32_property_value("cost".to_string());
        let reward_user_input_required: bool = reward_object
            .get_string_property_value("is_user_input_required".to_string())
            .parse()
            .unwrap();
        let reward_sub_only: bool = reward_object
            .get_string_property_value("is_sub_only".to_string())
            .parse()
            .unwrap();

        let reward_image_object_option =
            reward_object.get_nullable_object_property("image".to_string());
        let reward_image_signature = match reward_image_object_option {
            None => None,
            Some(reward_image_object) => {
                let reward_image_url_1x = Url::from_str(
                    reward_image_object
                        .get_string_property_value("url_1x".to_string())
                        .as_str(),
                )
                .unwrap();
                let reward_image_url_2x = Url::from_str(
                    reward_image_object
                        .get_string_property_value("url_2x".to_string())
                        .as_str(),
                )
                .unwrap();
                let reward_image_url_4x = Url::from_str(
                    reward_image_object
                        .get_string_property_value("url_4x".to_string())
                        .as_str(),
                )
                .unwrap();

                Some(ImageUrlSignature {
                    url_1x: reward_image_url_1x,
                    url_2x: reward_image_url_2x,
                    url_4x: reward_image_url_4x,
                })
            }
        };

        let reward_default_image_object =
            reward_object.get_object_property("default_image".to_string());
        let reward_default_image_url_1x = Url::from_str(
            reward_default_image_object
                .get_string_property_value("url_1x".to_string())
                .as_str(),
        )
        .unwrap();
        let reward_default_image_url_2x = Url::from_str(
            reward_default_image_object
                .get_string_property_value("url_2x".to_string())
                .as_str(),
        )
        .unwrap();
        let reward_default_image_url_4x = Url::from_str(
            reward_default_image_object
                .get_string_property_value("url_4x".to_string())
                .as_str(),
        )
        .unwrap();

        // reward object
        let reward_background_color =
            reward_object.get_string_property_value("background_color".to_string());
        let reward_enabled: bool = reward_object
            .get_string_property_value("is_enabled".to_string())
            .parse()
            .unwrap();
        let reward_paused: bool = reward_object
            .get_string_property_value("is_paused".to_string())
            .parse()
            .unwrap();
        let reward_in_stock: bool = reward_object
            .get_string_property_value("is_in_stock".to_string())
            .parse()
            .unwrap();

        let reward_max_per_stream_object =
            reward_object.get_object_property("max_per_stream".to_string());
        let reward_max_per_stream_enabled: bool = reward_max_per_stream_object
            .get_string_property_value("is_enabled".to_string())
            .parse()
            .unwrap();
        let reward_max_per_stream_count =
            reward_max_per_stream_object.get_u32_property_value("max_per_stream".to_string());

        // reward object
        let reward_should_redemptions_skip_request_queue: bool = reward_object
            .get_string_property_value("should_redemptions_skip_request_queue".to_string())
            .parse()
            .unwrap();

        // redemption object
        let redemption_user_input =
            redemption_object.get_string_property_value("user_input".to_string());
        let redemption_fulfilled =
            redemption_object.get_string_property_value("status".to_string()) == "FULFILLED";

        let reward_signature = RewardSignature {
            id: reward_id,
            channel_id: reward_channel_id,
            title: reward_title,
            prompt: reward_prompt,
            cost: reward_cost,
            user_input_required: reward_user_input_required,
            sub_only: reward_sub_only,
            image: reward_image_signature,
            default_image: ImageUrlSignature {
                url_1x: reward_default_image_url_1x,
                url_2x: reward_default_image_url_2x,
                url_4x: reward_default_image_url_4x,
            },
            background_color: reward_background_color,
            enabled: reward_enabled,
            paused: reward_paused,
            in_stock: reward_in_stock,
            max_per_stream: (reward_max_per_stream_enabled, reward_max_per_stream_count),
            should_redemptions_skip_request_queue: reward_should_redemptions_skip_request_queue,
        };

        ChannelPointsEvent {
            timestamp,
            id: redemption_id,
            user: (
                redemption_user_id,
                redemption_user_login,
                redemption_user_display_name,
            ),
            channel_id,
            redeemed_at,
            reward: reward_signature,
            user_input: redemption_user_input,
            fulfilled: redemption_fulfilled,
        }
    }
}
