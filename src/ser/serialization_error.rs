use serde;
use std::convert::From;
use std::error::Error;
use std::fmt;

/// Error that can occur while serializing a standard rust type or struct into a SQL parameter.
pub enum SerializationError {
    /// GeneralError, used by the serde framework.
    GeneralError(String),
    /// The structure of the provided type does not fit to the required list of parameters
    StructuralMismatch(&'static str),
    /// The input type does not fit to the required database type.
    TypeMismatch(&'static str, String),
    /// The input value is too big or too small for the required database type.
    /// This type is supposed to be used by the implementors of DbvFactory.
    RangeErr(&'static str, String),
}

impl Error for SerializationError {
    fn description(&self) -> &str {
        match *self {
            SerializationError::GeneralError(_) => "error from framework",
            SerializationError::StructuralMismatch(_) => "structural mismatch",
            SerializationError::TypeMismatch(_, _) => "type mismatch",
            SerializationError::RangeErr(_, _) => "range exceeded",
        }
    }
}

impl From<&'static str> for SerializationError {
    fn from(error: &'static str) -> SerializationError {
        SerializationError::StructuralMismatch(error)
    }
}

impl serde::ser::Error for SerializationError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerializationError::GeneralError(msg.to_string())
    }
}

impl fmt::Debug for SerializationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerializationError::GeneralError(ref s) => write!(fmt, "{}: {}", self.description(), s),
            SerializationError::StructuralMismatch(s) => {
                write!(fmt, "{}: {}", self.description(), s)
            }
            SerializationError::TypeMismatch(s, ref tc) => write!(
                fmt,
                "{}: given value of type \"{}\" cannot be converted into value of type code {}",
                self.description(),
                s,
                tc
            ),
            SerializationError::RangeErr(s1, ref s2) => write!(
                fmt,
                "{}: given value of type \"{}\" does not fit into supported range of SQL type {}",
                self.description(),
                s1,
                s2
            ),
        }
    }
}
impl fmt::Display for SerializationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, fmt)
    }
}
