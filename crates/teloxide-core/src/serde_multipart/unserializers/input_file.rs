use std::borrow::Cow;

use serde::{
    ser::{Impossible, SerializeStructVariant},
    Serialize, Serializer,
};

use crate::{
    serde_multipart::unserializers::{
        bytes::BytesUnserializer, string::StringUnserializer, UnserializerError,
    },
    types::InputFile,
};

pub(crate) enum InputFileUnserializer {
    Memory {
        file_name: String,
        data: Cow<'static, [u8]>,
    },
    NotMem,
}

impl InputFileUnserializer {
    pub(crate) fn memory() -> Self {
        Self::Memory {
            file_name: String::new(),
            data: Cow::Borrowed(&[]),
        }
    }
}

impl Serializer for InputFileUnserializer {
    type Ok = InputFile;
    type Error = UnserializerError;

    type SerializeSeq = Impossible<InputFile, UnserializerError>;
    type SerializeTuple = Impossible<InputFile, UnserializerError>;
    type SerializeTupleStruct = Impossible<InputFile, UnserializerError>;
    type SerializeTupleVariant = Impossible<InputFile, UnserializerError>;
    type SerializeMap = Impossible<InputFile, UnserializerError>;
    type SerializeStruct = Impossible<InputFile, UnserializerError>;

    type SerializeStructVariant = Self;

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        if name != "InputFile" {
            return Err(UnserializerError::UnsupportedType {
                ty: name,
                supported: "InputFile", // TODO
            });
        }

        // TODO
        match variant {
            "File" => Ok(InputFile::File(value.serialize(StringUnserializer)?.into())),
            "Url" => Ok(InputFile::Url(
                reqwest::Url::parse(&value.serialize(StringUnserializer)?).unwrap(),
            )),
            "FileId" => Ok(InputFile::FileId(value.serialize(StringUnserializer)?)),
            name => Err(UnserializerError::UnexpectedVariant {
                name,
                expected: &["File", "Url", "FileId"], // TODO
            }),
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        if name != "InputFile" {
            return Err(UnserializerError::UnsupportedType {
                ty: name,
                supported: "InputFile",
            });
        }

        if variant != "Memory" {
            return Err(UnserializerError::UnexpectedVariant {
                name: variant,
                expected: &["Memory"],
            });
        }

        if len != 2 {
            return Err(UnserializerError::WrongLen { len, expected: 2 });
        }

        Ok(self)
    }

    forward_to_unsuported_ty! {
        supported: "Newtype variant, struct variant";
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
            serialize_seq(_: Option<usize>) -> Self::SerializeSeq => "sequence"
            serialize_tuple(_: usize) -> Self::SerializeTuple => "tuple"
            serialize_tuple_struct(_: &'static str, _: usize) -> Self::SerializeTupleStruct => "tuple struct"
            serialize_tuple_variant(_: &'static str, _: u32, _: &'static str, _: usize) -> Self::SerializeTupleVariant => "tuple variant"
            serialize_map(_: Option<usize>) -> Self::SerializeMap => "map"
            serialize_struct(_: &'static str, _: usize) -> Self::SerializeStruct => "struct"
        }
    }
}

impl SerializeStructVariant for InputFileUnserializer {
    type Ok = InputFile;
    type Error = UnserializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let (file_name, data) = match self {
            Self::Memory { file_name, data } => (file_name, data),
            Self::NotMem => {
                *self = Self::memory();
                match self {
                    Self::Memory { file_name, data } => (file_name, data),
                    Self::NotMem => unreachable!(),
                }
            }
        };

        match key {
            "file_name" => *file_name = value.serialize(StringUnserializer)?,
            "data" => *data = Cow::Owned(value.serialize(BytesUnserializer::default())?),
            name => {
                return Err(UnserializerError::UnexpectedField {
                    name,
                    expected: &["file_name", "data"], // TODO
                });
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Memory { file_name, data } => Ok(InputFile::Memory { file_name, data }),
            Self::NotMem => unreachable!("struct without fields?"),
        }
    }
}
