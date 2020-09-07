// use Scope

use crate::json::crawler::scope::Scope;


pub trait CharacterContextHandler {
    //fn get_current_scope(&self) -> Scope;
    //
    fn register_none_context(&mut self, current_scope:&mut Scope);
    fn register_property_key_context(&mut self, current_scope:&mut Scope);
    fn register_property_value_object_context(&mut self, current_scope:&mut Scope);
    fn register_property_value_object_array_context(&mut self, current_scope:&mut Scope);
    fn register_property_value_primitive_as_string_context(&mut self, current_scope:&mut Scope);
    fn register_property_value_string_array_context(&mut self, current_scope:&mut Scope);
    fn register_property_value_string_context(&mut self, current_scope:&mut Scope);
    fn register_property_value_unknown_array_context(&mut self, current_scope:&mut Scope);
    fn register_root_object_context(&mut self, current_scope:&mut Scope);
    fn register_default_context(&mut self, current_scope:&mut Scope);
}