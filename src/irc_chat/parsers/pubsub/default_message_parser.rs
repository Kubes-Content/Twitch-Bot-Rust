use crate::send_error::{get_result, get_result_dyn, to_error, KubesError, SendError};
use crate::{
    irc_chat::{
        parsers::pubsub::event::channel_points_event::ChannelPointsEvent,
        response_context::ResponseContext, traits::message_parser::MessageParser,
    },
    user::user_properties::UserId,
};
use async_trait::async_trait;
use kubes_web_lib::json::crawler::crawl_json;
use std::sync::Arc;

pub struct DefaultPubSubParser;

#[async_trait]
impl MessageParser for DefaultPubSubParser {
    async fn process_response(
        &self,
        context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> Result<(), Box<(dyn SendError)>> {
        let json_object = {
            let context = get_result(context_mutex.try_lock())?;
            get_result_dyn(crawl_json(context.get_initial_response().as_str()))?
        };

        // return if not a sub message
        if json_object.get_string_property_value("type".to_string()) != "MESSAGE" {
            return Ok(());
        }

        let event_outer_wrapper_object = json_object.get_object_property("data".to_string());
        let event_topic = event_outer_wrapper_object.get_string_property_value("topic".to_string());

        let client_user_id = {
            let context = get_result(context_mutex.try_lock())?;
            context.get_client_user_data().get_user_id().get_value()
        };

        match event_topic[event_topic.len() - UserId::LENGTH..event_topic.len()].parse::<u32>() {
            // prevent triggering in channels other than the client's
            Ok(event_channel_id) => {
                if event_channel_id != client_user_id {
                    return Ok(());
                }
            }
            // unexpected signature
            Err(e) => {
                println!(
                    "Pubsub event's topic does not match expected format! Topic: {}",
                    event_topic
                );
                return Err(to_error(Box::new(e)));
            }
        }

        match &event_topic[0..event_topic.len() - UserId::LENGTH - 1] {
            "channel-bits-badge-unlocks" => Err(Box::new(KubesError {
                error: String::from(""),
            })),
            "channel-bits-events-v2" => Err(Box::new(KubesError {
                error: String::from(""),
            })),
            "channel-commerce-events-v1" => Err(Box::new(KubesError {
                error: String::from(""),
            })),
            "channel-points-channel-v1" => {
                let event_json_text =
                    event_outer_wrapper_object.get_string_property_value("message".to_string());
                let event_inner_wrapper_object = crawl_json(event_json_text.as_str())
                    .expect("Could not parse pubsub event data!");
                let event_json_object =
                    event_inner_wrapper_object.get_object_property("data".to_string());

                let _channel_points_event = ChannelPointsEvent::from_json(event_json_object);

                println!("---Channel points event was parsed successfully!");

                Ok(())
            }
            "channel-subscribe-events-v1" => Err(Box::new(KubesError {
                error: String::from(""),
            })),
            "whispers" => Err(Box::new(KubesError {
                error: String::from(""),
            })),
            _ => Err(Box::new(KubesError {
                error: String::from(""),
            })),
        }
    }
}

impl DefaultPubSubParser {
    pub fn new() -> DefaultPubSubParser {
        DefaultPubSubParser {}
    }

    // init commands fn
}
