use std::borrow::Cow;
use std::path::Path;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputFile<'a> {
    File(Cow<'a, Path>),
    Url(Cow<'a, str>),
    FileId(Cow<'a, str>),
}

impl<'a> serde::Serialize for InputFile<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            InputFile::File(path) => {
                // NOTE: file should be actually attached with
                // multipart/form-data
                serializer.serialize_str(
                    // TODO: remove unwrap (?)
                    &format!(
                        "attach://{}",
                        path.file_name().unwrap().to_string_lossy()
                    ),
                )
            }
            InputFile::Url(url) => serializer.serialize_str(url),
            InputFile::FileId(id) => serializer.serialize_str(id),
        }
    }
}
