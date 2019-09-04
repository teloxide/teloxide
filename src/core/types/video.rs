use crate::core::types::{PhotoSize, UnsignedInteger};


#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Video {
    pub file_id: String,
    pub width: UnsignedInteger,
    pub height: UnsignedInteger,
    pub duration: UnsignedInteger,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<UnsignedInteger>
}