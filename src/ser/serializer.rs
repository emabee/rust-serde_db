use super::dbv_factory::DbvFactory;
use super::type_error;
use super::SerializationError;
#[cfg(feature = "trace")]
use log::{log_enabled, trace};

type SerializationResult<T> = Result<T, SerializationError>;

/// A structure for serializing Rust values into a parameter row for a prepared statement.
#[allow(missing_debug_implementations)]
pub struct Serializer<'m, DF: 'm + DbvFactory> {
    output: Vec<DF::DBV>,
    metadata: &'m mut dyn std::iter::Iterator<Item = DF>,
}

impl<'m, DF: DbvFactory> Serializer<'m, DF> {
    pub fn new(metadata: &'m mut dyn std::iter::Iterator<Item = DF>) -> Self {
        Serializer {
            output: Vec::<DF::DBV>::new(),
            metadata,
        }
    }
    fn get_current_field(&mut self) -> SerializationResult<DF> {
        match self.metadata.next() {
            Some(df) => Ok(df),
            None => Err(SerializationError::StructuralMismatch(
                "too many values specified",
            )),
        }
    }

    fn push(&mut self, value: DF::DBV) {
        self.output.push(value);
    }

    pub fn into_inner(self) -> Vec<DF::DBV> {
        self.output
    }
}

