use crate::core::types::{PhotoSize, UnsignedInteger};


#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<UnsignedInteger>,
}
