use crate::json::crawler::json_object::JsonObject;
use crate::json::crawler::property_type::PropertyType;
use crate::json::crawler::reading_objects::readable_type::ReadableType;
use crate::json::crawler::reading_objects::reading_object::ReadingObject;
use crate::json::crawler::reading_objects::reading_property_key::ReadingPropertyKey;
use crate::json::crawler::reading_objects::reading_property_value::ReadingPropertyValue;
use crate::json::crawler::reading_objects::traits::reading_object::{IReadingObject, IReadingObjectBase};
use crate::json::crawler::scope::Scope;


pub mod character_context_handler;
pub mod closing_curly_brace_handler;
pub mod opening_curly_brace_handler;

// use Scope
// use ReadableType

pub struct CharacterHandler {
    opening_curly_brace_handler: opening_curly_brace_handler::OpeningCurlyBraceHandler,
    closing_curly_brace_handler: closing_curly_brace_handler::ClosingCurlyBraceHandler,
}


impl CharacterHandler {
    pub fn new() -> CharacterHandler {
        CharacterHandler {
            opening_curly_brace_handler: opening_curly_brace_handler::OpeningCurlyBraceHandler {  },
            closing_curly_brace_handler: closing_curly_brace_handler::ClosingCurlyBraceHandler {  }
        }
    }

    pub fn register_opening_curly_brace(&mut self, current_scope:&mut Scope){

        if current_scope.inside_parentheses {
            self.register_alphanumeric_character(current_scope);
            return;
        }


        CharacterHandler::register_character(&mut self.opening_curly_brace_handler, current_scope.get_current_context(), current_scope);


        let empty_func = | _:&mut ReadingObject | { };
        current_scope.reading_json_objects.create_nested_reading_value(current_scope.get_current_context(), empty_func);

        current_scope.set_current_context(ReadableType::PropertyValueObject);
    }

    pub fn register_closing_curly_brace(&mut self, current_scope:&mut Scope) {
        self.try_finalize_primitive_as_string(current_scope);


        let mut reading_object_previous_context = ReadableType::None;
        let func = | reading_object:&mut ReadingObject | {
            reading_object.register_ending_curly_brace();
            reading_object_previous_context = reading_object.get_previous_crawler_context();
        };
        current_scope.reading_json_objects.use_current_reading_value(func);

        CharacterHandler::register_character(&mut self.closing_curly_brace_handler, reading_object_previous_context, current_scope);
    }

    pub fn register_backslash(&mut self, current_scope:&mut Scope) {
        if current_scope.get_previous_character() != "\\"{ return; }


        if !current_scope.previous_character_escaped_via_backslash.clone() {
            self.register_escaped_character(current_scope);
        } else {
            current_scope.previous_character_escaped_via_backslash = false;
        }

    }

    pub fn register_quotation_mark(&mut self, current_scope:&mut Scope) {
        // opening quotation
        current_scope.inside_parentheses = !current_scope.inside_parentheses.clone();
        if current_scope.inside_parentheses {
            self.register_opening_quotation_mark(current_scope);
            return;
        }

        // escaped quotation character
        if current_scope.get_previous_character() == "\\" && ! current_scope.previous_character_escaped_via_backslash.clone() {
            current_scope.inside_parentheses = ! current_scope.inside_parentheses.clone();
            self.register_escaped_character(current_scope);
        }
        // closing quotation
        else {
            self.register_closing_quotation_mark(current_scope);
            current_scope.previous_character_escaped_via_backslash = false;
        }
    }

