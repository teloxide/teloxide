use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Document, InputFile, LivePhoto, Location, PhotoSize, Sticker, Venue, Video,
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
    pub link: Option<Link>,
    pub live_photo: Option<LivePhoto>,
    pub location: Option<Location>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub venue: Option<Venue>,
    pub video: Option<Video>,
}

/// This object represents an HTTP link attached to a poll option.
///
/// [The official docs](https://core.telegram.org/bots/api#link).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Link {
    /// URL of the link.
    pub url: String,
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
    Animation(crate::types::InputMediaAnimation),
    Link(crate::types::InputMediaLink),
    LivePhoto(crate::types::InputMediaLivePhoto),
    Location(crate::types::InputMediaLocation),
    Photo(crate::types::InputMediaPhoto),
    Sticker(crate::types::InputMediaSticker),
    Venue(crate::types::InputMediaVenue),
    Video(crate::types::InputMediaVideo),
}

impl InputPollMedia {
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        let mut files = Vec::new();

        match self {
            Self::Animation(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
            }
            Self::Audio(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
            }
            Self::Document(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
            }
            Self::LivePhoto(media) => {
                files.push(&media.media);
                files.push(&media.photo);
            }
            Self::Photo(media) => files.push(&media.media),
            Self::Video(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
                files.extend(media.cover.iter());
            }
            Self::Location(_) | Self::Venue(_) => {}
        }

        files.into_iter()
    }

    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        let mut files = Vec::new();

        match self {
            Self::Animation(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
            }
            Self::Audio(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
            }
            Self::Document(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
            }
            Self::LivePhoto(media) => {
                files.push(&mut media.media);
                files.push(&mut media.photo);
            }
            Self::Photo(media) => files.push(&mut media.media),
            Self::Video(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
                files.extend(media.cover.iter_mut());
            }
            Self::Location(_) | Self::Venue(_) => {}
        }

        files.into_iter()
    }
}

impl InputPollOptionMedia {
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        let mut files = Vec::new();

        match self {
            Self::Animation(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
            }
            Self::LivePhoto(media) => {
                files.push(&media.media);
                files.push(&media.photo);
            }
            Self::Photo(media) => files.push(&media.media),
            Self::Sticker(media) => files.push(&media.media),
            Self::Video(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
                files.extend(media.cover.iter());
            }
            Self::Link(_) | Self::Location(_) | Self::Venue(_) => {}
        }

        files.into_iter()
    }

    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        let mut files = Vec::new();

        match self {
            Self::Animation(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
            }
            Self::LivePhoto(media) => {
                files.push(&mut media.media);
                files.push(&mut media.photo);
            }
            Self::Photo(media) => files.push(&mut media.media),
            Self::Sticker(media) => files.push(&mut media.media),
            Self::Video(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
                files.extend(media.cover.iter_mut());
            }
            Self::Link(_) | Self::Location(_) | Self::Venue(_) => {}
        }

        files.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_poll_option_link_serializes() {
        let media =
            InputPollOptionMedia::Link(crate::types::InputMediaLink::new("https://example.com"));

        assert_eq!(
            serde_json::to_value(media).unwrap(),
            serde_json::json!({"type": "link", "url": "https://example.com"})
        );
    }
}
