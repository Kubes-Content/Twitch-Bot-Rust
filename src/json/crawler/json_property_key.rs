use crate::json::crawler::property_type::PropertyType;
use std::hash::{Hash, Hasher};


#[derive(Clone)]
pub struct JsonPropertyKey {
    key_value:String,
    pub paired_property_type:PropertyType
}

impl Hash for JsonPropertyKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key_value.hash(state)
    }
}

impl Eq for JsonPropertyKey {}

impl PartialEq for JsonPropertyKey {
    fn eq(&self, other: &Self) -> bool { self.key_value == other.key_value }
}

impl JsonPropertyKey {
    pub fn new(new_key:String, new_type:PropertyType) -> JsonPropertyKey {
        JsonPropertyKey { key_value : new_key, paired_property_type : new_type }
    }

    pub fn get_value(&self) -> String { self.key_value.clone() }
}

