use crate::oauth::signature::Signature;
use crate::credentials::client_id::ClientId;
use crate::oauth::validation_token::ValidationToken;
use crate::debug::fail_safely;
use crate::json::crawler::json_object::JsonObject;
use crate::json::crawler::json_property_value::JsonPropertyValue;
use crate::json::crawler::json_property_key::JsonPropertyKey;
use crate::json::crawler::property_type::PropertyType;
use chrono::{Local, DateTime};


#[derive(Clone)]
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

    pub fn from_json (json_object:JsonObject) -> TokenData {
        const PROPERTY_NAME_SIGNATURE:&str = "access_token";
        const PROPERTY_NAME_REFRESH_TOKEN:&str = "refresh_token";
        const PROPERTY_NAME_EXPIRES_IN:&str = "expires_in";
        const PROPERTY_NAME_SCOPE:&str = "scope";
        const PROPERTY_NAME_TOKEN_TYPE:&str = "token_type";


        let new_signature = Signature::new(json_object.get_string_property_value(PROPERTY_NAME_SIGNATURE.to_string()));

        let _refresh_token = {
            let mut out_refresh_token:Option<JsonPropertyValue> = None;
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

        println!("WARNING - not collecting the 'requested time' or 'ClientID' when building TokenData from JSON.");

        TokenData::new(new_token_type,new_scope,new_expiration, Local::now(), new_signature, ClientId::new("".to_string()))
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
            fail_safely("INVALID OAUTH TOKEN SIGNATURE!");
        }

        self.signature.clone()
    }

    pub fn get_client_id(&self) -> ClientId {
        self.client_id.clone()
    }

    pub fn from_str(token:&str, client_id:String) -> TokenData {

        /*let extract_signature_func = || -> Signature {

            const REDIRECT_PREFIX:&str = "https://twitchapps.com/tokengen/"; // duplicate const?


            let mut access_token:String = String::new();

            let prefix_chars = REDIRECT_PREFIX.as_chars();
            let url_chars = url.as_chars();

            // check url length
            if url_chars.len() <= prefix_chars.len() { fail_safely(stringify!(format!("ERROR url '{0}' is invalid for prefix '{1}'", url, REDIRECT_PREFIX))) }
            // check prefix
            if url_chars[0..prefix_chars.len()] != prefix_chars[0..prefix_chars.len()] { fail_safely(stringify!(format!("ERROR url {0} does not contain the correct prefix {1}", url, REDIRECT_PREFIX))); }

            let mut url_suffix_index_enumerator = Range { start: prefix_chars.len(), end: url_chars.len() };

            /* first character */ {
                let first_char_index = url_suffix_index_enumerator.next();
                if first_char_index == None { fail_safely("(Iterator error) First character doesnt exist?") }

                let first_char_in_prefix = url_chars[url_suffix_index_enumerator.next().unwrap()];
                if first_char_in_prefix != '#' { fail_safely("First character in URL suffix is not a pound sign!"); }


            }

            macro_rules! until_value_hit {
                ($target_value:expr, $current:ident, $do_stuff:expr) => {
                    loop {
                let current_char_index = url_suffix_index_enumerator.next();
                if current_char_index == None { break; }


                let $current = url_chars[current_char_index.unwrap()];
                if $current == $target_value {
                    break;
                }
                $do_stuff
            }
                };
            }

            // step over key name // you could verify the key name here
            until_value_hit!('=', curr, {});


            let mut char_buffer:[u8; 0] = [4; 0];

            // build value
            until_value_hit!('&', current, access_token = access_token + current.encode_utf8(&mut char_buffer));

            let c:char = 'o';




            Signature::new(access_token)
        };*/

        if token.len() != 30 {
            fail_safely("INVALID TOKEN!");
        }


        TokenData::new(String::from("bearer"), Vec::new(), 0, Local::now(),Signature::new(token.to_string()), ClientId::new(String::from(client_id)))
    }
}