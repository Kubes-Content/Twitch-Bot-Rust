use crate::JSON::crawler::JsonObject::JsonObject;
use crate::JSON::crawler::ReadingObjects::ReadableType::ReadableType;
use crate::JSON::crawler::ReadingObjects::IReadingObject::{IReadingObjectBase, IReadingObject};
use crate::Debug::fail_safely;
use crate::JSON::crawler::JsonPropertyKey::JsonPropertyKey;
use crate::JSON::crawler::JsonPropertyValue::JsonPropertyValue;

#[derive(Clone)]
pub struct ReadingObject {
    ending_curly_brace_registered:bool,
    built_value:JsonObject,
    previous_crawler_context:ReadableType
}

impl std::default::Default for ReadingObject {
    fn default() -> Self {
        ReadingObject {
            ending_curly_brace_registered: false,
            built_value: Default::default(),
            previous_crawler_context: ReadableType::None
        }
    }
}

impl IReadingObjectBase for ReadingObject {
    fn is_finalized(&self) -> bool {
        self.ending_curly_brace_registered
    }

    fn get_previous_crawler_context(&self) -> ReadableType {
        self.previous_crawler_context
    }

    fn set_previous_crawler_context(&mut self, new_type: ReadableType) {
        self.previous_crawler_context = new_type;
    }
}

impl IReadingObject<JsonObject> for ReadingObject {
    fn built_value(&mut self) -> JsonObject {
        if !self.is_finalized() { fail_safely("Trying to build a reading JSON object that is not finalized."); }

        self.built_value.clone()
    }
}

impl ReadingObject {
    pub fn register_ending_curly_brace(&mut self) {
        if self.ending_curly_brace_registered {
            fail_safely("Ending curly brace for reading JSON object was registered twice!!");
        }

        self.ending_curly_brace_registered = true;
    }

    pub fn add_property(&mut self, key:JsonPropertyKey, val:JsonPropertyValue) {
        self.built_value.add_property(key, val);
    }
}