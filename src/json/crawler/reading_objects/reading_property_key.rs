use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::property_type::PropertyType;
use crate::json::crawler::reading_objects::readable_type::ReadableType;
use crate::json::crawler::reading_objects::reading_string::ReadingString;
use crate::json::crawler::reading_objects::traits::reading_object::{IReadingObject, IReadingObjectBase};


pub struct ReadingPropertyKey {
    reading_key_value:ReadingString,
    paired_value_type:PropertyType,
    pub previous_crawler_context:ReadableType,
    colon_hit:bool
}

impl Default for ReadingPropertyKey {
    fn default() -> Self {
        ReadingPropertyKey {
            reading_key_value: ReadingString::new(ReadableType::None, true),
            paired_value_type: PropertyType::Invalid,
            previous_crawler_context: ReadableType::None,
            colon_hit: false
        }
    }
}

impl IReadingObjectBase for ReadingPropertyKey {
    fn is_finalized(&self) -> bool {
        self.reading_key_value.is_finalized()
            && self.paired_value_type != PropertyType::Invalid
            && self.colon_hit
    }

    fn get_previous_crawler_context(&self) -> ReadableType {
        self.previous_crawler_context
    }

    fn set_previous_crawler_context(&mut self, new_type: ReadableType) {
        self.previous_crawler_context = new_type;
    }
}

impl IReadingObject<JsonPropertyKey> for ReadingPropertyKey {
    fn built_value(&mut self) -> JsonPropertyKey {
        JsonPropertyKey::new(self.reading_key_value.built_value(), self.paired_value_type)
    }
}

impl ReadingPropertyKey {
    pub fn register_colon_hit(&mut self) {
        if self.colon_hit {
            panic!("Colon hit twice.");
        }

        self.colon_hit = true;
    }

    pub fn colon_hit(&self) -> bool {
        self.colon_hit
    }

    pub fn register_ending_quotation_mark_hit(&mut self) {
        self.reading_key_value.register_ending_quotation_mark()
    }

    pub fn set_paired_value_type(&mut self, new_value_type:PropertyType) {
        if self.paired_value_type != PropertyType::Invalid {
            panic!("Paired value type set twice.");
        }

        self.paired_value_type = new_value_type;
    }

    pub fn get_paired_value_type(&self) -> PropertyType {
        self.paired_value_type
    }

    pub fn string_is_finalized(&self) -> bool {
        self.reading_key_value.is_finalized()
    }

    pub fn add_character(&mut self, character:String) {
        self.reading_key_value.add_character(character);
    }
}