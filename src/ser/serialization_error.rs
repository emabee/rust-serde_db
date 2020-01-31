use serde;
use std::convert::From;
use std::error::Error;
use std::fmt;

/// Error that can occur while serializing a standard rust type or struct into a SQL parameter.
pub enum SerializationError {
    /// General Error, used by the serde framework.
    Serde(String),
    /// Parsing the target SQL parameter from the given String representation failed
    Parse {
        /// value
        value: String,
        /// Target SQL type
        db_type: String,
        /// Cause
        cause: Option<Box<dyn Error + Send + Sync + 'static>>,
    },
    /// The structure of the provided type does not fit to the required list of parameters
    StructuralMismatch(&'static str),
    /// The input type does not fit to the required database type.
    Type {
        /// Type of the value that is being serialized
        value_type: &'static str,
        /// Type of the target db parameter
        db_type: String,
    },
    /// The input value is too big or too small for the required database type.
    Range(&'static str, String),
}

/// Factory for Parse Error.
pub fn parse_error<S: AsRef<str>>(
    value: S,
    db_type: String,
    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
) -> SerializationError {
    SerializationError::Parse {
        value: value.as_ref().to_string(),
        db_type,
        cause,
    }
}

/// Factory for Type Error.
#[must_use]
pub fn type_error(value: &'static str, db_type: String) -> SerializationError {
    SerializationError::Type {
        value_type: value,
        db_type,
    }
}

impl Error for SerializationError {
    fn description(&self) -> &str {
        match *self {
            SerializationError::Serde(_) => "error from framework",
            SerializationError::StructuralMismatch(_) => "structural mismatch",
            SerializationError::Parse { cause: ref c, .. } => match c {
                Some(e) => e.description(),
                None => "parse error",
            },
            SerializationError::Type { .. } => "type mismatch",
            SerializationError::Range(_, _) => "range exceeded",
        }
    }
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SerializationError::Serde(_)
            | SerializationError::StructuralMismatch(_)
            | SerializationError::Type { .. }
            | SerializationError::Range(_, _) => None,
            SerializationError::Parse {
                value: ref _v,
                db_type: ref _t,
                cause: ref c,
            } => match c {
                Some(e) => Some(&**e),
                None => None,
            },
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
        SerializationError::Serde(msg.to_string())
    }
}

impl fmt::Debug for SerializationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerializationError::Serde(ref s) => write!(fmt, "{}: {}", self.description(), s),
            SerializationError::StructuralMismatch(s) => {
                write!(fmt, "{}: {}", self.description(), s)
            }
            SerializationError::Parse {
                value: ref v,
                db_type: ref t,
                cause: ref c,
            } => match c {
                Some(e) => write!(
                    fmt,
                    "given String \"{}\" cannot be parsed into SQL type {} due to {}",
                    v, t, e,
                ),
                None => write!(
                    fmt,
                    "given String \"{}\" cannot be parsed into SQL type {}",
                    v, t,
                ),
            },
            SerializationError::Type {
                value_type: ref v,
                db_type: ref d,
            } => write!(
                fmt,
                "given value of type \"{}\" cannot be converted into value of type code {}",
                v, d
            ),
            SerializationError::Range(ref s1, ref s2) => write!(
                fmt,
                "given value of type \"{}\" does not fit into supported range of SQL type {}",
                s1, s2
            ),
        }
    }
}
impl fmt::Display for SerializationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, fmt)
    }
}
