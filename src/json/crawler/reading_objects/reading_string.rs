use crate::json::crawler::reading_objects::readable_type::ReadableType;
use crate::json::crawler::reading_objects::traits::reading_object::{IReadingObject, IReadingObjectBase};


#[derive(Clone)]
pub struct ReadingString {
    building_string:Vec<String>,
    ending_quotation_mark_registered:bool,
    previous_crawler_context:ReadableType,
    is_valid:bool
}

impl Default for ReadingString {
    fn default() -> Self {
        ReadingString::new(ReadableType::None, false)
    }
}

impl IReadingObjectBase for ReadingString {
    fn is_finalized(&self) -> bool {
        self.is_valid()
        && self.ending_quotation_mark_registered
    }

    fn get_previous_crawler_context(&self) -> ReadableType {
        self.previous_crawler_context
    }

    fn set_previous_crawler_context(&mut self, new_type: ReadableType) {
        self.previous_crawler_context = new_type;
    }
}

impl IReadingObject<String> for ReadingString {

    fn built_value(&mut self) -> String {
        let mut built_string:String = "".to_string();

        for character in self.building_string.clone() {
            built_string = format!("{0}{1}", built_string, character);
        }
        built_string
    }
}

impl ReadingString {
    pub fn new(previous_context:ReadableType, new_is_valid:bool) -> ReadingString {
        ReadingString {
            building_string: vec![],
            ending_quotation_mark_registered: false,
            previous_crawler_context: previous_context,
            is_valid: new_is_valid
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn register_ending_quotation_mark(&mut self) {

        if self.ending_quotation_mark_registered {
            panic!("already finalized");
        }

        self.ending_quotation_mark_registered = true;
    }

    pub fn add_character(&mut self, character:String) {
        self.building_string.push(character);
    }
}