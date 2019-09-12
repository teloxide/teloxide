use crate::core::types::PhotoSize;

/// This object represents a video file.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Clone)]
pub struct Video {
    /// Identifier for this file
    pub file_id: String,
    /// Video width as defined by sender
    pub width: u32,
    /// Video height as defined by sender
    pub height: u32,
    /// Duration of the video in seconds as defined by sender
    pub duration: u32,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<u32>,
}
