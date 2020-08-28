use crate::Credentials::ClientId::ClientId;
use crate::JSON::crawler::JsonObject::JsonObject;
use chrono::{DateTime, Local};


pub struct ValidationToken {
    pub client_id: ClientId,
    pub scopes: Vec<String>,
    pub expires_in: u32,
    pub requested_time: DateTime<Local>,
}


impl ValidationToken {
    pub fn new(new_id: ClientId, new_scopes: Vec<String>, new_expires_in: u32, new_requested_time: DateTime<Local>) -> ValidationToken {
        ValidationToken {
            client_id: new_id,
            scopes: new_scopes,
            expires_in: new_expires_in,
            requested_time: new_requested_time,
        }
    }

    pub fn from_json(json_object: JsonObject) -> ValidationToken {
        const PROPERTY_NAME_CLIENT_ID: &str = "client_id";
        const PROPERTY_NAME_SCOPES: &str = "scopes";
        const PROPERTY_NAME_EXPIRES_IN: &str = "expires_in";

        let new_id = ClientId::new(json_object.get_string_property_value(PROPERTY_NAME_CLIENT_ID.to_string()).to_string());
        let new_scopes = json_object.get_non_empty_string_vector_property_value(PROPERTY_NAME_SCOPES.to_string());
        let new_expiration = json_object.get_u32_property_value(PROPERTY_NAME_EXPIRES_IN.to_string());
        let new_requested_time = Local::now();

        ValidationToken::new(new_id, new_scopes, new_expiration, new_requested_time)
    }
}