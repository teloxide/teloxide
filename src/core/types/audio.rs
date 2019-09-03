use crate::core::types::PhotoSize;


#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Audio {
    pub file_id: String,
    pub duration: u32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u32>,
    pub thumb: Option<PhotoSize>
}