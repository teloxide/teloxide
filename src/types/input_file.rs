use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum InputFile {
    File(PathBuf),
    Url(String),
    FileId(String),
}

impl InputFile {
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

impl serde::Serialize for InputFile {
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
