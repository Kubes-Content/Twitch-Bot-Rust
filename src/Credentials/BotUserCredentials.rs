use crate::Credentials;
use Credentials::ClientId::ClientId;
use Credentials::ClientSecret::ClientSecret;

pub const REDIRECT_URI:&str = "http://127.0.0.1:7878/";//"https://twitchapps.com/tokengen/";

pub struct BotUserCredentials {
    pub client_id:ClientId,
    pub client_secret:ClientSecret
}

impl BotUserCredentials {

    pub fn new(new_id:ClientId, new_secret:ClientSecret) -> BotUserCredentials {
        return BotUserCredentials { client_id : new_id, client_secret : new_secret };
    }
}