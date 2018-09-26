use serde;
use std::convert::From;
use std::error::Error;
use std::fmt;

/// Error that can occur while serializing a standard rust type or struct into a SQL parameter.
pub enum SerializationError {
    /// GeneralError, used by the serde framework.
    GeneralError(String),
    /// Parsing the target SQL parameter from the given String representation failed
    ParseError {
        /// value
        value: String,
        /// Target SQL type
        typedesc: String,
    },
    /// The structure of the provided type does not fit to the required list of parameters
    StructuralMismatch(&'static str),
    /// The input type does not fit to the required database type.
    TypeMismatch(&'static str, String),
    /// The input value is too big or too small for the required database type.
    /// This type is supposed to be used by the implementors of DbvFactory.
    RangeErr(&'static str, String),
}

impl SerializationError {
    /// Factory for ParseError.
    pub fn parse_error<S: AsRef<str>>(value: S, typedesc: S) -> SerializationError {
        SerializationError::ParseError {
            value: value.as_ref().to_string(),
            typedesc: typedesc.as_ref().to_string(),
        }
    }
}

impl Error for SerializationError {
    fn description(&self) -> &str {
        match *self {
            SerializationError::GeneralError(_) => "error from framework",
            SerializationError::StructuralMismatch(_) => "structural mismatch",
            SerializationError::ParseError {
                value: ref _v,
                typedesc: ref _t,
            } => "parse error",
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
            SerializationError::ParseError {
                value: ref v,
                typedesc: ref t,
            } => write!(
                fmt,
                "{}: given String \"{}\" cannot be parsed into SQL type {}",
                self.description(),
                v,
                t
            ),
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