impl<'a, 'm: 'a, DF: DbvFactory> serde::Serializer for &'a mut Serializer<'m, DF> {
    type Ok = ();
    type Error = SerializationError;
    type SerializeSeq = Compound<'a, 'm, DF>;
    type SerializeTuple = Compound<'a, 'm, DF>;
    type SerializeTupleStruct = Compound<'a, 'm, DF>;
    type SerializeTupleVariant = Compound<'a, 'm, DF>;
    type SerializeMap = Compound<'a, 'm, DF>;
    type SerializeStruct = Compound<'a, 'm, DF>;
    type SerializeStructVariant = Compound<'a, 'm, DF>;

    fn serialize_bool(self, value: bool) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_bool()");
        let val = self.get_current_field()?.serialize_bool(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_i8()");
        let val = self.get_current_field()?.serialize_i8(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_i16()");
        let val = self.get_current_field()?.serialize_i16(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_i32() for {}", value);
        let val = self.get_current_field()?.serialize_i32(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_i64()");
        let val = self.get_current_field()?.serialize_i64(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_u8()");
        let val = self.get_current_field()?.serialize_u8(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_u16()");
        let val = self.get_current_field()?.serialize_u16(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_u32()");
        let val = self.get_current_field()?.serialize_u32(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_u64()");
        let val = self.get_current_field()?.serialize_u64(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_f32()");
        let val = self.get_current_field()?.serialize_f32(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_f64()");
        let val = self.get_current_field()?.serialize_f64(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_char(self, value: char) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_char()");
        let val = self.get_current_field()?.serialize_char(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_str(self, value: &str) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        if log_enabled!(log::Level::Debug) {
            let l = value.len();
            if l < 100 {
                trace!("Serializer::serialize_str() with {}", value);
            } else {
                trace!(
                    "Serializer::serialize_str() with {}..{}",
                    head(20, value),
                    tail(20, value)
                );
            }
        }
        let val = self.get_current_field()?.serialize_str(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_bytes()");
        let val = self.get_current_field()?.serialize_bytes(value)?;
        self.push(val);
        Ok(())
    }

    fn serialize_unit(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_unit()");
        let val = self.get_current_field()?.serialize_none()?;
        self.push(val);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_unit_struct()");
        Err(type_error(
            "unit_struct",
            self.get_current_field()?.descriptor(),
        ))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_unit_variant()");
        Err(type_error(
            "unit_variant",
            self.get_current_field()?.descriptor(),
        ))
    }

    fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_newtype_struct()");
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_newtype_variant()");
        value.serialize(self)
    }

    fn serialize_none(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_none()");
        let val = self.get_current_field()?.serialize_none()?;
        self.push(val);
        Ok(())
    }

    fn serialize_some<T: ?Sized + serde::Serialize>(
        self,
        value: &T,
    ) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_some()");
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> SerializationResult<Self::SerializeSeq> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_seq()");
        Ok(Compound { ser: self })
    }

    fn serialize_tuple(self, _len: usize) -> SerializationResult<Self::SerializeTuple> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_tuple()");
        Ok(Compound { ser: self })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerializationResult<Self::SerializeTupleStruct> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_tuple_struct()");
        Ok(Compound { ser: self })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerializationResult<Self::SerializeTupleVariant> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_tuple_variant()");
        Ok(Compound { ser: self })
    }

    fn serialize_map(self, _len: Option<usize>) -> SerializationResult<Self::SerializeMap> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_map()");
        Ok(Compound { ser: self })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerializationResult<Self::SerializeStruct> {
        #[cfg(feature = "trace")]
        trace!("Serializer::serialize_struct()");
        Ok(Compound { ser: self })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerializationResult<Self::SerializeStructVariant> {
        Err(SerializationError::StructuralMismatch(
            "serialize_struct_variant() not implemented",
        ))
    }
}

#[cfg(feature = "trace")]
fn head(count: usize, s: &str) -> String {
    let head: String = s.chars().take(count).collect();
    head
}
#[cfg(feature = "trace")]
fn tail(count: usize, s: &str) -> String {
    let rev_tail: String = s.chars().rev().take(count).collect();
    let tail: String = rev_tail.chars().rev().collect();
    tail
}

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct Compound<'a, 'm: 'a, DF: 'm + DbvFactory> {
    ser: &'a mut Serializer<'m, DF>,
}

impl<'a, 'm, DF: 'm + DbvFactory> serde::ser::SerializeSeq for Compound<'a, 'm, DF> {
    type Ok = ();
    type Error = SerializationError;

    fn serialize_element<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeSeq::serialize_element()");
        let t: &mut Serializer<DF> = self.ser;
        value.serialize(t)
    }

    fn end(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeSeq::end()");
        Ok(())
    }
}

impl<'a, 'm, DF: 'm + DbvFactory> serde::ser::SerializeTuple for Compound<'a, 'm, DF> {
    type Ok = ();
    type Error = SerializationError;

    fn serialize_element<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeTuple::serialize_element()");
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeTuple::end()");
        Ok(())
    }
}

impl<'a, 'm, DF: 'm + DbvFactory> serde::ser::SerializeTupleStruct for Compound<'a, 'm, DF> {
    type Ok = ();
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeTupleStruct::serialize_field()");
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeTupleStruct::end()");
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a, 'm, DF: 'm + DbvFactory> serde::ser::SerializeTupleVariant for Compound<'a, 'm, DF> {
    type Ok = ();
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeTupleVariant::serialize_field()");
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeTupleVariant::end()");
        Ok(())
    }
}

impl<'a, 'm, DF: 'm + DbvFactory> serde::ser::SerializeMap for Compound<'a, 'm, DF> {
    type Ok = ();
    type Error = SerializationError;

    fn serialize_key<T: ?Sized + serde::Serialize>(&mut self, _key: &T) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeMap::serialize_key()");
        Ok(())
    }

    fn serialize_value<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeMap::serialize_value()");
        let t: &mut Serializer<DF> = self.ser;
        value.serialize(t)
    }

    fn end(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeMap::end()");
        Ok(())
    }
}

impl<'a, 'm, DF: 'm + DbvFactory> serde::ser::SerializeStruct for Compound<'a, 'm, DF> {
    type Ok = ();
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeStruct::serialize_field()");
        serde::ser::SerializeMap::serialize_key(self, key)?;
        serde::ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeStruct::end()");
        serde::ser::SerializeMap::end(self)
    }
}

impl<'a, 'm, DF: 'm + DbvFactory> serde::ser::SerializeStructVariant for Compound<'a, 'm, DF> {
    type Ok = ();
    type Error = SerializationError;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> SerializationResult<()> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeStructVariant::serialize_field()");
        serde::ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> SerializationResult<Self::Ok> {
        #[cfg(feature = "trace")]
        trace!("Compound: SerializeStructVariant::end()");
        serde::ser::SerializeStruct::end(self)
    }
}
