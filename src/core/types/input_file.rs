#[derive(Debug, Hash, PartialEq, Eq)]
pub enum InputFile {
    File(std::path::PathBuf),
    Url(String),
    FileId(String),
}

impl serde::Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            InputFile::File(path) => {
                // NOTE: file should be actually attached with multipart/form-data
                serializer.serialize_str(
                    // TODO: remove unwrap (?)
                    &format!("attach://{}", path.file_name().unwrap().to_string_lossy())
                )
            },
            InputFile::Url(url) => serializer.serialize_str(url),
            InputFile::FileId(id) => serializer.serialize_str(id),
        }
    }

}
