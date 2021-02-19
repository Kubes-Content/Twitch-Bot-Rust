use crate::{
    irc_chat::parsers::pubsub::event::channel_points_event::ChannelPointsEvent,
    user::{
        oauth_token::OauthToken as UserOauthToken,
        user_properties::{ChannelId, UserId},
    },
};
use async_trait::async_trait;
use kubes_web_lib::{
    error::{send_error, send_result, SendResult},
    json::crawler::crawl_json,
};
use std::sync::Arc;
/*
#[allow(dead_code)]
pub struct DefaultPubSubParser {
    channel: ChannelId,
    auth: UserOauthToken,
}

#[async_trait]
impl MessageParser<DefaultPubSubParser> for DefaultPubSubParser {
    async fn process_response(
        &self,
        context_mutex: Arc<tokio::sync::Mutex<ResponseContext>>,
    ) -> SendResult<()> {
        let json_object = {
            let context = send_result::from(context_mutex.try_lock())?;
            send_result::from_dyn(crawl_json(context.message.as_str()))?
        };

        // return if not a sub message
        if json_object.get_string_property_value("type".to_string()) != "MESSAGE" {
            return Ok(());
        }

        let event_outer_wrapper_object = json_object.get_object_property("data".to_string());
        let event_topic = event_outer_wrapper_object.get_string_property_value("topic".to_string());

        let client_user_id = {
            let context = send_result::from(context_mutex.try_lock())?;
            context.parser.channel.get_value().get_value()
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
                    "Error! Pubsub event's topic does not match expected format! Topic: {}",
                    event_topic
                );
                return Err(Box::new(send_error::from_error(e)));
            }
        }

        let blank_error: SendResult<()> = Err(send_error::boxed(""));
        match &event_topic[0..event_topic.len() - UserId::LENGTH - 1] {
            "channel-bits-badge-unlocks" => blank_error,
            "channel-bits-events-v2" => blank_error,
            "channel-commerce-events-v1" => blank_error,
            "channel-points-channel-v1" => {
                let event_json_text =
                    event_outer_wrapper_object.get_string_property_value("message".to_string());
                let event_inner_wrapper_object =
                    send_result::from_dyn(crawl_json(event_json_text.as_str()))?;
                let event_json_object =
                    event_inner_wrapper_object.get_object_property("data".to_string());

                let _channel_points_event = ChannelPointsEvent::from_json(event_json_object);

                println!("---Channel points event was parsed successfully!");

                Ok(())
            }
            "channel-subscribe-events-v1" => blank_error,
            "whispers" => blank_error,
            _ => blank_error,
        }
    }
}

impl DefaultPubSubParser {
    pub fn new(channel: ChannelId, token: UserOauthToken) -> DefaultPubSubParser {
        DefaultPubSubParser {
            channel,
            auth: token,
        }
    }

    // init commands fn
}
*/
