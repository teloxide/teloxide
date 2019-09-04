use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// This object represents a [video message](https://telegram.org/blog/video-messages-and-telescope)
/// (available in Telegram apps as of v.4.0).
struct VideoNote {
    /// Identifier for this file
    pub file_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: u32,
    /// Duration of the video in seconds as defined by sender
    pub duration: u32,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. File size
    pub file_size: Option<u32>,
}