use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, PhotoSize, Seconds};

/// This object represents a video file.
///
/// [The official docs](https://core.telegram.org/bots/api#video).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Video {
    /// Metadata of the video file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Video width as defined by sender.
    pub width: u32,

    /// Video height as defined by sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by sender.
    pub duration: Seconds,

    /// Video thumbnail.
    pub thumbnail: Option<PhotoSize>,

    /// Original filename as defined by sender
    pub file_name: Option<String>,

    /// Mime type of a file as defined by sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    pub mime_type: Option<Mime>,
}
