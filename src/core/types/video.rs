use crate::core::types::PhotoSize;


#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Video {
    pub file_id: String,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<u32>
}