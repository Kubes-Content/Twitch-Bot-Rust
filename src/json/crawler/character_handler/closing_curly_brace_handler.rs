use crate::json::crawler::scope::Scope;
// use PropertyType

use crate::json::crawler::character_handler::character_context_handler::CharacterContextHandler;
use crate::json::crawler::property_type::PropertyType;
use crate::json::crawler::reading_objects::reading_object::ReadingObject;
use crate::json::crawler::reading_objects::traits::reading_object::IReadingObject;
use crate::json::crawler::reading_objects::reading_property_value::ReadingPropertyValue;
use crate::json::crawler::json_property_value::JsonPropertyValue;


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
        current_scope.use_current_reading_property_list(PropertyType::JsonObjectVector, get_object_list);


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