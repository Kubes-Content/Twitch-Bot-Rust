use crate::irc_chat::parsers::pubsub::event::channel_points_event::ChannelPointsEvent;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::traits::message_parser::MessageParser;
use crate::json::crawler::crawl_json;
use crate::logger::Logger;
use crate::user::user_properties::UserId;


pub struct DefaultPubSubParser
{

}

impl<TLogger: Clone + Logger> MessageParser<TLogger> for DefaultPubSubParser
{
    fn process_response(&self, context: &mut ResponseContext, logger: &TLogger) -> bool {

        let json_object = crawl_json(context.get_initial_response().as_str());

        // return if not a sub message
        if json_object.get_string_property_value("type".to_string()) != "MESSAGE" { return true; }


        let event_outer_wrapper_object =  json_object.get_object_property("data".to_string());
        let event_topic = event_outer_wrapper_object.get_string_property_value("topic".to_string());

        let client_user_id = context.get_client_user().get_user_id().get_value();

        match event_topic[event_topic.len()-UserId::LENGTH..event_topic.len()].parse::<u32>() {
            // prevent triggering in channels other than the client's
            Ok(event_channel_id) => { if event_channel_id != client_user_id { return true; } },
            // unexpected signature
            Err(_) => { logger.write_line(format!("Pubsub event's topic does not match expected format! Topic: {}", event_topic)); return false; },
        }

        match &event_topic[0..event_topic.len()-UserId::LENGTH-1] {
            "channel-bits-badge-unlocks" => {
                false
            }
            "channel-bits-events-v2" => {
                false
            }
            "channel-commerce-events-v1" => {
                false
            }
            "channel-points-channel-v1" => {
                let event_json_text = event_outer_wrapper_object.get_string_property_value("message".to_string());
                let event_inner_wrapper_object = crawl_json(event_json_text.as_str());
                let event_json_object = event_inner_wrapper_object.get_object_property("data".to_string());

                let channel_points_event = ChannelPointsEvent::from_json(event_json_object);

                println!("---Channel points event was parsed successfully!");

                true
            }
            "channel-subscribe-events-v1" => {
                false
            }
            "whispers" => {
                false
            }
            _ => {

                false
            }
        }
    }


}

impl DefaultPubSubParser
{
    pub fn new() -> DefaultPubSubParser {
        DefaultPubSubParser { }
    }

    // init commands fn
}