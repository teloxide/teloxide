use crate::core::types::PhotoSize;


#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Audio {
    pub file_id: String,
    pub duration: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>
}