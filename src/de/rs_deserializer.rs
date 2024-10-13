use crate::de::row_deserializer::RowDeserializer;
use crate::de::{
    DbValue, DeserializableResultSet, DeserializableRow, DeserializationError,
    DeserializationResult,
};
#[cfg(feature = "trace")]
use log::trace;
use serde::Deserialize as SD;

#[derive(Debug)]
enum Need {
    Must,
    Can,
    Done,
}

// Deserialize a ResultSet into a normal rust type.
#[derive(Debug)]
pub struct RsDeserializer<RS> {
    rs: RS,
    need: Need,
}

impl<RS> RsDeserializer<RS>
where
    RS: DeserializableResultSet,
    <<RS as DeserializableResultSet>::Row as DeserializableRow>::Value: DbValue,
{
    pub fn try_new(mut rs: RS) -> Result<RsDeserializer<RS>, DeserializationError> {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::new()");
        let need = if rs.has_multiple_rows()? {
            Need::Must
        } else {
            Need::Can
        };
        Ok(RsDeserializer { rs, need })
    }

    fn pop_single_row(&mut self) -> DeserializationResult<<RS as DeserializableResultSet>::Row> {
        if let Need::Must = self.need {
            return Err(DeserializationError::TrailingRows);
        };
        match self.rs.next()? {
            None => Err(DeserializationError::Usage(String::from(
                "no row found in result set",
            ))),
            Some(row) => Ok(row),
        }
    }
}

impl<'x, 'a, RS: DeserializableResultSet> serde::Deserializer<'x> for &'a mut RsDeserializer<RS>
where
    <<RS as DeserializableResultSet>::Row as DeserializableRow>::Value: DbValue,
{
    type Error = DeserializationError;

    fn deserialize_any<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        visitor.visit_string(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_bool<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_bool()");
        visitor.visit_bool(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_u8()");
        visitor.visit_u8(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_u16()");
        visitor.visit_u16(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_u32()");
        visitor.visit_u32(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_u64()");
        visitor.visit_u64(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_i8()");
        visitor.visit_i8(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_i16()");
        visitor.visit_i16(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_i32()");
        visitor.visit_i32(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_i64()");
        visitor.visit_i64(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_f32()");
        visitor.visit_f32(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_f64()");
        visitor.visit_f64(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RsDeserializer::deserialize_char()",
        ))
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_str(), delegates to deserialize_string()");
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_string()");
        visitor.visit_string(SD::deserialize(&mut RowDeserializer::new(
            self.pop_single_row()?,
        ))?)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RsDeserializer::deserialize_unit()",
        ))
    }

    fn deserialize_option<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_option()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_option(visitor)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_seq()");
        if let Need::Done = self.need {
            Err(DeserializationError::Usage(
                "deserialize_seq() when already done".to_string(),
            ))
        } else {
            self.need = Need::Done;
            Ok(visitor.visit_seq(RowsVisitor::new(self))?)
        }
    }

    fn deserialize_map<V>(self, _visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RsDeserializer::deserialize_map()",
        ))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RsDeserializer::deserialize_unit_struct()",
        ))
    }

    #[allow(clippy::used_underscore_binding)]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!(
            "RsDeserializer::deserialize_newtype_struct() with name = {}",
            _name
        );
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!(
            "RsDeserializer::deserialize_tuple_struct() with name = {}",
            name
        );
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_struct() with name = {}", name);
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_bytes()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_byte_buf()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_byte_buf(visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RsDeserializer::deserialize_tuple()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_tuple(len, visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RsDeserializer::deserialize_enum()",
        ))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RsDeserializer::deserialize_identifier()",
        ))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RsDeserializer::deserialize_ignored_any()",
        ))
    }
}

// we use generalization <R> here because this allows us to bind the parameter to the lifetime 'a
struct RowsVisitor<'a, R: 'a> {
    de: &'a mut RsDeserializer<R>,
}

impl<'a, R> RowsVisitor<'a, R> {
    pub fn new(de: &'a mut RsDeserializer<R>) -> Self {
        #[cfg(feature = "trace")]
        trace!("RowsVisitor::new()");
        RowsVisitor { de }
    }
}

impl<'x, 'a, R: DeserializableResultSet> serde::de::SeqAccess<'x> for RowsVisitor<'a, R> {
    type Error = DeserializationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowsVisitor.next_element_seed()");
        match self.de.rs.next()? {
            None => Ok(None),
            Some(row) => seed.deserialize(&mut RowDeserializer::new(row)).map(Some),
        }
    }
}
