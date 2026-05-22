use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Document, LivePhoto, Location, PhotoSize, Sticker, Venue, Video,
};

/// This object describes media attached to a poll description, quiz
/// explanation, or poll option.
///
/// [The official docs](https://core.telegram.org/bots/api#pollmedia).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollMedia {
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub live_photo: Option<LivePhoto>,
    pub location: Option<Location>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub venue: Option<Venue>,
    pub video: Option<Video>,
}

/// Content of a poll description or quiz explanation to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpollmedia).
#[derive(Clone, Debug)]
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollMedia {
    Animation(crate::types::InputMediaAnimation),
    Audio(crate::types::InputMediaAudio),
    Document(crate::types::InputMediaDocument),
    LivePhoto(crate::types::InputMediaLivePhoto),
    Location(crate::types::InputMediaLocation),
    Photo(crate::types::InputMediaPhoto),
    Venue(crate::types::InputMediaVenue),
    Video(crate::types::InputMediaVideo),
}

/// Content of a poll option to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloptionmedia).
#[derive(Clone, Debug)]
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum InputPollOptionMedia {
    Photo(crate::types::InputMediaPhoto),
    Sticker(crate::types::InputMediaSticker),
    Video(crate::types::InputMediaVideo),
}
