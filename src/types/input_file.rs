use serde::{Deserialize, Serialize};

use std::{borrow::Cow, path::PathBuf};

/// This object represents the contents of a file to be uploaded.
///
/// [The official docs](https://core.telegram.org/bots/api#inputfile).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InputFile {
    File(PathBuf),
    Memory {
        file_name: String,
        data: Cow<'static, [u8]>,
    },
    Url(String),
    FileId(String),
}

impl InputFile {
    pub fn file<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::File(path.into())
    }

    pub fn memory<S, D>(file_name: S, data: D) -> Self
    where
        S: Into<String>,
        D: Into<Cow<'static, [u8]>>,
    {
        Self::Memory {
            file_name: file_name.into(),
            data: data.into(),
        }
    }

    pub fn url<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self::Url(url.into())
    }

    pub fn file_id<T>(file_id: T) -> Self
    where
        T: Into<String>,
    {
        Self::FileId(file_id.into())
    }

    pub fn as_file(&self) -> Option<&PathBuf> {
        match self {
            Self::File(path) => Some(path),
            _ => None,
        }
    }

    pub fn as_url(&self) -> Option<&String> {
        match self {
            Self::Url(url) => Some(url),
            _ => None,
        }
    }

    pub fn as_file_id(&self) -> Option<&String> {
        match self {
            Self::FileId(id) => Some(id),
            _ => None,
        }
    }
}

impl From<InputFile> for Option<PathBuf> {
    fn from(file: InputFile) -> Self {
        match file {
            InputFile::File(path) => Some(path),
            _ => None,
        }
    }
}

// internal api

use reqwest::multipart::Part;

impl InputFile {
    pub(crate) async fn into_part(self) -> std::io::Result<Part> {
        use bytes::{Bytes, BytesMut};
        use reqwest::Body;
        use tokio_util::codec::{Decoder, FramedRead};

        struct FileDecoder;

        impl Decoder for FileDecoder {
            type Item = Bytes;
            type Error = std::io::Error;

            fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
                if src.is_empty() {
                    return Ok(None);
                }
                Ok(Some(src.split().freeze()))
            }
        }

        match self {
            Self::File(path_to_file) => {
                let file_name = path_to_file
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();

                let file = FramedRead::new(tokio::fs::File::open(path_to_file).await?, FileDecoder);

                Ok(Part::stream(Body::wrap_stream(file)).file_name(file_name))
            }
            Self::Memory { file_name, data } => Ok(Part::bytes(data).file_name(file_name)),
            Self::Url(s) | Self::FileId(s) => Ok(Part::text(s)),
        }
    }
}
