#[derive(Copy, Clone, PartialEq)]
pub enum PropertyType {
    Invalid,
    String,
    StringVector,
    JsonObject,
    JsonObjectVector,
    EmptyVector,
    Null
}