    // if adding functionality after/before this switch, Property_key must use a GOTO statement to repeat the switch (instead of the whole function)
    pub fn register_opening_quotation_mark(&mut self, current_scope:&mut Scope) {
        match current_scope.get_current_context() {
            ReadableType::RootObject
            | ReadableType::PropertyValueObject => {

                let empty_func = | _:&mut ReadingPropertyKey | { };
                current_scope.reading_json_property_keys.create_nested_reading_value(current_scope.get_current_context(), empty_func);

                current_scope.set_current_context(ReadableType::PropertyKey);
            },
            ReadableType::PropertyKey => {
                let func = | reading_key:&mut ReadingPropertyKey | {
                    reading_key.set_paired_value_type(PropertyType::String);

                    if ! reading_key.is_finalized()
                    {
                        panic!("Property key's json is not finalized when a opening quotation mark was hit.");
                    }
                };
                current_scope.reading_json_property_keys.use_current_reading_value(func);
                
                let set_type = | reading_value:&mut ReadingPropertyValue | {
                    reading_value.value_type = PropertyType::String;
                };
                current_scope.reading_json_property_values.create_nested_reading_value(current_scope.get_current_context(), set_type);

                current_scope.set_current_context( ReadableType::PropertyValueString);
                //self.register_opening_quotation_mark(current_scope);
            },
            ReadableType::PropertyValueUnknownArray => {
                let finalize_key = |reading_key:&mut ReadingPropertyKey | {
                    reading_key.set_paired_value_type(PropertyType::StringVector);

                    if ! reading_key.is_finalized() { panic!("JSON - Property Key is not finalized before an opening quotation mark."); }
                };
                current_scope.reading_json_property_keys.use_current_reading_value(finalize_key);

                let init_value = |reading_value:&mut ReadingPropertyValue | {
                    reading_value.value_type = PropertyType::StringVector;
                };
                current_scope.reading_json_property_values.create_nested_reading_value(current_scope.get_current_context(), init_value);

                current_scope.set_current_context(ReadableType::PropertyValueStringArray);

                self.register_opening_quotation_mark(current_scope);
            },
            ReadableType::PropertyValueStringArray
            |ReadableType::PropertyValueString => {
                let func = | reading_value:&mut ReadingPropertyValue | {
                    reading_value.value_type = PropertyType::String;
                };
                current_scope.reading_json_property_values.create_nested_reading_value(current_scope.get_current_context(), func);
                current_scope.set_current_context(ReadableType::PropertyValueStringArray);
                current_scope.set_current_context(ReadableType::PropertyValueString);
            },
            ReadableType::None
            | ReadableType::PropertyValuePrimitiveAsString
            | ReadableType::PropertyValueObjectArray => { panic!("Opening quotation mark fell through!"); },
        }
    }

    pub fn register_closing_quotation_mark(&mut self, current_scope:&mut Scope) {
        match current_scope.get_current_context() {
            ReadableType::PropertyKey => {
                let func = | reading_key:&mut ReadingPropertyKey | {
                    reading_key.register_ending_quotation_mark_hit();
                };
                current_scope.reading_json_property_keys.use_current_reading_value(func);
            },
            ReadableType::PropertyValueString
            | ReadableType::PropertyValuePrimitiveAsString => {

                let reading_value = {
                    let mut out_reading_value_clone: Option<ReadingPropertyValue> = None;

                    let func = |reading_value: &mut ReadingPropertyValue| {
                        reading_value.closing_quotation_mark_hit();

                        out_reading_value_clone = Some((*reading_value).clone());
                    };
                    current_scope.reading_json_property_values.use_current_reading_value(func);
                    out_reading_value_clone.unwrap()
                };

                if reading_value.get_previous_crawler_context() != ReadableType::PropertyValueStringArray {
                    current_scope.add_current_property_to_current_object();

                    self.set_context_to_last_keys_context(current_scope);

                    return;
                }

                let list_func = | reading_list_value:&mut ReadingPropertyValue | {
                    reading_list_value.add_list_element(reading_value.clone().built_value());
                };
                current_scope.use_current_reading_property_list(PropertyType::StringVector, list_func);

                current_scope.reading_json_property_values.remove_current_value();

                current_scope.set_current_context(ReadableType::PropertyValueStringArray);
            },
            ReadableType::None
            | ReadableType::RootObject
            | ReadableType::PropertyValueUnknownArray
            | ReadableType::PropertyValueObject
            | ReadableType::PropertyValueStringArray
            | ReadableType::PropertyValueObjectArray => { panic!("JSON - Closing quotation mark fell through!"); },

        }
    }

