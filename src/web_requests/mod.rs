// use callbacks instead of passing in junk
extern crate reqwest;

//use self::reqwest::header::HeaderMap;
pub use kubes_web_lib::web_request::Header;

pub mod twitch;

pub const WEB_REQUEST_ATTEMPTS: u16 = 4;
