use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum HttpType {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalError = 500,
    UnprocessableEntity = 422,
}

impl Display for HttpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