    pub fn register_comma(&mut self, current_scope:&mut Scope) {
        self.try_finalize_primitive_as_string(current_scope);
    }

    pub fn register_colon(&self, current_scope:&mut Scope) {
        if current_scope.get_current_context() != ReadableType::PropertyKey { panic!("RegisterColon switch fell through"); }

        let colon_hit_func = |reading_key:&mut ReadingPropertyKey | {
            reading_key.register_colon_hit();
        };
        current_scope.reading_json_property_keys.use_current_reading_value(colon_hit_func);
    }

    pub fn register_opening_bracket(&self, current_scope:&mut Scope) {
        if current_scope.get_current_context() != ReadableType::PropertyKey { panic!("JSON - Opening bracket registered while not in a PropertyKey context."); }

        current_scope.set_current_context(ReadableType::PropertyValueUnknownArray);
    }

    pub fn register_closing_bracket(&mut self, current_scope:&mut Scope) {
        self.try_finalize_primitive_as_string(current_scope);

        match current_scope.get_current_context() {
            ReadableType::None |
            ReadableType::RootObject |
            ReadableType::PropertyKey |
            ReadableType::PropertyValueString |
            ReadableType::PropertyValuePrimitiveAsString |
            ReadableType::PropertyValueObjectArray => { panic!("JSON - registering closing bracket fell through."); },
            ReadableType::PropertyValueUnknownArray => {
                let finalize_value_func = |reading_array_value:&mut ReadingPropertyValue | {
                    reading_array_value.value_type = PropertyType::EmptyVector;

                    reading_array_value.register_closing_bracket();
                };
                current_scope.reading_json_property_values.create_nested_reading_value(current_scope.get_current_context(), finalize_value_func);

                let finalize_key_func = | reading_key:&mut ReadingPropertyKey | {
                    reading_key.set_paired_value_type(PropertyType::EmptyVector);
                };
                current_scope.reading_json_property_keys.use_current_reading_value(finalize_key_func);

                self.set_context_to_last_keys_context(current_scope);
            },
            ReadableType::PropertyValueObject => {
                let func = |object_to_finalize:&mut ReadingPropertyValue | {
                    object_to_finalize.register_closing_bracket();
                };
                current_scope.use_current_reading_property_list(PropertyType::JsonObjectVector, func);
            },
            ReadableType::PropertyValueStringArray => {
                let func = | string_vector_to_finalize:&mut ReadingPropertyValue | {
                    string_vector_to_finalize.register_closing_bracket();
                };
                current_scope.use_current_reading_property_list(PropertyType::StringVector, func);

                self.set_context_to_last_keys_context(current_scope);
            },
        }

        current_scope.add_current_property_to_current_object();
    }

    pub fn register_final_closing(&self, current_scope:&mut Scope) -> JsonObject {
        if !current_scope.reading_json_property_keys.is_empty()
            || !current_scope.reading_json_property_values.is_empty() { panic!("JSON - Property keys and values have not all been used before final_closing was called."); }

        let mut finalized_root_object:JsonObject = Default::default();

        let finalize = |reading_root_object:&mut ReadingObject | {
            reading_root_object.register_ending_curly_brace();

            finalized_root_object = reading_root_object.built_value();
        };
        current_scope.reading_json_objects.use_current_reading_value(finalize);

        finalized_root_object
    }

