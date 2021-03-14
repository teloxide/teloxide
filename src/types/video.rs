use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::PhotoSize;

/// This object represents a video file.
///
/// [The official docs](https://core.telegram.org/bots/api#video).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Video {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// Video width as defined by sender.
    pub width: u32,

    /// Video height as defined by sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by sender.
    pub duration: u32,

    /// Video thumbnail.
    pub thumb: Option<PhotoSize>,

    /// Original filename as defined by sender
    pub file_name: Option<String>,

    /// Mime type of a file as defined by sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    pub mime_type: Option<Mime>,

    /// File size.
    pub file_size: Option<u32>,
}
