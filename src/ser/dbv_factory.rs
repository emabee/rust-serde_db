use super::SerializationError;

/// A factory for database objects.
pub trait DbvFactory: Sized {
    /// The type of the database objects.
    type DBV;
    /// Serialize a bool.
    fn from_bool(&self, value: bool) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i8.
    fn from_i8(&self, value: i8) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i16.
    fn from_i16(&self, value: i16) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i32.
    fn from_i32(&self, value: i32) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i64.
    fn from_i64(&self, value: i64) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u8.
    fn from_u8(&self, value: u8) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u16.
    fn from_u16(&self, value: u16) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u32.
    fn from_u32(&self, value: u32) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u64.
    fn from_u64(&self, value: u64) -> Result<Self::DBV, SerializationError>;
    /// Serialize an f32.
    fn from_f32(&self, value: f32) -> Result<Self::DBV, SerializationError>;
    /// Serialize an f64.
    fn from_f64(&self, value: f64) -> Result<Self::DBV, SerializationError>;
    /// Serialize a char.
    fn from_char(&self, value: char) -> Result<Self::DBV, SerializationError>;
    /// Serialize a str.
    fn from_str(&self, value: &str) -> Result<Self::DBV, SerializationError>;
    /// Serialize bytes.
    fn from_bytes(&self, value: &[u8]) -> Result<Self::DBV, SerializationError>;
    /// Serialize a none.
    fn from_none(&self) -> Result<Self::DBV, SerializationError>;
    /// Provide a descriptive String of the type that is required (for error messages).
    fn descriptor(&self) -> String;
}
