use crate::OAuth::Signature::Signature;
use crate::OAuth::ValidationToken::ValidationToken;


pub trait HasOauthSignature {

    fn get_oauth_signature(&self) -> Signature;

    fn update_oauth(&mut self, validation_token:ValidationToken);
}