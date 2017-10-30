use serde_db::de::DeserializationError;
use serde_db::ser::SerializationError;
use std::error;
use std::result;
use std::fmt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DESERIALIZATION(DeserializationError),
    SERIALIZATION(SerializationError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DESERIALIZATION(ref e) => e.description(),
            Error::SERIALIZATION(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::DESERIALIZATION(ref e) => Some(e),
            Error::SERIALIZATION(ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DESERIALIZATION(ref e) => write!(fmt, "{}", e),
            Error::SERIALIZATION(ref e) => write!(fmt, "{}", e),
        }
    }
}
