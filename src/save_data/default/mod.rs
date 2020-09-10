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
pub mod user_rpg_stats;

pub trait Serializable {
    fn to_json(self) -> String;
    fn from_json(json:String) -> Self;
}