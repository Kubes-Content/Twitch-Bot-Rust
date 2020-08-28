use crate::JSON::crawler::Scope::Scope;
// use PropertyType

use crate::JSON::crawler::CharacterHandler::CharacterContextHandler::CharacterContextHandler;
use crate::JSON::crawler::PropertyType::PropertyType;
use crate::JSON::crawler::JsonObject::JsonObject;
use crate::JSON::crawler::ReadingObjects::ReadingObject::ReadingObject;
use crate::JSON::crawler::ReadingObjects::IReadingObject::IReadingObject;
use crate::JSON::crawler::ReadingObjects::ReadingPropertyValue::ReadingPropertyValue;
use crate::JSON::crawler::JsonPropertyValue::JsonPropertyValue;


#[derive(Copy,Clone)]
pub struct ClosingCurlyBraceHandler {
}

impl ClosingCurlyBraceHandler {
    pub fn new() -> ClosingCurlyBraceHandler {
        ClosingCurlyBraceHandler { }
    }
}

impl CharacterContextHandler for ClosingCurlyBraceHandler {

    fn register_none_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_property_key_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_property_value_object_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_property_value_object_array_context(&mut self, current_scope:&mut Scope) {


        let built_object = {
            let mut out_built_object:Option<JsonPropertyValue> = None;
            let get_built_object = | reading_object:&mut ReadingObject | {
                out_built_object = Some(JsonPropertyValue::new_with_object(reading_object.built_value()));
            };
            current_scope.reading_json_objects.use_current_reading_value(get_built_object);

            out_built_object.unwrap()
        };

        let get_object_list = | reading_object_list:&mut ReadingPropertyValue | {
            reading_object_list.add_list_element(built_object);
        };
        current_scope.use_current_reading_property_list(PropertyType::JSON_Object_List, get_object_list);


        current_scope.reading_json_objects.remove_current_value();
    }

    fn register_property_value_primitive_as_string_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_property_value_string_array_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_property_value_string_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_property_value_unknown_array_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_root_object_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_default_context(&mut self, current_scope:&mut Scope) {
        current_scope.finalize_current_object();
    }
}