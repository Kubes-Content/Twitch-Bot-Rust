use crate::user::oauth_token::OauthToken;

#[allow(dead_code)]
pub struct PubsubListenRequest {
    nonce:String,
    topics:Vec<String>,
    auth_token:OauthToken,
}