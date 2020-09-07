use std::collections::HashMap;
use std::str::FromStr;

use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::json_property_value::JsonPropertyValue;
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
            panic!("Invalid property key.");
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
            panic!("failed to get JSON-Object's property's value");
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

    pub fn get_string_vector_property_value(&self, key:String) -> Vec<String> {
        let property = self.get_property_value_copy(JsonPropertyKey::new(key, PropertyType::Invalid));

        // cannot distinguish object vs. string vector if empty
        if property.value_type == PropertyType::EmptyVector {
            Vec::new()
        } else {
            property.get_string_vector_value()
        }
    }

    pub fn get_non_empty_string_vector_property_value(&self, key:String) -> Vec<String> {
        let mut out_value:Vec<String> = Vec::new();
        if !self.try_get_non_empty_string_vector_property_value(key, &mut out_value) {
            panic!("AHHHH!!! REAL MONSTERS!");
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
        if ! self.try_get_non_empty_object_array_vector_property(key.clone(), &mut out_value) { panic!("Property not found! Key = '{}'", key.clone().get_value()); }


        out_value.clone()
    }

    pub fn get_object_property(&self, key: String) -> JsonObject {
        self.get_property_value_copy(JsonPropertyKey::new(key, PropertyType::JsonObject)).get_object_value()
    }

    pub fn get_nullable_object_property(&self, key:String) -> Option<JsonObject> {
        self.get_property_value_copy(JsonPropertyKey::new(key, PropertyType::Invalid)).get_nullable_object_value()
    }

    pub fn use_all_key_value_pairs<UseKVPFunc>(&self, mut func:UseKVPFunc)
        where UseKVPFunc : FnMut(JsonPropertyKey, JsonPropertyValue) {
        for (key, value) in self.properties.clone() {
            func(key, value);
        }
    }
}