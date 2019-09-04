use crate::core::types::{Integer, UnsignedInteger};


#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: Integer,
    pub heigth: Integer,
    pub file_size: Option<UnsignedInteger>
}
