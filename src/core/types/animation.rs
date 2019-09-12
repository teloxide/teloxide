use crate::core::types::PhotoSize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Animation {
    pub file_id: String,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumb: PhotoSize,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u32>
}