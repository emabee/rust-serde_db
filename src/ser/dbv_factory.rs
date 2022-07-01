use super::SerializationError;

/// A factory for database objects.
///
/// This trait is to be implemented by descriptors of parameters for database commands.
///
/// # Example
///
/// A parameter descriptor for a String-valued database type might implement all
/// methods with adequate conversions, while a parameter descriptor for an integer type might
/// only support conversions from the rust integer types.
pub trait DbvFactory: Sized {
    /// The type of the database objects.
    type DBV;
    /// Serialize a bool.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_bool(&self, value: bool) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i8.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_i8(&self, value: i8) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i16.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_i16(&self, value: i16) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i32.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_i32(&self, value: i32) -> Result<Self::DBV, SerializationError>;
    /// Serialize an i64.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_i64(&self, value: i64) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u8.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_u8(&self, value: u8) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u16.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_u16(&self, value: u16) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u32.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_u32(&self, value: u32) -> Result<Self::DBV, SerializationError>;
    /// Serialize an u64.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_u64(&self, value: u64) -> Result<Self::DBV, SerializationError>;
    /// Serialize an f32.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_f32(&self, value: f32) -> Result<Self::DBV, SerializationError>;
    /// Serialize an f64.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_f64(&self, value: f64) -> Result<Self::DBV, SerializationError>;
    /// Serialize a char.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_char(&self, value: char) -> Result<Self::DBV, SerializationError>;
    /// Serialize a str.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_str(&self, value: &str) -> Result<Self::DBV, SerializationError>;
    /// Serialize bytes.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_bytes(&self, value: &[u8]) -> Result<Self::DBV, SerializationError>;
    /// Serialize a none.
    ///
    /// # Errors
    ///
    /// `SerializationError` if the required database parameter cannot be constructed
    /// from the given value.
    fn serialize_none(&self) -> Result<Self::DBV, SerializationError>;
    /// Provide a descriptive String of the type that is required (for error messages).
    fn descriptor(&self) -> String;
}
