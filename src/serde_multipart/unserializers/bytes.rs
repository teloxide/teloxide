use crate::serde_multipart::unserializers::UnserializerError;
use serde::{
    ser::{Impossible, SerializeSeq},
    Serialize, Serializer,
};

#[derive(Default)]
pub(crate) struct BytesUnserializer(Vec<u8>);

impl Serializer for BytesUnserializer {
    type Ok = Vec<u8>;
    type Error = UnserializerError;

    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Vec<u8>, UnserializerError>;
    type SerializeTupleStruct = Impossible<Vec<u8>, UnserializerError>;
    type SerializeTupleVariant = Impossible<Vec<u8>, UnserializerError>;
    type SerializeMap = Impossible<Vec<u8>, UnserializerError>;
    type SerializeStruct = Impossible<Vec<u8>, UnserializerError>;
    type SerializeStructVariant = Impossible<Vec<u8>, UnserializerError>;

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_owned())
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            self.0.reserve_exact(len);
        }
        Ok(self)
    }

    forward_to_unsuported_ty! {
        supported: "&[u8], Vec<u8>, Cow<[u8]>";
        simple {
            serialize_bool  bool
            serialize_i8    i8
            serialize_i16   i16
            serialize_i32   i32
            serialize_i64   i64
            serialize_u8    u8
            serialize_u16   u16
            serialize_u32   u32
            serialize_u64   u64
            serialize_f32   f32
            serialize_f64   f64
            serialize_char  char
            serialize_str   &str
        }
        unit {
            serialize_none "None"
            serialize_unit "unit"
        }
        compound {
            serialize_some<T: ?Sized + Serialize>(_: &T) -> Self::Ok => "Some(_)"
            serialize_unit_struct(_: &'static str) -> Self::Ok => "unit struct"
            serialize_unit_variant(_: &'static str, _: u32, _: &'static str) -> Self::Ok => "unit variant"
            serialize_newtype_struct<T: ?Sized + Serialize>(_: &'static str, _: &T) -> Self::Ok => "newtype struct"
            serialize_newtype_variant<T: ?Sized + Serialize>(_: &'static str, _: u32, _: &'static str, _: &T) -> Self::Ok => "newtype variant"
            serialize_tuple(_: usize) -> Self::SerializeTuple => "tuple"
            serialize_tuple_struct(_: &'static str, _: usize) -> Self::SerializeTupleStruct => "tuple struct"
            serialize_tuple_variant(_: &'static str, _: u32, _: &'static str, _: usize) -> Self::SerializeTupleVariant => "tuple variant"
            serialize_map(_: Option<usize>) -> Self::SerializeMap => "map"
            serialize_struct(_: &'static str, _: usize) -> Self::SerializeStruct => "struct"
            serialize_struct_variant(_: &'static str, _: u32, _: &'static str, _: usize) -> Self::SerializeStructVariant => "struct variant"
        }
    }
}

impl SerializeSeq for BytesUnserializer {
    type Ok = Vec<u8>;
    type Error = UnserializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(BytesUnserializerPush(&mut self.0))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.0)
    }
}

pub(crate) struct BytesUnserializerPush<'a>(&'a mut Vec<u8>);

impl Serializer for BytesUnserializerPush<'_> {
    type Ok = ();
    type Error = UnserializerError;

    type SerializeSeq = Impossible<(), UnserializerError>;
    type SerializeTuple = Impossible<(), UnserializerError>;
    type SerializeTupleStruct = Impossible<(), UnserializerError>;
    type SerializeTupleVariant = Impossible<(), UnserializerError>;
    type SerializeMap = Impossible<(), UnserializerError>;
    type SerializeStruct = Impossible<(), UnserializerError>;
    type SerializeStructVariant = Impossible<(), UnserializerError>;

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.0.push(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.extend_from_slice(v);
        Ok(())
    }

    forward_to_unsuported_ty! {
        supported: "&[u8], Vec<u8>, Cow<[u8]>";
        simple {
            serialize_bool  bool
            serialize_i8    i8
            serialize_i16   i16
            serialize_i32   i32
            serialize_i64   i64
            serialize_u16   u16
            serialize_u32   u32
            serialize_u64   u64
            serialize_f32   f32
            serialize_f64   f64
            serialize_char  char
            serialize_str   &str
        }
        unit {
            serialize_none "None"
            serialize_unit "unit"
        }
        compound {
            serialize_some<T: ?Sized + Serialize>(_: &T) -> Self::Ok => "Some(_)"
            serialize_unit_struct(_: &'static str) -> Self::Ok => "unit struct"
            serialize_unit_variant(_: &'static str, _: u32, _: &'static str) -> Self::Ok => "unit variant"
            serialize_newtype_struct<T: ?Sized + Serialize>(_: &'static str, _: &T) -> Self::Ok => "newtype struct"
            serialize_newtype_variant<T: ?Sized + Serialize>(_: &'static str, _: u32, _: &'static str, _: &T) -> Self::Ok => "newtype variant"
            serialize_seq(_: Option<usize>) -> Self::SerializeSeq => "sequence"
            serialize_tuple(_: usize) -> Self::SerializeTuple => "tuple"
            serialize_tuple_struct(_: &'static str, _: usize) -> Self::SerializeTupleStruct => "tuple struct"
            serialize_tuple_variant(_: &'static str, _: u32, _: &'static str, _: usize) -> Self::SerializeTupleVariant => "tuple variant"
            serialize_map(_: Option<usize>) -> Self::SerializeMap => "map"
            serialize_struct(_: &'static str, _: usize) -> Self::SerializeStruct => "struct"
            serialize_struct_variant(_: &'static str, _: u32, _: &'static str, _: usize) -> Self::SerializeStructVariant => "struct variant"
        }
    }
}
