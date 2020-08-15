use crate::serde_multipart::unserializers::UnserializerError;
use serde::{ser::Impossible, Serialize, Serializer};

pub(crate) struct StringUnserializer;

impl Serializer for StringUnserializer {
    type Ok = String;
    type Error = UnserializerError;

    type SerializeSeq = Impossible<String, UnserializerError>;
    type SerializeTuple = Impossible<String, UnserializerError>;
    type SerializeTupleStruct = Impossible<String, UnserializerError>;
    type SerializeTupleVariant = Impossible<String, UnserializerError>;
    type SerializeMap = Impossible<String, UnserializerError>;
    type SerializeStruct = Impossible<String, UnserializerError>;
    type SerializeStructVariant = Impossible<String, UnserializerError>;

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_owned())
    }

    forward_to_unsuported_ty! {
        supported: "&str, String";
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
            serialize_bytes &[u8]
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
