use crate::json::crawler::reading_objects::reading_string::ReadingString;
use crate::json::crawler::json_object::JsonObject;
use crate::json::crawler::property_type::PropertyType;
use crate::json::crawler::reading_objects::traits::reading_object::{IReadingObjectBase, IReadingObject};
use crate::debug::fail_safely;
use crate::json::crawler::json_property_value::JsonPropertyValue;
use crate::json::crawler::reading_objects::readable_type::ReadableType;

#[derive(Clone)]
pub struct ReadingPropertyValue {
    reading_string:ReadingString,
    reading_string_vector:Vec<String>,
    reading_json_object:JsonObject,
    reading_json_object_vector:Vec<JsonObject>,
    closing_bracket_hit:bool,
    pub value_type:PropertyType,
    previous_crawler_context:ReadableType
}

impl Default for ReadingPropertyValue {
    fn default() -> Self {
        ReadingPropertyValue::new(PropertyType::Invalid)
    }
}

impl IReadingObjectBase for ReadingPropertyValue {
    fn is_finalized(&self) -> bool {
        match self.value_type {
            PropertyType::String => { self.reading_string.is_finalized() }
            PropertyType::Invalid => { fail_safely(""); false }
            PropertyType::StringVector => { self.closing_bracket_hit }
            PropertyType::JsonObject => { self.reading_json_object.is_valid() }
            PropertyType::JsonObjectVector => { self.closing_bracket_hit }
            PropertyType::EmptyVector => { true }
            PropertyType::Null => { fail_safely("Fall-through match."); false }
        }
    }

    fn get_previous_crawler_context(&self) -> ReadableType {
        self.previous_crawler_context
    }

    fn set_previous_crawler_context(&mut self, new_type: ReadableType) {
        self.previous_crawler_context = new_type
    }
}


impl IReadingObject<JsonPropertyValue> for ReadingPropertyValue {
    fn built_value(&mut self) -> JsonPropertyValue {
        match self.value_type {
            PropertyType::Invalid => { JsonPropertyValue::new_with_string(self.reading_string.built_value()) }
            PropertyType::String => { JsonPropertyValue::new_with_string(self.reading_string.built_value()) }
            PropertyType::StringVector => { JsonPropertyValue::new_with_string_vector(self.reading_string_vector.clone()) }
            PropertyType::JsonObject => { JsonPropertyValue::new_with_object(self.reading_json_object.clone()) }
            PropertyType::JsonObjectVector => { JsonPropertyValue::new_with_object_vector(self.reading_json_object_vector.clone()) }
            PropertyType::EmptyVector => { JsonPropertyValue::new_with_empty_vector() }
            PropertyType::Null => { JsonPropertyValue::new_with_string(self.reading_string.built_value()) }
        }
    }
}

impl ReadingPropertyValue {
    pub fn new(new_value_type:PropertyType) -> ReadingPropertyValue {
        if new_value_type == PropertyType::Invalid {
            //fail_safely("ERROR! Invalid type.");
        }

        ReadingPropertyValue {
            reading_string: ReadingString::new(ReadableType::None, new_value_type == PropertyType::String),
            reading_string_vector: vec![],
            reading_json_object: JsonObject::new(new_value_type == PropertyType::JsonObject),
            reading_json_object_vector: vec![],
            closing_bracket_hit: false,
            value_type: new_value_type,
            previous_crawler_context: ReadableType::None // should this be a param?
        }
    }

    //fn current_reading_string_vector_index (&self) -> usize { self.reading_string_vector.len() - 1 }

    pub fn add_list_element(&mut self, new_element:JsonPropertyValue) {
        if new_element.value_type == PropertyType::String
            && self.value_type == PropertyType::StringVector {
            self.reading_string_vector.push(new_element.get_string_value());
            return;
        }
        if new_element.value_type == PropertyType::JsonObject
            && self.value_type == PropertyType::JsonObjectVector {
            self.reading_json_object_vector.push(new_element.get_object_value());
            return;
        }

        fail_safely("Trying to add an element of an invalid type.");
    }

    pub fn closing_quotation_mark_hit(&mut self) {
        if self.value_type != PropertyType::String {
            fail_safely("ERROR!");
        }

        self.reading_string.register_ending_quotation_mark();
    }

    pub fn add_character(&mut self, character:String) {
        if self.value_type != PropertyType::String {
            fail_safely("ERROR!");
        }

        self.reading_string.add_character(character);
    }

    pub fn add_string(&mut self, readable_string:ReadingString) {
        self.reading_string_vector.push(readable_string.clone().built_value());
    }

    pub fn set_object(&mut self, new_object:JsonObject) {
        if self.value_type != PropertyType::JsonObject
            || !self.reading_json_object.is_valid()
            || !new_object.is_valid() {
            fail_safely("ERROR!");
        }

        self.reading_json_object = new_object;
    }

    pub fn register_closing_bracket(&mut self) {
        if self.closing_bracket_hit {
            fail_safely("Closing bracket hit twice!");
        }

        self.closing_bracket_hit = true;
    }
}