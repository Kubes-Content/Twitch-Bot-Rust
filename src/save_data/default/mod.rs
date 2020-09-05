macro_rules! serialize_root_object_wrapper {
    ($inside:expr) => {
        format!("{{{}}}", $inside)
    };
}

macro_rules! serialize_object_wrapper {
    ($key:tt, $inside:expr) => {
        format!("\"{0}\":{{{1}}}", $key, $inside);
     };
}

macro_rules! serialize_field_wrapper {
    ($key:tt, $value_as_string:tt) => {
        format!("\"{0}\":\"{1}\"", $key, $value_as_string)
    };
}

pub mod custom_commands_save_data;
pub mod chat_users_data;

pub trait Serializable {
    fn to_json(self) -> String;
    fn from_json(json:String) -> Self;
}



/*pub struct TestSerializable {
    pub int_value:i32,
    pub string_value:String,
    pub bool_value:bool
}


impl Serializable for TestSerializable {
    fn to_json(self) -> String {
        serialize_root_object_wrapper!(
            format!("{0},{1}",
                serialize_field_wrapper!(
                    "int_value",
                    (self.int_value.to_string())
                ),
                serialize_field_wrapper!(
                    "string_value",
                    (self.string_value)
                )
            )
        )
    }

    fn from_json(json: String) -> Self {
        unimplemented!()
    }
}*/