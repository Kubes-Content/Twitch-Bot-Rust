use std::error::Error;
use std::fmt::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Errors {
    JsonParseError,
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