    pub fn register_alphanumeric_character(&mut self, current_scope:&mut Scope) {
        current_scope.previous_character_escaped_via_backslash = false;

        match current_scope.get_current_context() {
            ReadableType::PropertyKey => {
                let mut return_from_func:bool = false;
                let current_character = current_scope.get_current_character();
                let key_func = |reading_key:&mut ReadingPropertyKey | {
                    if !reading_key.colon_hit()
                        || !reading_key.string_is_finalized() {

                        reading_key.add_character(current_character);

                        return_from_func = true;
                        return;
                    }

                    if reading_key.is_finalized() { panic!("JSON - triyng to add a character to a property's value while its key is already finalized."); }
                };
                current_scope.reading_json_property_keys.use_current_reading_value(key_func);
                if return_from_func { return; }


                self.register_opening_quotation_mark(current_scope);

                let current_character = current_scope.get_current_character();
                let value_func = |new_reading_value:&mut ReadingPropertyValue | {
                    new_reading_value.add_character(current_character);
                };
                current_scope.reading_json_property_values.use_current_reading_value(value_func);

                current_scope.set_current_context(ReadableType::PropertyValuePrimitiveAsString);
            },
            ReadableType::PropertyValueString
            | ReadableType::PropertyValuePrimitiveAsString => {
                let current_character = current_scope.get_current_character();
                let func = | reading_value:&mut ReadingPropertyValue | {
                    reading_value.add_character(current_character);
                };
                current_scope.reading_json_property_values.use_current_reading_value(func);
            },
            ReadableType::None
            | ReadableType::RootObject
            | ReadableType::PropertyValueUnknownArray
            | ReadableType::PropertyValueObject
            | ReadableType::PropertyValueStringArray
            | ReadableType::PropertyValueObjectArray => { panic!("JSON - register alphanumeric character fell through!\nDebugString: {}", current_scope.debug_string); },
        }
    }

    pub fn register_escaped_character(&mut self, current_scope:&mut Scope) {
        self.register_alphanumeric_character(current_scope);

        current_scope.previous_character_escaped_via_backslash = true;
    }

    pub fn try_finalize_primitive_as_string(&mut self, current_scope:&mut Scope) -> bool {
        if current_scope.get_current_context() != ReadableType::PropertyValuePrimitiveAsString { return false; }

        self.register_closing_quotation_mark(current_scope);
        true
    }

    pub fn set_context_to_last_keys_context(&self, current_scope:&mut Scope) {
        if ! current_scope.reading_json_property_keys.has_reading_value() {
            current_scope.set_current_context(ReadableType::PropertyValueObject);
            return;
        }

        let mut reading_key_previous_context = ReadableType::None;
        let func = | reading_key:&mut ReadingPropertyKey | {
            reading_key_previous_context = reading_key.previous_crawler_context;
        };
        current_scope.reading_json_property_keys.use_current_reading_value(func);
        current_scope.set_current_context(reading_key_previous_context);

    }

    pub fn register_character<T> (character_context_handler:&mut T, context:ReadableType, current_scope:&mut Scope)
        where T : character_context_handler::CharacterContextHandler {
        match context {
            ReadableType::None => {
                character_context_handler.register_none_context(current_scope);
            },
            ReadableType::RootObject => {
                character_context_handler.register_root_object_context(current_scope);
            },
            ReadableType::PropertyKey => {
                character_context_handler.register_property_key_context(current_scope);
            },
            ReadableType::PropertyValueUnknownArray => {
                character_context_handler.register_property_value_unknown_array_context(current_scope);
            },
            ReadableType::PropertyValueString => {
                character_context_handler.register_property_value_string_context(current_scope);
            },
            ReadableType::PropertyValuePrimitiveAsString => {
                character_context_handler.register_property_value_primitive_as_string_context(current_scope);
            },
            ReadableType::PropertyValueObject => {
                character_context_handler.register_property_value_object_context(current_scope);
            },
            ReadableType::PropertyValueStringArray => {
                character_context_handler.register_property_value_string_array_context(current_scope);
            },
            ReadableType::PropertyValueObjectArray => {
                character_context_handler.register_property_value_object_array_context(current_scope);
            }//,
            //_ => { character_context_handler.register_default_context(current_scope); }
        }
    }
}