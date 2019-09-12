use crate::core::types::PhotoSize;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct UserProfilePhotos {
    pub total_count: u32,
    pub photos: Vec<Vec<PhotoSize>>
}
