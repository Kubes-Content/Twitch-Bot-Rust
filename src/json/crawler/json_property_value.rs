use crate::json::crawler::json_object::JsonObject;
use crate::json::crawler::property_type::PropertyType;

#[derive(Clone)]
pub struct JsonPropertyValue {
    string_value:String,
    string_array_value:Vec<String>,
    json_object_value:JsonObject,
    json_object_array_value:Vec<JsonObject>,
    pub value_type:PropertyType
}

impl std::default::Default for JsonPropertyValue {
    fn default() -> Self {
        JsonPropertyValue::new_with_string("".to_string())
    }
}

impl PartialEq for JsonPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.value_type == other.value_type
        && self.string_value == other.string_value
        && self.string_array_value == other.string_array_value
        && self.json_object_value == other.json_object_value
        && self.json_object_array_value == other.json_object_array_value
    }
}

impl JsonPropertyValue {

    pub fn new_with_string(new_value:String) -> JsonPropertyValue {
        let mut new_value_type:PropertyType = PropertyType::String;
        if new_value == "" { new_value_type = PropertyType::Null; }

        // return
        JsonPropertyValue {
            string_value : new_value,
            string_array_value: Default::default(),
            json_object_value: Default::default(),
            json_object_array_value: Default::default(),
            value_type : new_value_type,
        }
    }

    pub fn new_with_string_vector(new_value:Vec<String>) -> JsonPropertyValue {
        let new_value_type:PropertyType = PropertyType::StringVector;

        if new_value.len() == 0 { unimplemented!(); }

        JsonPropertyValue {
            string_value : Default::default(),
            string_array_value: new_value,
            json_object_value: Default::default(),
            json_object_array_value: Default::default(),
            value_type : new_value_type,
        }
    }

    pub fn new_with_empty_vector() -> JsonPropertyValue {
        let new_value_type:PropertyType = PropertyType::EmptyVector;

        JsonPropertyValue {
            string_value : Default::default(),
            string_array_value: Default::default(),
            json_object_value: Default::default(),
            json_object_array_value: Default::default(),
            value_type : new_value_type,
        }
    }

    pub fn new_with_object(new_value:JsonObject) -> JsonPropertyValue {
        let new_value_type:PropertyType = PropertyType::JsonObject;

        JsonPropertyValue {
            string_value : Default::default(),
            string_array_value: Default::default(),
            json_object_value: new_value,
            json_object_array_value: Default::default(),
            value_type : new_value_type,
        }
    }

    pub fn new_with_object_vector(new_value:Vec<JsonObject>) -> JsonPropertyValue {
        let new_value_type:PropertyType = PropertyType::JsonObjectVector;
        //if new_value == null { new_value_type = PropertyType::Null; }
        //unimplemented!(); // ^^^^^ what would be null?

        JsonPropertyValue {
            string_value : Default::default(),
            string_array_value: Default::default(),
            json_object_value: Default::default(),
            json_object_array_value: new_value,
            value_type : new_value_type,
        }
    }

    pub fn get_string_value(&self) -> String {
        if self.value_type != PropertyType::String
            && self.value_type != PropertyType::Null {
            panic!("Could not get string.");
        }

        println!("WARNING: JSON is allowing null to be returned as an empty string. Find a way to differentiate null and empty strings before this point.");

        self.string_value.clone()
    }

    pub fn get_object_value(&self) -> JsonObject {
        if self.value_type != PropertyType::JsonObject {
            panic!("Could not get JSON object.");
        }

        self.json_object_value.clone()
    }

    pub fn get_nullable_object_value(&self) -> Option<JsonObject> {
        if self.value_type == PropertyType::String && self.get_string_value() == "null" {
            None
        } else {
            Some(self.get_object_value())
        }
    }

    pub fn get_object_vector_value(&self) -> Vec<JsonObject> {
        if self.value_type != PropertyType::JsonObjectVector {
            panic!("Could not get vector of JSON objects.");
        }

        self.json_object_array_value.clone()
    }

    pub fn get_string_vector_value(&self) -> Vec<String> {
        if self.value_type != PropertyType::StringVector {
            panic!("Could not get vector of strings.");
        }

        self.string_array_value.clone()
    }

}