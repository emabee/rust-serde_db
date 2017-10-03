//! Deserialize a ResultSet into a normal rust type.

mod db_value;
mod conversion_error;
mod deserializable_resultset;
mod deserializable_row;
mod deserialization_error;
mod field_deserializer;
pub mod row;
mod row_deserializer;
mod rs_deserializer;

pub use de::conversion_error::ConversionError;
pub use self::deserialization_error::DeserError;

pub use self::db_value::{DbValue, DbValueInto};
pub use self::deserializable_resultset::DeserializableResultset;
pub use self::deserializable_row::DeserializableRow;
