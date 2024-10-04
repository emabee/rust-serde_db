use crate::de::field_deserializer::FieldDeserializer;
use crate::de::{
    DbValue, DbValueInto, DeserializableRow, DeserializationError, DeserializationResult,
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

// Deserialize a single Row into a normal rust type.
#[derive(Debug)]
pub struct RowDeserializer<Row> {
    row: Row,
    need: Need,
}

impl<Row> RowDeserializer<Row>
where
    Row: DeserializableRow,
    <Row as DeserializableRow>::V: DbValue,
{
    pub fn new(row: Row) -> RowDeserializer<Row> {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::new()");
        let cols_treat = match row.len() {
            1 => Need::Can,
            _ => Need::Must,
        };
        RowDeserializer {
            row,
            need: cols_treat,
        }
    }

    fn get_field_name(&self, idx: usize) -> Option<&str> {
        self.row.field_name(idx)
    }

    fn next_value(&mut self) -> DeserializationResult<Row::V> {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::next_value()");

        if let Need::Must = self.need {
            return Err(DeserializationError::TrailingCols);
        }
        match self.row.next() {
            Some(tv) => Ok(tv),
            None => Err(impl_err("next_value(): no more value found in row")),
        }
    }
}

impl<'x, 'a, Row: DeserializableRow> serde::Deserializer<'x> for &'a mut RowDeserializer<Row>
where
    <Row as DeserializableRow>::V: DbValue,
{
    type Error = DeserializationError;

    fn deserialize_any<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_any()");
        visitor.visit_string(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_bool<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_bool()");
        visitor.visit_bool(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_u8()");
        visitor.visit_u8(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_u16()");
        visitor.visit_u16(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_u32()");
        visitor.visit_u32(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_u64()");
        visitor.visit_u64(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_i8()");
        visitor.visit_i8(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_i16()");
        visitor.visit_i16(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_i32()");
        visitor.visit_i32(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_i64()");
        visitor.visit_i64(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_f32()");
        visitor.visit_f32(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_f64()");
        visitor.visit_f64(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RowDeserializer::deserialize_char()!",
        ))
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_str(), delegates to deserialize_string()");
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_string()");
        visitor.visit_string(SD::deserialize(FieldDeserializer::new(self.next_value()?))?)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RowDeserializer::deserialize_unit()",
        ))
    }

    fn deserialize_option<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_option()");
        FieldDeserializer::new(self.next_value()?).deserialize_option(visitor)
    }

    #[inline]
    fn deserialize_seq<V>(mut self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_seq()");
        if let Need::Done = self.need {
            Err(impl_err(
                "double-nesting (struct/tuple in struct/tuple) not possible",
            ))
        } else {
            self.need = Need::Done;
            visitor.visit_seq(FieldsSeqVisitor::new(self))
        }
    }

    fn deserialize_map<V>(self, _visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RowDeserializer::deserialize_map()",
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
            "RowDeserializer::deserialize_unit_struct()",
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
            "RowDeserializer::deserialize_newtype_struct() with _name = {}",
            _name
        );
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        Err(DeserializationError::NotImplemented(
            "RowDeserializer::deserialize_tuple_struct()",
        ))
    }

    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_struct()");
        if let Need::Done = self.need {
            Err(impl_err("double-nesting (struct in struct) not possible"))
        } else {
            self.need = Need::Done;
            visitor.visit_map(FieldsMapVisitor::new(self))
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_bytes()");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.next_value()?)?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_byte_buf()");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.next_value()?)?)
    }

    fn deserialize_tuple<V>(mut self, _len: usize, visitor: V) -> DeserializationResult<V::Value>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_tuple()");
        if let Need::Done = self.need {
            Err(impl_err(
                "double-nesting (struct/tuple in struct/tuple) not possible",
            ))
        } else {
            self.need = Need::Done;
            visitor.visit_seq(FieldsSeqVisitor::new(self))
        }
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
            "RowDeserializer::deserialize_enum()",
        ))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'x>,
    {
        match self.row.len() {
            0 => Err(impl_err(
                "empty row in RowDeserializer::deserialize_identifier()",
            )),
            curr_len => {
                let idx = self.row.number_of_fields() - curr_len;
                match self.get_field_name(idx) {
                    Some(field_name) => {
                        #[cfg(feature = "trace")]
                        trace!(
                            "RowDeserializer::deserialize_identifier(): column {:?} ({})",
                            idx,
                            field_name
                        );
                        visitor.visit_str(field_name)
                    }
                    None => Err(impl_err(
                        "no field_name in RowDeserializer::deserialize_identifier()",
                    )),
                }
            }
        }
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("RowDeserializer::deserialize_ignored_any()");
        let field_name = self
            .get_field_name(self.row.number_of_fields() - self.row.len())
            .unwrap_or("unknown");
        Err(DeserializationError::UnknownField(field_name.to_string()))
    }
}

struct FieldsMapVisitor<'a, R: 'a + DeserializableRow>
where
    <R as DeserializableRow>::V: DbValue,
{
    de: &'a mut RowDeserializer<R>,
}

impl<'a, R: DeserializableRow> FieldsMapVisitor<'a, R>
where
    <R as DeserializableRow>::V: DbValue,
{
    pub fn new(de: &'a mut RowDeserializer<R>) -> Self {
        #[cfg(feature = "trace")]
        trace!("FieldsMapVisitor::new()");
        FieldsMapVisitor { de }
    }
}

impl<'x, 'a, R: DeserializableRow> serde::de::MapAccess<'x> for FieldsMapVisitor<'a, R>
where
    <R as DeserializableRow>::V: DbValue,
{
    type Error = DeserializationError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'x>,
    {
        match self.de.row.len() {
            0 => {
                #[cfg(feature = "trace")]
                trace!("FieldsMapVisitor::next_key_seed() on empty row");
                Ok(None)
            }
            len => {
                let idx = self.de.row.number_of_fields() - len;
                #[cfg(feature = "trace")]
                trace!("FieldsMapVisitor::next_key_seed() for col {}", idx);
                let value = seed.deserialize(&mut *self.de);
                if let Ok(res) = value {
                    Ok(Some(res))
                } else {
                    let fname = self.de.get_field_name(idx).unwrap();
                    #[cfg(feature = "trace")]
                    trace!("FieldsMapVisitor::next_key_seed(): Error at {}", fname);
                    Err(DeserializationError::UnknownField(fname.to_string()))
                }
            }
        }
    }

    #[allow(clippy::used_underscore_binding)]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'x>,
    {
        match self.de.row.len() {
            0 => Err(impl_err(
                "FieldsMapVisitor::next_value_seed(): no more value",
            )),
            _len => {
                #[cfg(feature = "trace")]
                trace!(
                    "FieldsMapVisitor::next_value_seed() for col {}",
                    self.de.row.number_of_fields() - _len
                );
                seed.deserialize(&mut *self.de)
            }
        }
    }
}

fn impl_err(s: &'static str) -> DeserializationError {
    DeserializationError::Usage(s.to_string())
}

struct FieldsSeqVisitor<'a, R: 'a + DeserializableRow>
where
    <R as DeserializableRow>::V: DbValue,
{
    de: &'a mut RowDeserializer<R>,
}
impl<'a, R: DeserializableRow> FieldsSeqVisitor<'a, R>
where
    <R as DeserializableRow>::V: DbValue,
{
    pub fn new(de: &'a mut RowDeserializer<R>) -> Self {
        #[cfg(feature = "trace")]
        trace!("FieldsSeqVisitor::new()");
        FieldsSeqVisitor { de }
    }
}

impl<'x, 'a, R> serde::de::SeqAccess<'x> for FieldsSeqVisitor<'a, R>
where
    R: DeserializableRow,
    <R as DeserializableRow>::V: DbValue,
{
    type Error = DeserializationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'x>,
    {
        #[cfg(feature = "trace")]
        trace!("FieldsSeqVisitor.next_element_seed()");
        match self.de.row.next() {
            None => Ok(None),
            Some(val) => seed.deserialize(FieldDeserializer::new(val)).map(Some),
        }
    }
}
