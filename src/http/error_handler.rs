use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum HttpError {
    NotFound,
    BadRequest,
    NotImplemented,
    InternalServer,
    Other(String),
}

impl Error for HttpError {}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::BadRequest => write!(f, "HTTP/1.1 400 Bad Request"),
            HttpError::NotFound => write!(f, "HTTP/1.1 404 Not Found"),
            HttpError::InternalServer => write!(f, "HTTP/1.1 500 Internal Server Error"),
            HttpError::NotImplemented => write!(f, "HTTP/1.1 501 Not Implemented"),
            HttpError::Other(message) => write!(f, "Other error: {}", message),
        }
    }
}

