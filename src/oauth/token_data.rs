use chrono::{DateTime, Local};

use crate::credentials::client_id::ClientId;
use crate::json::crawler::json_object::JsonObject;
use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::json_property_value::JsonPropertyValue;
use crate::json::crawler::property_type::PropertyType;
use crate::oauth::signature::Signature;
use crate::oauth::validation_token::ValidationToken;


#[derive(Clone, Hash)]
pub struct TokenData {
    token_type:String,
    scopes:Vec<String>,
    expires_in:u32,
    requested_time:DateTime<Local>,
    signature:Signature,
    client_id:ClientId
}

impl ToString for TokenData {
    fn to_string(&self) -> String {
        format!("TokenData: {0} {1} {2}", self.client_id.to_string(), self.token_type, self.signature.to_string())
    }
}

impl PartialEq for TokenData {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
        && self.scopes == other.scopes
        && self.signature == other.signature
        && self.client_id == other.client_id
        // times do not have to be the same
    }
}

impl TokenData {

    fn new (new_token_type:String, new_scopes:Vec<String>, new_expiration:u32, new_requested_time:DateTime<Local>, new_signature:Signature, new_client_id:ClientId) -> TokenData {
        TokenData {
            token_type: new_token_type,
            scopes: new_scopes,
            expires_in: new_expiration,
            requested_time: new_requested_time,
            signature: new_signature,
            client_id: new_client_id
        }
    }

    pub fn from_json (json_object:JsonObject, client_id:ClientId) -> TokenData {
        const PROPERTY_NAME_SIGNATURE:&str = "access_token";
        const PROPERTY_NAME_REFRESH_TOKEN:&str = "refresh_token";
        const PROPERTY_NAME_EXPIRES_IN:&str = "expires_in";
        const PROPERTY_NAME_SCOPE:&str = "scope";
        const PROPERTY_NAME_TOKEN_TYPE:&str = "token_type";


        let new_signature = Signature::new(json_object.get_string_property_value(PROPERTY_NAME_SIGNATURE.to_string()));

        let _refresh_token = {
            let mut out_refresh_token:Option<JsonPropertyValue> = None; // is this supposed to be in case a
            let get_refresh_token = |value:JsonPropertyValue | {
                out_refresh_token = Some(value);
            };
            json_object.use_property_value(JsonPropertyKey::new(PROPERTY_NAME_REFRESH_TOKEN.to_string(), PropertyType::Invalid), get_refresh_token);
            out_refresh_token.unwrap()
        };


        let new_expiration = json_object.get_u32_property_value(PROPERTY_NAME_EXPIRES_IN.to_string());

        let new_scope = {
            let mut out_scope:Option<Vec<String>> = None;
            let get_scope = | property:JsonPropertyValue | {
                out_scope = Some(property.get_string_vector_value());
            };
            json_object.use_property_value(JsonPropertyKey::new(PROPERTY_NAME_SCOPE.to_string(), PropertyType::Invalid), get_scope);
            out_scope.unwrap()
        };

        let new_token_type = json_object.get_string_property_value(PROPERTY_NAME_TOKEN_TYPE.to_string());

        println!("WARNING - not collecting the 'requested time' or 'ClientID' when building TokenData from JSON. Or the refresh token");

        TokenData::new(new_token_type,new_scope,new_expiration, Local::now(), new_signature, client_id)
    }

    pub fn update(&mut self, validation_token:ValidationToken) {
        if validation_token.expires_in <= 0 { unimplemented!(); }

        self.expires_in = validation_token.expires_in;

        self.client_id = validation_token.client_id;

        self.requested_time = validation_token.requested_time;

        self.scopes = validation_token.scopes;
    }

    pub fn get_signature(&self) -> Signature {
        if self.signature.to_string() == "" {
            panic!("INVALID OAUTH TOKEN SIGNATURE!");
        }

        self.signature.clone()
    }

    pub fn get_client_id(&self) -> ClientId {
        self.client_id.clone()
    }

    pub fn from_str(token:&str, client_id:String) -> TokenData {

        if token.len() != 30 {
            panic!("INVALID TOKEN!");
        }


        TokenData::new(String::from("bearer"), Vec::new(), 0, Local::now(),Signature::new(token.to_string()), ClientId::new(String::from(client_id)))
    }
}