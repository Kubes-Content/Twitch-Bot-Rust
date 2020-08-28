#[derive(Copy, Clone, PartialEq)]
pub enum PropertyType {
    Invalid,
    String,
    String_List,
    JSON_Object,
    JSON_Object_List,
    Empty_List,
    Null
}