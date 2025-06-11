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
    use crate::{
        serde_multipart::unserializers::{
            input_file::InputFileUnserializer, string::StringUnserializer,
        },
        types::InputFile,
    };

    use serde::Serialize;

    use std::{borrow::Cow, path::Path};

    let value = String::from("test");
    assert!(matches!(value.serialize(StringUnserializer), Ok(v) if v == value));

    let url = reqwest::Url::parse("http://example.com").unwrap();
    let value = InputFile::Url(url.clone());
    assert!(
        matches!(value.serialize(InputFileUnserializer::NotMem), Ok(InputFile::Url(v)) if v == url)
    );

    let value = InputFile::FileId("file_id".into());
    assert!(
        matches!(value.serialize(InputFileUnserializer::NotMem), Ok(InputFile::FileId(v.into())) if v == "file_id")
    );

    let value = InputFile::Memory {
        file_name: String::from("name"),
        data: Cow::Owned(vec![1, 2, 3]),
    };
    assert!(
        matches!(value.serialize(InputFileUnserializer::memory()), Ok(InputFile::Memory { file_name, data }) if file_name == "name" && *data == [1, 2, 3])
    );

    let value = InputFile::File("a/b/c".into());
    assert!(
        matches!(value.serialize(InputFileUnserializer::NotMem), Ok(InputFile::File(v)) if v == Path::new("a/b/c"))
    );
}
