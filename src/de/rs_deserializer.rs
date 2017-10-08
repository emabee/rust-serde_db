use serde;

use serde::de::Deserialize as SD;

use super::{DbValue, DeserError, DeserResult, DeserializableRow};
use super::DeserializableResultset as De_Resultset;
use super::row_deserializer::RowDeserializer;

enum MCD {
    Must,
    Can,
    Done,
}

// Deserialize a ResultSet into a normal rust type.
pub struct RsDeserializer<RS> {
    rs: RS,
    rows_treat: MCD,
}

impl<RS> RsDeserializer<RS>
    where RS: De_Resultset,
          <<RS as De_Resultset>::ROW as DeserializableRow>::V: DbValue
{
    pub fn new(mut rs: RS) -> Result<RsDeserializer<RS>, DeserError> {
        trace!("RsDeserializer::new()");
        let rows_treat = match rs.has_multiple_rows()? {
            true => MCD::Must,
            false => MCD::Can,
        };
        Ok(RsDeserializer {
            rows_treat: rows_treat,
            rs: rs,
        })
    }

    fn pop_single_row(&mut self) -> DeserResult<<RS as De_Resultset>::ROW> {
        self.single_row_deserialization_allowed()?;
        match self.rs.pop_row()? {
            None => Err(DeserError::Implementation(String::from("no row found in resultset"))),
            Some(row) => Ok(row),
        }
    }

    fn single_row_deserialization_allowed(&self) -> DeserResult<()> {
        match self.rows_treat {
            MCD::Must => Err(DeserError::TrailingRows),
            _ => Ok(()),
        }
    }
}

impl<'x, 'a, RS: De_Resultset> serde::Deserializer<'x> for &'a mut RsDeserializer<RS>
where <<RS as De_Resultset>::ROW as DeserializableRow>::V: DbValue {
    type Error = DeserError;

    fn deserialize_any<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize()"))
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_bool()");
        visitor.visit_bool(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_u8()");
        visitor.visit_u8(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_u16<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_u16()");
        visitor.visit_u16(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_u32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_u32()");
        visitor.visit_u32(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_u64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_u64()");
        visitor.visit_u64(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_i8()");
        visitor.visit_i8(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_i16<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_i16()");
        visitor.visit_i16(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_i32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_i32()");
        visitor.visit_i32(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_i64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_i64()");
        visitor.visit_i64(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_f32()");
        visitor.visit_f32(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_f64()");
        visitor.visit_f64(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize_char()"))
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_str(), delegates to deserialize_string()");
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_string()");
        visitor.visit_string(SD::deserialize(&mut RowDeserializer::new(self.pop_single_row()?))?)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize_unit()"))
    }

    fn deserialize_option<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_option()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_option(visitor)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_seq()");
        match self.rows_treat {
            MCD::Done => {
                Err(DeserError::Implementation("deserialize_seq() when rows_treat = MCD::Done"
                    .to_string()))
            }
            _ => {
                self.rows_treat = MCD::Done;
// consuming from the end is easier and faster
                Ok(visitor.visit_seq(RowsVisitor::new(&mut self))?)
            }
        }
    }

    fn deserialize_map<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize_map()"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize_unit_struct()"))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V)
    -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_newtype_struct() with _name = {}", _name);
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple_struct<V>(self,
                                   name: &'static str,
                                   len: usize,
                                   visitor: V)
                                   -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_tuple_struct() with name = {}", name);
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_struct<V>(self,
                             name: &'static str,
                             fields: &'static [&'static str],
                             visitor: V)
                             -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_struct() with name = {}", name);
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_bytes()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_byte_buf()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_byte_buf(visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RsDeserializer::deserialize_tuple()");
        let mut rd = RowDeserializer::new(self.pop_single_row()?);
        rd.deserialize_tuple(len, visitor)
    }

    fn deserialize_enum<V>(self,
                           _name: &'static str,
                           _variants: &'static [&'static str],
                           _visitor: V)
                           -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize_enum()"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize_identifier()"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RsDeserializer::deserialize_ignored_any()"))
    }
}

// we use generalization <R> here because this allows us to bind the parameter to the lifetime 'a
struct RowsVisitor<'a, R: 'a> {
    de: &'a mut RsDeserializer<R>,
}

impl<'a, R> RowsVisitor<'a, R> {
    pub fn new(de: &'a mut RsDeserializer<R>) -> Self {
        trace!("RowsVisitor::new()");
        RowsVisitor { de: de }
    }
}

impl<'x, 'a, R: De_Resultset> serde::de::SeqAccess<'x> for RowsVisitor<'a, R> {
    type Error = DeserError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: serde::de::DeserializeSeed<'x>
    {
        trace!("RowsVisitor.next_element_seed()");
        match self.de.rs.pop_row()? {
            None => Ok(None),
            Some(row) => seed.deserialize(&mut RowDeserializer::new(row)).map(|v| Some(v)),
        }
    }
}
