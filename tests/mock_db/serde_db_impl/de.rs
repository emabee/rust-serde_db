use mock_db;
use mock_db::{MValue, Resultset};
use serde_db::de::{ConversionError, DbValue, DbValueInto, DeserializableResultset,
                   DeserializationError};
use std::{i16, i32, i8, u16, u32, u8};

fn not_implemented(s: &'static str) -> ConversionError {
    ConversionError::ValueType(format!("{} not implemented", s))
}

impl DbValue for MValue {
    fn is_null(&self) -> bool {
        match *self {
            MValue::NullableShort(None) => true,
            _ => false,
        }
    }
}

impl DbValueInto<bool> for MValue {
    fn try_into(self) -> Result<bool, ConversionError> {
        Err(not_implemented("DbValueInto<bool>"))
    }
}
impl DbValueInto<u8> for MValue {
    fn try_into(self) -> Result<u8, ConversionError> {
        Err(not_implemented("DbValueInto<u8>"))
    }
}
impl DbValueInto<u16> for MValue {
    fn try_into(self) -> Result<u16, ConversionError> {
        Err(not_implemented("DbValueInto<u16>"))
    }
}
impl DbValueInto<u32> for MValue {
    fn try_into(self) -> Result<u32, ConversionError> {
        Err(not_implemented("DbValueInto<u32>"))
    }
}
impl DbValueInto<u64> for MValue {
    fn try_into(self) -> Result<u64, ConversionError> {
        Err(not_implemented("DbValueInto<u64>"))
    }
}
impl DbValueInto<i8> for MValue {
    fn try_into(self) -> Result<i8, ConversionError> {
        Err(not_implemented("DbValueInto<i8>"))
    }
}
impl DbValueInto<i16> for MValue {
    fn try_into(self) -> Result<i16, ConversionError> {
        Err(not_implemented("DbValueInto<i16>"))
    }
}
impl DbValueInto<i32> for MValue {
    fn try_into(self) -> Result<i32, ConversionError> {
        match self {
            MValue::Short(i) | MValue::NullableShort(Some(i)) => Ok(i32::from(i)),
            mv => Err(ConversionError::ValueType(
                format!("DbValueInto<i32> not implemented for {:?}", mv),
            )),
        }
    }
}
impl DbValueInto<i64> for MValue {
    fn try_into(self) -> Result<i64, ConversionError> {
        Err(not_implemented("DbValueInto<i64>"))
    }
}
impl DbValueInto<f32> for MValue {
    fn try_into(self) -> Result<f32, ConversionError> {
        Err(not_implemented("DbValueInto<f32>"))
    }
}
impl DbValueInto<f64> for MValue {
    fn try_into(self) -> Result<f64, ConversionError> {
        Err(not_implemented("DbValueInto<f64>"))
    }
}
impl DbValueInto<String> for MValue {
    fn try_into(self) -> Result<String, ConversionError> {
        trace!("try_into -> String");
        match self {
            MValue::String(s) => Ok(s),
            MValue::Timestamp(ts) => Ok(ts.to_string()),
            mv => Err(ConversionError::ValueType(
                format!("DbValueInto<String> not implemented for {:?}", mv),
            )),
        }
    }
}
impl DbValueInto<Vec<u8>> for MValue {
    fn try_into(self) -> Result<Vec<u8>, ConversionError> {
        Err(not_implemented("DbValueInto<Vec<u8>>"))
    }
}


impl DeserializableResultset for Resultset {
    type E = mock_db::Error;
    type ROW = mock_db::Row;
    fn has_multiple_rows(&mut self) -> Result<bool, DeserializationError> {
        Ok(self.rows.len() > 1_usize)
    }

    fn reverse_rows(&mut self) {
        self.rows.reverse()
    }

    fn pop_row(&mut self)
               -> Result<Option<<Self as DeserializableResultset>::ROW>, DeserializationError> {
        Ok(self.rows.pop())
    }

    fn number_of_fields(&self) -> usize {
        self.md.number_of_fields()
    }

    fn get_fieldname(&self, i: usize) -> Option<&String> {
        self.md.get_fieldname(i)
    }

    fn fetch_all(&mut self) -> Result<(), <Self as DeserializableResultset>::E> {
        Ok(())
    }
}


impl From<DeserializationError> for mock_db::Error {
    fn from(e: DeserializationError) -> mock_db::Error {
        mock_db::Error::DESERIALIZATION(e)
    }
}
