use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub enum InputFile {
    File(std::fs::File),
    Url(String),
    FileId(String),
}
