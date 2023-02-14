use thiserror::Error;

/// Error that can occur while serializing a standard rust type or struct into a SQL parameter.
#[derive(Error)] //Copy, Clone, Debug, Eq, PartialEq,
pub enum SerializationError {
    /// Error occured within the serde framework
    #[error("Error occured within the serde framework")]
    Serde(String),

    /// Error occured while parsing the SQL parameter from the given String representation
    #[error("Error occured while parsing the SQL parameter from the given String representation")]
    Parse {
        /// value
        value: String,
        /// Target SQL type
        db_type: String,

        /// The causing Error
        cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },

    /// The structure of the provided type does not fit to the required list of parameters
    #[error("The structure of the provided type does not fit to the required list of parameters")]
    StructuralMismatch(&'static str),

    /// The input type does not fit to the required database type
    #[error("The input type does not fit to the required database type")]
    Type {
        /// Type of the value that is being serialized
        value_type: &'static str,
        /// Type of the target db parameter
        db_type: String,
    },

    /// The input value is too big or too small for the required database type
    #[error("The input value is too big or too small for the required database type")]
    Range(&'static str, String),
}

/// Factory for Parse Error.
pub fn parse_error<S: AsRef<str>>(
    value: S,
    db_type: String,
    cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
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

impl From<&'static str> for SerializationError {
    fn from(error: &'static str) -> SerializationError {
        SerializationError::StructuralMismatch(error)
    }
}

impl serde::ser::Error for SerializationError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        SerializationError::Serde(msg.to_string())
    }
}

impl std::fmt::Debug for SerializationError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SerializationError::Serde(ref s) => write!(fmt, "{self}: {s}"),
            SerializationError::StructuralMismatch(s) => write!(fmt, "{self}: {s}"),
            SerializationError::Parse {
                ref value,
                ref db_type,
                ref cause,
            } => match cause {
                Some(e) => write!(
                    fmt,
                    "given String \"{value}\" cannot be parsed into SQL type {db_type} due to {e}",
                ),
                None => write!(
                    fmt,
                    "given String \"{value}\" cannot be parsed into SQL type {db_type}",
                ),
            },
            SerializationError::Type {
                value_type,
                ref db_type,
            } => write!(
                fmt,
                "given value of type \"{value_type}\" cannot be converted into value of type code {db_type}",
            ),
            SerializationError::Range(s1, ref s2) => write!(
                fmt,
                "given value of type \"{s1}\" does not fit into supported range of SQL type {s2}",
            ),
        }
    }
}
