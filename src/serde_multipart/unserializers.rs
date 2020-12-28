mod bytes;
mod input_file;
mod string;

pub(crate) use input_file::InputFileUnserializer;
pub(crate) use string::StringUnserializer;

use std::fmt::{self, Display};

use serde::ser;

#[derive(Debug, PartialEq, Eq)]
pub enum UnserializerError {
    Custom(String),
    UnsupportedType {
        ty: &'static str,
        supported: &'static str,
    },
    UnexpectedField {
        name: &'static str,
        expected: &'static [&'static str],
    },
    UnexpectedVariant {
        name: &'static str,
        expected: &'static [&'static str],
    },
    WrongLen {
        len: usize,
        expected: usize,
    },
}

impl ser::Error for UnserializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl Display for UnserializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedField { name, expected } => write!(
                f,
                "Unexpected field: `{}`, expected field(s): `{}`",
                name,
                expected.join(", ")
            ),
            Self::Custom(s) => write!(f, "Custom serde error: {}", s),
            Self::UnsupportedType { ty, supported } => {
                write!(
                    f,
                    "Unsupported type: `{}`, supported type(s): `{}`",
                    ty, supported
                )
            }
            Self::UnexpectedVariant { name, expected } => write!(
                f,
                "Unexpected variant: `{}`, expected variants(s): `{}`",
                name,
                expected.join(", ")
            ),
            Self::WrongLen { len, expected } => {
                write!(f, "Wrong len: `{}`, expected `{}`", len, expected)
            }
        }
    }
}

impl std::error::Error for UnserializerError {}

#[test]
fn test() {
    use crate::serde_multipart::unserializers::string::StringUnserializer;
    use serde::Serialize;

    use crate::{
        serde_multipart::unserializers::input_file::InputFileUnserializer, types::InputFile,
    };
    use std::borrow::Cow;

    let value = String::from("test");
    assert_eq!(value.serialize(StringUnserializer), Ok(value));

    let value = InputFile::Url(String::from("url"));
    assert_eq!(value.serialize(InputFileUnserializer::NotMem), Ok(value));

    let value = InputFile::FileId(String::from("file_id"));
    assert_eq!(value.serialize(InputFileUnserializer::NotMem), Ok(value));

    let value = InputFile::Memory {
        file_name: String::from("name"),
        data: Cow::Owned(vec![1, 2, 3]),
    };
    assert_eq!(value.serialize(InputFileUnserializer::memory()), Ok(value));

    let value = InputFile::File("a/b/c".into());
    assert_eq!(value.serialize(InputFileUnserializer::NotMem), Ok(value));
}
