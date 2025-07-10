use serde::Serialize;

use crate::types::InputFile;

/// This object describes a profile photo to set.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum InputProfilePhoto {
    Static(InputProfilePhotoStatic),
    Animated(InputProfilePhotoAnimated),
}

/// A static profile photo in the .JPG format.
#[derive(Clone, Debug, Serialize)]
pub struct InputProfilePhotoStatic {
    /// The static profile photo. Profile photos can't be reused and can only be
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>” if
    /// the photo was uploaded using multipart/form-data under
    /// <file_attach_name>. [More information on Sending Files »]
    ///
    /// [More information on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub photo: InputFile,
}

/// An animated profile photo in the MPEG4 format.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputProfilePhotoAnimated {
    /// The animated profile photo. Profile photos can't be reused and can only
    /// be uploaded as a new file, so you can pass “attach://<file_attach_name>”
    /// if the photo was uploaded using multipart/form-data under
    /// <file_attach_name>. [More information on Sending Files »]
    ///
    /// [More information on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub animation: InputFile,

    /// Timestamp in seconds of the frame that will be used as the static
    /// profile photo. Defaults to 0.0
    pub main_frame_timestamp: Option<f64>,
}
