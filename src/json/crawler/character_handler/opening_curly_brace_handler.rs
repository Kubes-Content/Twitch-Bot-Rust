use crate::json::crawler::character_handler::character_context_handler::CharacterContextHandler;
use crate::json::crawler::scope::Scope;
use crate::json::crawler::reading_objects::readable_type::ReadableType;
use crate::json::crawler::property_type::PropertyType;
use crate::json::crawler::reading_objects::reading_property_value::ReadingPropertyValue;
use crate::json::crawler::reading_objects::reading_property_key::ReadingPropertyKey;


#[derive(Copy,Clone)]
pub struct OpeningCurlyBraceHandler {
}

impl OpeningCurlyBraceHandler {
    pub fn new() -> OpeningCurlyBraceHandler {
        OpeningCurlyBraceHandler { }
    }
}

impl CharacterContextHandler for OpeningCurlyBraceHandler {

    fn register_none_context(&mut self, current_scope:&mut Scope) {
        current_scope.set_current_context(ReadableType::RootObject);
    }

    fn register_property_key_context(&mut self, current_scope:&mut Scope) {

        let init_object_key = | key:&mut ReadingPropertyKey | {
            key.set_paired_value_type(PropertyType::JsonObject);
        };
        current_scope.reading_json_property_keys.use_current_reading_value(init_object_key);
    }

    fn register_property_value_object_context(&mut self, current_scope:&mut Scope) {
        current_scope.set_current_context(ReadableType::PropertyValueObjectArray);
    }

    fn register_property_value_object_array_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
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
        const VALUE_TYPE:PropertyType = PropertyType::JsonObjectVector;

        let init_key = | key:&mut ReadingPropertyKey| {
            key.set_paired_value_type(VALUE_TYPE);
        };
        current_scope.reading_json_property_keys.use_current_reading_value(init_key);

        let init_value = |prop:&mut ReadingPropertyValue | {
            prop.value_type = VALUE_TYPE;
        };
        current_scope
            .reading_json_property_values
            .create_nested_reading_value(current_scope.get_current_context(), init_value);

        current_scope.set_current_context(ReadableType::PropertyValueObjectArray);
    }

    fn register_root_object_context(&mut self, current_scope:&mut Scope) {
        self.register_default_context(current_scope);
    }

    fn register_default_context(&mut self, _:&mut Scope) {
        assert!(false); // Oh no! Context to register for {nameof(OpeningCurlyBraceHandler)} fell-through!
    }
}