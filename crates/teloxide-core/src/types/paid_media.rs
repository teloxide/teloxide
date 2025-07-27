use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Seconds, Video};

/// Describes the paid media added to a message.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediainfo).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the
    /// media.
    pub star_count: u32,

    /// Information about the paid media.
    pub paid_media: Vec<PaidMedia>,
}

/// This object describes paid media.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmedia).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(Box<PaidMediaVideo>),
}

/// The paid media isn't available before the payment.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediapreview).
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender.
    pub width: Option<u32>,

    /// Media height as defined by the sender.
    pub height: Option<u32>,

    /// Duration of the media in seconds as defined by the sender.
    pub duration: Option<Seconds>,
}

/// The paid media is a photo.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediaphoto).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaPhoto {
    pub photo: PhotoSize,
}

/// The paid media is a video.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediavideo).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaVideo {
    pub video: Video,
}

impl PaidMedia {
    pub fn preview(&self) -> Option<PaidMediaPreview> {
        match self {
            Self::Preview(preview) => Some(*preview),
            _ => None,
        }
    }

    pub fn photo(&self) -> Option<&PhotoSize> {
        match self {
            Self::Photo(photo) => Some(&photo.photo),
            _ => None,
        }
    }

    pub fn video(&self) -> Option<&Video> {
        match self {
            Self::Video(video) => Some(&video.video),
            _ => None,
        }
    }
}
