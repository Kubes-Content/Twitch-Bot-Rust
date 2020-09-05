use crate::user::user_properties::UserLogin;
use std::collections::HashMap;


pub struct DefaultUserSaveData {
    username:UserLogin,
    // cmd  // full messages
    commands_used:HashMap<String, Vec<String>>
}