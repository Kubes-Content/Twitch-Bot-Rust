use crate::Debug::fail_safely;
use crate::JSON::crawler::JsonObject::JsonObject;
use crate::JSON::crawler::PropertyType::PropertyType;

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
        let new_value_type:PropertyType = PropertyType::String_List;

        if new_value.len() == 0 { unimplemented!(); }

        JsonPropertyValue {
            string_value : Default::default(),
            string_array_value: new_value,
            json_object_value: Default::default(),
            json_object_array_value: Default::default(),
            value_type : new_value_type,
        }
    }

    pub fn new_with_object(new_value:JsonObject) -> JsonPropertyValue {
        let new_value_type:PropertyType = PropertyType::JSON_Object;

        JsonPropertyValue {
            string_value : Default::default(),
            string_array_value: Default::default(),
            json_object_value: new_value,
            json_object_array_value: Default::default(),
            value_type : new_value_type,
        }
    }

    pub fn new_with_object_vector(new_value:Vec<JsonObject>) -> JsonPropertyValue {
        let new_value_type:PropertyType = PropertyType::JSON_Object_List;
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
            fail_safely("Could not get string.");
        }

        println!("WARNING: JSON is allowing null to be returned as an empty string. Find a way to differentiate null and empty strings before this point.");

        self.string_value.clone()
    }

    pub fn get_object_value(&self) -> JsonObject {
        if self.value_type != PropertyType::JSON_Object {
            fail_safely("Could not get JSON object.");
        }

        self.json_object_value.clone()
    }

    pub fn get_object_vector_value(&self) -> Vec<JsonObject> {
        if self.value_type != PropertyType::JSON_Object_List {
            fail_safely("Could not get vector of JSON objects.");
        }

        self.json_object_array_value.clone()
    }

    pub fn get_string_vector_value(&self) -> Vec<String> {
        if self.value_type != PropertyType::String_List {
            fail_safely("Could not get vector of strings.");
        }

        self.string_array_value.clone()
    }

}