#[derive(Copy, Clone, PartialEq)]
pub enum ReadableType {
    None,
    RootObject,
    PropertyKey,
    PropertyValueUnknownArray,
    PropertyValueString,
    PropertyValuePrimitiveAsString,
    PropertyValueObject,
    PropertyValueStringArray,
    PropertyValueObjectArray
}