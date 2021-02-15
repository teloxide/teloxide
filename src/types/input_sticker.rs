use serde::Serialize;

use crate::types::InputFile;

/// Sticker file that may be uploaded to telegram.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum InputSticker {
    /// PNG image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    #[serde(rename = "png_sticker")]
    Png(InputFile),

    /// TGS animation with the sticker, uploaded using multipart/form-data.
    ///
    /// See <https://core.telegram.org/animated_stickers#technical-requirements> for technical requirements
    #[serde(rename = "tgs_sticker")]
    Tgs(InputFile),
}
