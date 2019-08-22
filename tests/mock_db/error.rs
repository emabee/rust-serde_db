use serde_db::{de::DeserializationError, ser::SerializationError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DESERIALIZATION(DeserializationError),
    SERIALIZATION(SerializationError),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DESERIALIZATION(ref e) => e.description(),
            Error::SERIALIZATION(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::DESERIALIZATION(ref e) => Some(e),
            Error::SERIALIZATION(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::DESERIALIZATION(ref e) => write!(fmt, "{}", e),
            Error::SERIALIZATION(ref e) => write!(fmt, "{}", e),
        }
    }
}
