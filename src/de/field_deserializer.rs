use serde;

use de::{DeserError, DeserResult, DbValue, DbValueInto};

/// Deserialize a single DbValue into a normal rust type.
pub struct FieldDeserializer<DBV>(DBV);

impl<DBV> FieldDeserializer<DBV>
    where DBV: DbValue
{
    pub fn new(value: DBV) -> FieldDeserializer<DBV> {
        trace!("FieldDeserializer::new()");
        FieldDeserializer(value)
    }
}

impl<'x, 'a, DBV: DbValue> serde::Deserializer<'x> for FieldDeserializer<DBV> {
    type Error = DeserError;

    fn deserialize_any<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_any()"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_bool()");
        visitor.visit_bool(self.0.try_into()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_u8()");
        visitor.visit_u8(self.0.try_into()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_u16()");
        visitor.visit_u16(self.0.try_into()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_u32()");
        visitor.visit_u32(self.0.try_into()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_u64()");
        visitor.visit_u64(self.0.try_into()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_i8()");
        visitor.visit_i8(self.0.try_into()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_i16()");
        visitor.visit_i16(self.0.try_into()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_i32()");
        visitor.visit_i32(self.0.try_into()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_i64()");
        visitor.visit_i64(self.0.try_into()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_f32()");
        visitor.visit_f32(self.0.try_into()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_f64()");
        visitor.visit_f64(self.0.try_into()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_char()"))
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_str(), delegates to deserialize_string()");
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_string()");
        visitor.visit_string(self.0.try_into()?)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_unit()"))
    }

    fn deserialize_option<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_option()");
        match self.0.is_null() {
            false => visitor.visit_some(self),
            true => visitor.visit_none(),
        }
    }

    fn deserialize_seq<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_seq()"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_map()"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_unit_struct()"))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_newtype_struct()");
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V)
                                   -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_tuple_struct()"))
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str],
                             _visitor: V)
                             -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_struct()"))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_bytes()");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.0)?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        trace!("FieldDeserializer::deserialize_bytes()");
        visitor.visit_bytes(&DbValueInto::<Vec<u8>>::try_into(self.0)?)
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> DeserResult<V::Value>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_tuple()"))
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str],
                           _visitor: V)
                           -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_enum()"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_identifier()"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'x>
    {
        Err(DeserError::NotImplemented("FieldDeserializer::deserialize_ignored_any()"))
    }
}
