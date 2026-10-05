use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Seconds};

/// This object represents a live photo.
///
/// [The official docs](https://core.telegram.org/bots/api#livephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LivePhoto {
    /// Available sizes of the corresponding static photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// Identifier for the video file, which can be used to download or reuse
    /// the file.
    pub file_id: String,

    /// Unique identifier for the video file.
    pub file_unique_id: String,

    /// Video width as defined by the sender.
    pub width: u32,

    /// Video height as defined by the sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by the sender.
    pub duration: Seconds,

    /// MIME type of the file as defined by the sender.
    pub mime_type: Option<String>,

    /// File size in bytes.
    pub file_size: Option<u64>,
}
