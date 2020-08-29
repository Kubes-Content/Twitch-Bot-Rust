use crate::oauth::signature::Signature;
use crate::oauth::validation_token::ValidationToken;


pub trait HasOauthSignature {

    fn get_oauth_signature(&self) -> Signature;

    fn update_oauth(&mut self, validation_token:ValidationToken);
}