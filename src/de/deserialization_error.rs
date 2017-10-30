use serde;
use std::error;
use std::error::Error;
use std::fmt;

use de::ConversionError;

/// The errors that can arise while deserializing with `serde_db::de`.
pub enum DeserializationError {
    /// Deserialization failed due to a conversion error.
    ConversionError(ConversionError),
    /// Raised when there is a general error in the serde framework when deserializing.
    SerdeError(String),
    /// Structure of target object does not fit to the structure of the object being deserialized.
    Implementation(String),
    /// Thrown by functions in the Deserializer interface that are not implemented.
    /// This exception should never be seen in practice.
    NotImplemented(&'static str),
    /// The target structure misses a field for which data are provided.
    UnknownField(String),
    /// The conversion cannot consume all existing rows.
    TrailingRows,
    /// The conversion cannot consume all existing columns.
    TrailingCols,
}

impl error::Error for DeserializationError {
    fn description(&self) -> &str {
        match *self {
            DeserializationError::ConversionError(_) => {
                "Conversion of database type to rust type failed"
            }
            DeserializationError::SerdeError(_) => {
                "general error from the deserialization framework"
            }
            DeserializationError::Implementation(_) => {
                "error in the implementation of the serde_db"
            }
            DeserializationError::NotImplemented(_) => "function not implemented",
            DeserializationError::UnknownField(_) => {
                "the target structure misses a field for which data are provided"
            }
            DeserializationError::TrailingRows => "trailing rows",
            DeserializationError::TrailingCols => "trailing columns",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DeserializationError::ConversionError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<ConversionError> for DeserializationError {
    fn from(error: ConversionError) -> DeserializationError {
        DeserializationError::ConversionError(error)
    }
}

impl serde::de::Error for DeserializationError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DeserializationError::SerdeError(msg.to_string())
    }
}

impl fmt::Debug for DeserializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializationError::ConversionError(ref e) => write!(formatter, "{:?}", e),
            DeserializationError::SerdeError(ref s) |
            DeserializationError::Implementation(ref s) |
            DeserializationError::UnknownField(ref s) => {
                write!(formatter, "{} (\"{}\")", self.description(), s)
            }
            DeserializationError::NotImplemented(s) => {
                write!(formatter, "{} (\"{}\")", self.description(), s)
            }
            DeserializationError::TrailingRows | DeserializationError::TrailingCols => {
                write!(formatter, "{}", self.description())
            }
        }
    }
}
impl fmt::Display for DeserializationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializationError::ConversionError(ref e) => write!(fmt, "{}", e),
            DeserializationError::SerdeError(ref s) |
            DeserializationError::Implementation(ref s) |
            DeserializationError::UnknownField(ref s) => write!(fmt, "{} ", s),
            DeserializationError::NotImplemented(s) => write!(fmt, "{} ", s),
            DeserializationError::TrailingRows => write!(fmt, "{} ", "TrailingRows"),
            DeserializationError::TrailingCols => write!(fmt, "{} ", "TrailingCols"),
        }
    }
}

/// A specialized Result type for deserialization.
pub type DeserializationResult<T> = Result<T, DeserializationError>;
