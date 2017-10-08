use serde;
use serde::de::Deserialize as SD;

use de::{DbValue, DbValueInto, DeserError, DeserResult, DeserializableRow};
use de::field_deserializer::FieldDeserializer;

enum MCD {
    Must,
    Can,
    Done,
}

// Deserialize a single Row into a normal rust type.
pub struct RowDeserializer<ROW> {
    row: ROW,
    cols_treat: MCD,
}

impl<ROW> RowDeserializer<ROW>
    where ROW: DeserializableRow,
          <ROW as DeserializableRow>::V: DbValue
{
    pub fn new(row: ROW) -> RowDeserializer<ROW> {
        trace!("RowDeserializer::new()");
        let cols_treat = match row.len() {
            1 => MCD::Can,
            _ => MCD::Must,
        };
        RowDeserializer {
            cols_treat: cols_treat,
            row: row,
        }
    }

    fn value_deserialization_allowed(&self) -> DeserResult<()> {
        match self.cols_treat {
            MCD::Must => Err(DeserError::TrailingCols),
            _ => Ok(()),
        }
    }

    fn get_fieldname(&self, idx: usize) -> Option<&String> {
        self.row.get_fieldname(idx)
    }


    fn value_pop(&mut self) -> DeserResult<ROW::V> {
        trace!("RowDeserializer::value_pop()");
        self.value_deserialization_allowed()?;
        match self.row.pop() {
            Some(tv) => Ok(tv),
            None => Err(impl_err("value_pop(): no more value found in row")),
        }
    }
}

impl<'x, 'a, ROW: DeserializableRow> serde::Deserializer<'x> for &'a mut RowDeserializer<ROW>
    where <ROW as DeserializableRow>::V: DbValue
{
    type Error = DeserError;

    fn deserialize_any<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize()"))
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_bool()");
        visitor.visit_bool(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u8()");
        visitor.visit_u8(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_u16<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u16()");
        visitor.visit_u16(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_u32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u32()");
        visitor.visit_u32(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_u64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_u64()");
        visitor.visit_u64(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i8()");
        visitor.visit_i8(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_i16<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i16()");
        visitor.visit_i16(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_i32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i32()");
        visitor.visit_i32(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_i64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_i64()");
        visitor.visit_i64(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_f32()");
        visitor.visit_f32(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_f64()");
        visitor.visit_f64(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize_char()!"))
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_str(), delegates to deserialize_string()");
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_string()");
        visitor.visit_string(SD::deserialize(FieldDeserializer::new(self.value_pop()?))?)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize_unit()"))
    }

    fn deserialize_option<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_option()");
        FieldDeserializer::new(self.value_pop()?).deserialize_option(visitor)
    }

    #[inline]
    fn deserialize_seq<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize_seq()"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize_map()"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize_unit_struct()"))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_newtype_struct() with _name = {}", _name);
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V)
                                   -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize_tuple_struct()"))
    }

    fn deserialize_struct<V>(mut self, _name: &'static str, _fields: &'static [&'static str],
                             visitor: V)
                             -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_struct()");
        match self.cols_treat {
            MCD::Done => Err(impl_err("double-nesting (struct in struct) not possible")),
            _ => {
                self.cols_treat = MCD::Done;
                visitor.visit_map(FieldsMapVisitor::new(&mut self))
            }
        }
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_bytes()");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.value_pop()?)?)
    }

    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_byte_buf()");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.value_pop()?)?)
    }

    fn deserialize_tuple<V>(mut self, _len: usize, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_tuple()");
        match self.cols_treat {
            MCD::Done => {
                Err(impl_err("double-nesting (struct/tuple in struct/tuple) not possible"))
            }
            _ => {
                self.cols_treat = MCD::Done;
                visitor.visit_seq(FieldsSeqVisitor::new(&mut self))
            }
        }
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str],
                           _visitor: V)
                           -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("RowDeserializer::deserialize_enum()"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        let i: usize = self.row.len();
        if i == 0 {
            return Err(impl_err("empty row in RowDeserializer::deserialize_identifier()"));
        }
        match self.get_fieldname(i - 1) {
            Some(fieldname) => {
                trace!("RowDeserializer::deserialize_identifier(): column {:?} ({})",
                       i - 1,
                       fieldname);
                visitor.visit_str(fieldname)
            }
            None => Err(impl_err("no fieldname in RowDeserializer::deserialize_identifier()")),
        }
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        trace!("RowDeserializer::deserialize_ignored_any()");
        let fieldname = self.get_fieldname(self.row.len() - 1)
                            .map(|s| s.clone())
                            .unwrap_or("unknown".to_string());
        Err(DeserError::UnknownField(fieldname))
    }
}



struct FieldsMapVisitor<'a, R: 'a + DeserializableRow>
    where <R as DeserializableRow>::V: DbValue
{
    de: &'a mut RowDeserializer<R>,
}

impl<'a, R: DeserializableRow> FieldsMapVisitor<'a, R>
    where <R as DeserializableRow>::V: DbValue
{
    pub fn new(de: &'a mut RowDeserializer<R>) -> Self {
        trace!("FieldsMapVisitor::new()");
        FieldsMapVisitor { de: de }
    }
}

impl<'x, 'a, R: DeserializableRow> serde::de::MapAccess<'x> for FieldsMapVisitor<'a, R>
    where <R as DeserializableRow>::V: DbValue
{
    type Error = DeserError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where K: serde::de::DeserializeSeed<'x>
    {
        match self.de.row.len() {
            0 => {
                trace!("FieldsMapVisitor::next_key_seed() on empty row");
                Ok(None)
            }
            len => {
                let idx = len - 1;
                trace!("FieldsMapVisitor::next_key_seed() for col {}", idx);
                let value = seed.deserialize(&mut *self.de);
                match value {
                    Ok(res) => Ok(Some(res)),
                    Err(_) => {
                        let fname = self.de.get_fieldname(idx).unwrap();
                        trace!("FieldsMapVisitor::next_key_seed(): Error at {}", fname);
                        Err(DeserError::UnknownField(fname.clone()))
                    }
                }
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where V: serde::de::DeserializeSeed<'x>
    {
        match self.de.row.len() {
            0 => Err(impl_err("FieldsMapVisitor::next_value_seed(): no more value")),
            len => {
                trace!("FieldsMapVisitor::next_value_seed() for col {}", len - 1);
                seed.deserialize(&mut *self.de)
            }
        }
    }
}

fn impl_err(s: &'static str) -> DeserError {
    DeserError::Implementation(s.to_string())
}


struct FieldsSeqVisitor<'a, R: 'a + DeserializableRow>
    where <R as DeserializableRow>::V: DbValue
{
    de: &'a mut RowDeserializer<R>,
}
impl<'a, R: DeserializableRow> FieldsSeqVisitor<'a, R>
    where <R as DeserializableRow>::V: DbValue
{
    pub fn new(de: &'a mut RowDeserializer<R>) -> Self {
        trace!("FieldsSeqVisitor::new()");
        de.row.reverse_values();
        FieldsSeqVisitor { de: de }
    }
}

impl<'x, 'a, R> serde::de::SeqAccess<'x> for FieldsSeqVisitor<'a, R>
    where R: DeserializableRow,
          <R as DeserializableRow>::V: DbValue
{
    type Error = DeserError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: serde::de::DeserializeSeed<'x>
    {
        trace!("FieldsSeqVisitor.next_element_seed()");
        match self.de.row.pop() {
            None => Ok(None),
            Some(val) => seed.deserialize(FieldDeserializer::new(val)).map(|v| Some(v)),
        }
    }
}
