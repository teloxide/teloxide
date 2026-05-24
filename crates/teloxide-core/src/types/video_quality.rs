use serde::{Deserialize, Serialize};

/// This object represents a video file of a specific quality.
///
/// [The official docs](https://core.telegram.org/bots/api#videoquality).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the
    /// file.
    pub file_id: String,

    /// Unique identifier for this file.
    pub file_unique_id: String,

    /// Video width.
    pub width: u32,

    /// Video height.
    pub height: u32,

    /// Codec that was used to encode the video.
    pub codec: String,

    /// File size in bytes.
    pub file_size: Option<u64>,
}
