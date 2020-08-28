use crate::JSON::crawler::ReadingObjects::ReadableType::ReadableType;
use crate::Debug::fail_safely;


pub mod CharacterHandler;
pub mod ReadingObjects;
pub mod Scope;
pub mod JsonPropertyKey;
pub mod JsonPropertyValue;
pub mod JsonObject;
pub mod PropertyType;

pub fn crawl_json(json:&str) -> JsonObject::JsonObject {

    let mut current_scope:Scope::Scope = Scope::Scope::new();
    let mut progress_tracker:String = String::new();
    let mut character_handler:CharacterHandler::CharacterHandler = CharacterHandler::CharacterHandler::new();


    for character in json.chars() {
        let mut tmp:[u8; 4] = [0; 4];
        let char_as_string = character.to_string();
        current_scope.debug_string = format!("{0}{1}", current_scope.debug_string, char_as_string);
        current_scope.set_current_character(&char_as_string);

        progress_tracker = format!("{0}before char- '{1}' current context = MISSING\n", progress_tracker, char_as_string);

        match character {

            '{' => { // new object
                character_handler.register_opening_curly_brace(&mut current_scope);
            },
            '}' => { // end of object
                if current_scope.inside_parentheses {
                    character_handler.register_alphanumeric_character(&mut current_scope);
                } else {
                    if current_scope.current_context == ReadableType::PropertyValuePrimitiveAsString {
                        character_handler.register_closing_quotation_mark(&mut current_scope);
                    }

                    if current_scope.reading_json_objects.current_value_is_root_value() {
                        return character_handler.register_final_closing(&mut current_scope);
                    }

                    character_handler.register_closing_curly_brace(&mut current_scope);
                }
            },
            '\\' => {
                character_handler.register_backslash(&mut current_scope);
            },
            '"' => {
                character_handler.register_quotation_mark(&mut current_scope);
            },
            ':' => {
                if current_scope.inside_parentheses {
                    character_handler.register_alphanumeric_character(&mut current_scope);
                } else {
                    character_handler.register_colon(&mut current_scope);
                }
            },
            '[' => {
                if current_scope.inside_parentheses {
                    character_handler.register_alphanumeric_character(&mut current_scope);
                } else {
                    character_handler.register_opening_bracket(&mut current_scope);
                }
            },
            ']' => {
                if current_scope.inside_parentheses {
                    character_handler.register_alphanumeric_character(&mut current_scope);
                } else {
                    character_handler.register_closing_bracket(&mut current_scope);
                }
            },
            ',' => {
                if current_scope.inside_parentheses {
                    character_handler.register_alphanumeric_character(&mut current_scope);
                } else {
                    character_handler.register_comma(&mut current_scope);
                }
            },
            _ => {
                character_handler.register_alphanumeric_character(&mut current_scope);
            }
        }
    }

    fail_safely("JSON CRAWLER FELL THROUGH!!!!!!!");

    Default::default()
}