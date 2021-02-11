use crate::oauth::validation_token::ValidationToken;
use kubes_web_lib::oauth::Signature;

pub trait HasOauthSignature {
    fn get_oauth_signature(&self) -> Signature;

    fn update_oauth(&mut self, validation_token: ValidationToken);
}
