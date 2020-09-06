use crate::logger::Logger;
use crate::irc::response_context::ResponseContext;
use crate::irc::traits::message_parser::MessageParser;
use crate::json::crawler::crawl_json;
use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::property_type::PropertyType;
use crate::user::user_properties::{UserLogin, UserId};
use crate::irc::parsers::pubsub::event::channel_points_event::reward_signature::{RewardSignature, ImageUrlSignature};
use url::Url;
use std::str::FromStr;
use crate::irc::parsers::pubsub::event::channel_points_event::ChannelPointsEvent;


pub struct DefaultPubSubParser
{

}

impl<TLogger: Clone + Logger> MessageParser<TLogger> for DefaultPubSubParser
{
    fn process_response(&self, context: &mut ResponseContext, _logger: &TLogger) -> bool {

        let json_object = crawl_json(context.get_initial_response().as_str());

        // return if not a sub message
        if json_object.get_string_property_value("type".to_string()) != "MESSAGE" {
            return true;
        }

        let event_outer_wrapper_object =  json_object.get_object_property(JsonPropertyKey::new("data".to_string(), PropertyType::JsonObject));

        let event_topic = event_outer_wrapper_object.get_string_property_value("topic".to_string());

        let client_user_id = context.get_client_user().get_user_id().get_value();

        // channel points event
        if event_topic == format!("channel-points-channel-v1.{}", client_user_id) {
            let event_json_text = event_outer_wrapper_object.get_string_property_value("message".to_string());
            let event_inner_wrapper_object = crawl_json(event_json_text.as_str());
            let event_json_object = event_inner_wrapper_object.get_object_property(JsonPropertyKey::new("data".to_string(), PropertyType::JsonObject));

            let channel_points_event = ChannelPointsEvent::from_json(event_json_object);

            println!("---Channel points event was parsed successfully!");
        }

        //let event_object = json_object.get_object_property(JsonPropertyKey::new("data".to_string(), PropertyType::JsonObject));



        true
    }
}

impl DefaultPubSubParser
{
    pub fn new() -> DefaultPubSubParser {
        DefaultPubSubParser { }
    }


    // init commands fn
}