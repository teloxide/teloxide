use serde::{Deserialize, Serialize};
use crate::types::{PhotoSize, Seconds, Video};

/// Describes the paid media added to a message.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media.
    pub star_count: u32,

    /// Information about the paid media.
    pub paid_media: Vec<PaidMedia>,
}

/// This object describes paid media.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(PaidMediaVideo),
}

/// The paid media isn't available before the payment.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender
    pub width: Option<u32>,

    /// Media height as defined by the sender
    pub height: Option<u32>,

    /// Duration of the media in seconds as defined by the sender.
    pub duration: Seconds,
}

/// The paid media is a photo.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PaidMediaPhoto {
    /// The photo.
    pub photo: Vec<PhotoSize>,
}

/// The paid media is a video.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PaidMediaVideo {
    /// The video
    pub video: Video,
}