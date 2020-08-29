use std::collections::HashMap;
use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::json_property_value::JsonPropertyValue;
use crate::debug::fail_safely;
use std::str::FromStr;
use crate::json::crawler::property_type::PropertyType;


#[derive(Clone)]
pub struct JsonObject {
    properties: HashMap<JsonPropertyKey, JsonPropertyValue>,
    is_valid:bool
}

impl std::default::Default for JsonObject {
    fn default() -> Self {
        JsonObject::new(false)
    }
}

impl PartialEq for JsonObject {
    fn eq(&self, other: &Self) -> bool {
        self.is_valid == other.is_valid
            && self.properties == other.properties
    }
}

impl JsonObject {
    pub fn new(new_is_valid:bool) -> JsonObject {
        JsonObject { properties: Default::default(), is_valid: new_is_valid }
    }

    pub fn is_valid(&self) -> bool { self.is_valid.clone() }

    pub fn add_property(&mut self, key: JsonPropertyKey, value: JsonPropertyValue) {
        if key.paired_property_type == PropertyType::Invalid {
            fail_safely("Invalid property key.");
        }

        self.properties.insert(key, value);
    }

    pub fn try_get_property_value_copy(&self, key: JsonPropertyKey, out_value: &mut JsonPropertyValue) -> bool {
        if !self.properties.contains_key(&key) {
            return false;
        }

        *out_value = self.properties[&key].clone();
        true
    }

    pub fn get_property_value_copy(&self, key: JsonPropertyKey) -> JsonPropertyValue {
        let mut return_value: JsonPropertyValue = JsonPropertyValue::new_with_string(String::from("ERROR"));
        if !self.try_get_property_value_copy(key, &mut return_value) {
            fail_safely("failed to get JSON-Object's property's value");
        }

        return_value
    }

    pub fn use_property_value<UseFunc>(&self, key: JsonPropertyKey, func: UseFunc)
        where UseFunc: FnOnce(JsonPropertyValue) {
        func((*self.properties.get(&key).unwrap()).clone());
    }

    pub fn get_u32_property_value(&self, key: String) -> u32 {
        u32::from_str(self.get_property_value_copy(JsonPropertyKey::new(key, PropertyType::Invalid))
                          .get_string_value().as_str())
            .unwrap()
    }

    pub fn get_string_property_value(&self, key:String) -> String {
        self.get_property_value_copy(JsonPropertyKey::new(key, PropertyType::Invalid))
            .get_string_value()
    }

    fn try_get_non_empty_string_vector_property_value(&self, key: String, out_value: &mut Vec<String>) -> bool {
        let property = self.get_property_value_copy(JsonPropertyKey::new(key, PropertyType::Invalid));

        if property.value_type == PropertyType::EmptyVector {
            *out_value = Vec::new();
            return false;
        }

        *out_value = property.get_string_vector_value();
        true
    }

    pub fn get_non_empty_string_vector_property_value(&self, key:String) -> Vec<String> {
        let mut out_value:Vec<String> = Vec::new();
        if !self.try_get_non_empty_string_vector_property_value(key, &mut out_value) {
            fail_safely("AHHHH!!! REAL MONSTERS!");
        }

        out_value
    }

    pub fn try_get_non_empty_object_array_vector_property(&self, key: JsonPropertyKey, out_value: &mut Vec<JsonObject>) -> bool {
        let property = self.get_property_value_copy(key);

        if property.value_type == PropertyType::EmptyVector {
            *out_value = Vec::new();
            return false;
        }

        *out_value = property.get_object_vector_value();
        true
    }

    pub fn get_non_empty_object_array_vector_property(&self, key: JsonPropertyKey) -> Vec<JsonObject> {
        let mut out_value = Default::default();
        if ! self.try_get_non_empty_object_array_vector_property(key, &mut out_value) { fail_safely(stringify!(format!("Property not found! Key = '{}'", key))); }


        out_value.clone()
    }

    pub fn get_object_property(&self, key: JsonPropertyKey) -> JsonObject {
        self.get_property_value_copy(key)
            .get_object_value()
    }
}