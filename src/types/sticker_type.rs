use serde::Serialize;

use crate::types::InputFile;

/// Sticker file that may be uploaded to telegram.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
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
    Png { png_sticker: InputFile },

    /// TGS animation with the sticker, uploaded using multipart/form-data.
    ///
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    Tgs { tgs_sticker: InputFile },
}

impl InputSticker {
    /// Create png-`InputSticker`.
    ///
    /// See [`InputSticker::Png`] for more
    ///
    /// [`InputSticker::Png`]: crate::types::InputSticker::Png
    pub fn png(png_sticker: InputFile) -> Self {
        Self::Png { png_sticker }
    }

    /// Create tgs-`InputSticker`.
    ///
    /// See [`InputSticker::Tgs`] for more
    ///
    /// [`InputSticker::Tgs`]: crate::types::InputSticker::Tgs
    pub fn tgs(tgs_sticker: InputFile) -> Self {
        Self::Tgs { tgs_sticker }
    }
}
