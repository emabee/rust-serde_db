use serde;
use std::marker::Sized;
use std::{i16, i32, i8, u16, u32, u8};

use crate::de::field_deserializer::FieldDeserializer;
use crate::de::{DbValueInto, DeserializationError};

/// Provides the conversion of a database value into a standard rust type.
pub trait DbValue:
    Sized
    + DbValueInto<bool>
    + DbValueInto<u8>
    + DbValueInto<u16>
    + DbValueInto<u32>
    + DbValueInto<u64>
    + DbValueInto<i8>
    + DbValueInto<i16>
    + DbValueInto<i32>
    + DbValueInto<i64>
    + DbValueInto<f32>
    + DbValueInto<f64>
    + DbValueInto<String>
    + DbValueInto<Vec<u8>>
{
    /// Returns true if this is a NULL value.
    fn is_null(&self) -> bool;

    /// Converts the `DbValue` into a plain rust value.
    ///
    /// The generic implementation of this method is based on to-be-provided
    /// implementations of the target-type-specific trait `DbValueInto`.
    ///
    /// # Errors
    /// `DeserializationError` if the value cannot be converted into the target type.
    fn into_typed<'de, T>(self) -> Result<T, DeserializationError>
    where
        T: serde::de::Deserialize<'de>,
    {
        Ok(serde::de::Deserialize::deserialize(
            FieldDeserializer::new(self),
        )?)
    }
}
