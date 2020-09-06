use crate::user::oauth_token::OauthToken;


pub struct PubsubListenRequest {
    nonce:String,
    topics:Vec<String>,
    auth_token:OauthToken,
}