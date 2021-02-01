use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait SendError: Error + Send {}

#[derive(Clone, Debug)]
pub struct KubesError {
    pub error: String,
}

impl Display for KubesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl Error for KubesError {}

impl SendError for KubesError {}

pub fn get_result<TReturn, TError: Error + 'static>(
    result: Result<TReturn, TError>,
) -> Result<TReturn, Box<dyn SendError>> {
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(to_error(Box::new(e))),
    }
}

pub fn get_result_dyn<TReturn>(
    result: Result<TReturn, Box<dyn Error>>,
) -> Result<TReturn, Box<dyn SendError>> {
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(to_error(e)),
    }
}

pub fn get_option<TReturn>(
    option: Option<TReturn>,
    error: String,
) -> Result<TReturn, Box<dyn SendError>> {
    match option {
        None => Err(Box::new(KubesError { error })),
        Some(r) => Ok(r),
    }
}

pub fn to_error(e: Box<dyn Error>) -> Box<dyn SendError> {
    Box::new(KubesError {
        error: e.to_string(),
    })
}
