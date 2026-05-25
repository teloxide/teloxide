use serde::Serialize;

use crate::types::InputFile;

/// This object describes the content of a story to post.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum InputStoryContent {
    Photo(InputStoryContentPhoto),
    Video(InputStoryContentVideo),
}

/// Describes a regular gift owned by a user or a chat.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputStoryContentPhoto {
    /// The photo to post as a story. The photo must be of the size 1080x1920
    /// and must not exceed 10 MB. The photo can't be reused and can only be
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>” if
    /// the photo was uploaded using multipart/form-data under
    /// <file_attach_name>. [More information on Sending Files »]
    ///
    /// [More information on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub photo: InputFile,
}

/// Describes a unique gift received and owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputStoryContentVideo {
    /// The video to post as a story. The video must be of the size 720x1280,
    /// streamable, encoded with H.265 codec, with key frames added each second
    /// in the MPEG4 format, and must not exceed 30 MB. The video can't be
    /// reused and can only be uploaded as a new file, so you can pass
    /// “attach://<file_attach_name>” if the video was uploaded using
    /// multipart/form-data under <file_attach_name>. [More information on
    /// Sending Files »]
    ///
    /// [More information on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub video: InputFile,

    /// Precise duration of the video in seconds; 0-60
    pub duration: Option<f64>,

    /// Timestamp in seconds of the frame that will be used as the static cover
    /// for the story. Defaults to 0.0
    pub cover_frame_timestamp: Option<f64>,

    /// Pass _true_ if the video has no sound
    pub is_animation: Option<bool>,
}
