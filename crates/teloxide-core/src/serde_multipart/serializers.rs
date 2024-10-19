use crate::serde_multipart::error::Error;

use reqwest::multipart::{Form, Part};
use serde::{
    ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

/// The main serializer that serializes top-level and structures
pub(super) struct MultipartSerializer(Form);

/// Serializer for maps (support for `#[serde(flatten)]`)
pub(super) struct MultipartMapSerializer {
    form: Form,
    key: Option<String>,
}

/// Serializer for single "fields" that are serialized as multipart "part"s.
///
/// - Integers serialized as their text decimal representation
/// - Strings and byte slices are serialized as-is, without any changes
/// - Structs are serialized with JSON
/// - C-like enums are serialized as their names
struct PartSerializer;

/// Struct or Seq -> Json -> Part serializer
struct JsonPartSerializer {
    buf: String,
    state: PartSerializerStructState,
}

/// State for `PartSerializerStruct`
///
/// Json doesn't allow trailing commas, so we need to know if we already
/// serialized something and need to add a comma before next field
enum PartSerializerStructState {
    Empty,
    Rest,
}

impl MultipartSerializer {
    pub(super) fn new() -> Self {
        Self(Form::new())
    }
}

impl Serializer for MultipartSerializer {
    type Ok = Form;
    type Error = Error;

    // for `serde(flatten)` (e.g.: in CreateNewStickerSet)
    type SerializeMap = MultipartMapSerializer;

    // The main serializer - struct
    type SerializeStruct = Self;

    // Unimplemented
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MultipartMapSerializer { form: Form::new(), key: None })
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    // Everything down below in this impl just returns
    // `Err(Error::TopLevelNotStruct)`

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }
}

impl SerializeStruct for MultipartSerializer {
    type Ok = Form;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let part = value.serialize(PartSerializer {})?;
        take_mut::take(&mut self.0, |f| f.part(key, part));

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.0)
    }
}

impl SerializeMap for MultipartMapSerializer {
    type Ok = Form;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if let Ok(serde_json::Value::String(s)) = serde_json::to_value(key) {
            self.key = Some(s);
        }

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = self.key.take().expect("Value serialized before key or key is not string");

        let part = value.serialize(PartSerializer {})?;

        take_mut::take(&mut self.form, |f| f.part(key, part));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.form)
    }
}

impl Serializer for PartSerializer {
    type Ok = Part;
    type Error = Error;

    type SerializeStruct = JsonPartSerializer;
    type SerializeSeq = JsonPartSerializer;

    // Unimplemented
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(v.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Part::bytes(v.to_owned()))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant_name: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(variant_name))
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(JsonPartSerializer { buf: String::new(), state: PartSerializerStructState::Empty })
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(JsonPartSerializer { buf: String::new(), state: PartSerializerStructState::Empty })
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    // Unimplemented

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!(
            "We use `#[serde_with::skip_serializing_none]` everywhere so `None`s are not \
             serialized"
        )
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

impl SerializeStruct for JsonPartSerializer {
    type Ok = Part;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        use std::fmt::Write;
        use PartSerializerStructState::*;

        let value = serde_json::to_string(value)?;
        match self.state {
            Empty => {
                self.state = Rest;

                write!(&mut self.buf, "{{\"{key}\":{value}")?
            }
            Rest => write!(&mut self.buf, ",\"{key}\":{value}")?,
        }

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        use PartSerializerStructState::*;

        match self.state {
            Empty => Ok(Part::text("{{}}")),
            Rest => {
                self.buf += "}";

                Ok(Part::text(self.buf))
            }
        }
    }
}

impl SerializeSeq for JsonPartSerializer {
    type Ok = Part;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        use std::fmt::Write;
        use PartSerializerStructState::*;

        let value = serde_json::to_string(value)?;
        match self.state {
            Empty => {
                self.state = Rest;

                write!(&mut self.buf, "[{value}")?
            }
            Rest => write!(&mut self.buf, ",{value}")?,
        }

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        use PartSerializerStructState::*;

        match self.state {
            Empty => Ok(Part::text("[]")),
            Rest => {
                self.buf += "]";

                Ok(Part::text(self.buf))
            }
        }
    }
}
