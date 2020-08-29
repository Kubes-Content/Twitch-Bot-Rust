use crate::debug::fail_safely;
use crate::json::crawler::reading_objects::traits::reading_object::{IReadingObjectBase, IReadingObject, initialize_interface};
use crate::json::crawler::property_type::PropertyType;
use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::json_property_value::JsonPropertyValue;
use crate::json::crawler::reading_objects::readable_type::ReadableType;
use crate::json::crawler::reading_objects::reading_property_key::ReadingPropertyKey;
use crate::json::crawler::reading_objects::reading_property_value::ReadingPropertyValue;
use crate::json::crawler::reading_objects::reading_object::ReadingObject;
use crate::json::crawler::json_object::JsonObject;


pub struct Scope {
    pub debug_string: String,
    pub current_context: ReadableType,
    previous_char: String,
    current_char: String,
    pub inside_parentheses: bool,
    pub previous_character_escaped_via_backslash: bool,
    pub reading_json_objects: Context<ReadingObject>,
    pub reading_json_property_keys: Context<ReadingPropertyKey>,
    pub reading_json_property_values: Context<ReadingPropertyValue>,
}


pub struct Context<TValue>
    where TValue: IReadingObjectBase{
    current_reading_values: Vec<TValue>
}


impl Scope {
    pub fn new() -> Scope {
        Scope {
            debug_string: String::new(),
            current_context: ReadableType::None,
            previous_char: String::new(),
            current_char: String::new(),
            inside_parentheses: false,
            previous_character_escaped_via_backslash: false,
            reading_json_objects: Context::new(),
            reading_json_property_keys: Context::new(),
            reading_json_property_values: Context::new(),
        }
    }

    pub fn set_current_character(&mut self, new_character: &String) {
        self.previous_char = self.current_char.clone();
        self.current_char = new_character.clone();
    }

    pub fn get_current_character(&self) -> String {
        self.current_char.to_string()
    }

    pub fn get_previous_character(&self) -> String {
        self.previous_char.clone()
    }

    fn try_use_current_reading_property_list<UseFunc>(&mut self, value_type: PropertyType, func:UseFunc) -> bool
        where UseFunc: FnOnce(&mut ReadingPropertyValue) {
        let reading_values = &mut self.reading_json_property_values.current_reading_values;
        for reading_value in reading_values.iter_mut().rev() {
            if reading_value.value_type == value_type {
                func(reading_value);
                return true;
            }
        }
        false
    }

    pub fn use_current_reading_property_list<UseFunc>(&mut self, value_type: PropertyType, func: UseFunc)
    where UseFunc: FnOnce(&mut ReadingPropertyValue) {
        if !self.try_use_current_reading_property_list(value_type, func) {
            fail_safely("ERROR!");
        }
    }

    pub fn finalize_current_object(&mut self) {

        let object_to_be_property = {
            let mut out_object:Option<JsonObject> = None;

            let get_object = | current_reading_object:&mut ReadingObject | {
                out_object = Some(current_reading_object.built_value());
            };
            self.reading_json_objects.use_current_reading_value(get_object);

            self.reading_json_objects.remove_current_value();

            out_object.unwrap()
        };

        let set_object_as_property = |object_as_property_value: &mut ReadingPropertyValue| {
            *object_as_property_value = ReadingPropertyValue::new(PropertyType::JsonObject);


            object_as_property_value.set_object(object_to_be_property);
        };
        self.reading_json_property_values.create_nested_reading_value(self.current_context, set_object_as_property);

        self.add_current_property_to_current_object();
    }

    pub fn add_current_property_to_current_object(&mut self) {

        // get field
        let member_key = {
            let mut out_member_key: Option<JsonPropertyKey> = None;
            {
                let get_built_key = |reading_key: &mut ReadingPropertyKey| {
                    out_member_key = Some(reading_key.built_value());
                };
                self.reading_json_property_keys.use_current_reading_value(get_built_key);
            }
            out_member_key.unwrap()
        };
        let member_value = {
            let mut out_member_value: Option<JsonPropertyValue> = None;
            {
                let get_built_value = |reading_value: &mut ReadingPropertyValue| {
                    out_member_value = Some(reading_value.built_value());
                };
                self.reading_json_property_values.use_current_reading_value(get_built_value);
            }
            out_member_value.unwrap()
        };

        let add_field_to_object = |reading_object:&mut ReadingObject | {
            reading_object.add_property(member_key, member_value);
        };
        self.reading_json_objects.use_current_reading_value(add_field_to_object);

        // remove the added key and value from their reading vectors
        self.reading_json_property_keys.remove_current_value();
        self.reading_json_property_values.remove_current_value();
    }

    pub fn add_current_property_to_current_list(&mut self) {

        let new_value:JsonPropertyValue = {
            let mut temp_value:Option<JsonPropertyValue> = None;

            let get_built_reading_value = | reading_value:&mut ReadingPropertyValue | {
                temp_value = Some(reading_value.built_value());
            };
            self.reading_json_property_values.use_current_reading_value(get_built_reading_value); // is this value removed from the reading vector?

            temp_value.unwrap()
        };
        let list_type: PropertyType = match new_value.value_type {
            PropertyType::JsonObject => PropertyType::JsonObjectVector,
            PropertyType::String => PropertyType::StringVector,
            _ => { fail_safely("Invalid PropertyType"); PropertyType::Invalid },
        };

        // add to reading field-vector
        let func = |reading_list: &mut ReadingPropertyValue| {
            reading_list.add_list_element(new_value);
        };
        self.use_current_reading_property_list(list_type, func);
    }
}


impl<TValue> Context<TValue>
    where TValue: IReadingObjectBase,
          TValue: std::default::Default {
    pub fn new() -> Context<TValue> {
        Context { current_reading_values: vec![] }
    }

    pub fn has_reading_value(&self) -> bool {
        self.current_reading_values.len() > 0
    }

    pub fn get_current_reading_index(&self) -> usize {
        self.current_reading_values.len() - 1
    }

    /*fn try_use_current_reading_value(&mut self, func: fn(&mut TValue)) -> bool {
        if self.current_reading_values.len() == 0 {
            return false;
        }

        self.use_current_reading_value(func);
        true
    }*/

    pub fn use_current_reading_value<UseFunc>(&mut self, func: UseFunc)
        where UseFunc: FnOnce(&mut TValue)
    {
        if self.current_reading_values.len() == 0 {
            fail_safely("No existing reading value to use.");
        }

        let index = self.get_current_reading_index();
        func(&mut self.current_reading_values[index]);
    }

    pub fn create_nested_reading_value<UseFunc>(&mut self, current_scope_context: ReadableType, use_new_value: UseFunc)
        where UseFunc: FnOnce(&mut TValue),
        TValue: IReadingObjectBase
    {
        let mut new_value: TValue = Default::default();
        initialize_interface(&mut new_value, current_scope_context);

        self.current_reading_values.push(new_value);

        self.use_current_reading_value(use_new_value);
    }

    pub fn current_value_is_root_value(&self) -> bool {
        self.current_reading_values.len() == 1
    }

    pub fn is_empty(&self) -> bool {
        self.current_reading_values.len() == 0
    }

    pub fn remove_current_value(&mut self) {
        self.current_reading_values.remove(self.get_current_reading_index() as usize);
    }
}
