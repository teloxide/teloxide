use crate::types::InputFile;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum StickerType {
    /// PNG image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px.
    ///
    /// Pass [`InputFile::FileId`] to send a sticker that exists on the Telegram
    /// servers (recommended), pass an [`InputFile::Url`] for Telegram to get a
    /// sticker (.WEBP file) from the Internet, pass [`InputFile::File`] to
    /// upload a sticker from the file system or [`InputFile::Memory`] to upload
    /// a sticker from memory [More info on Sending Files »].
    ///
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Memory`]: crate::types::InputFile::Memory
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    Png(InputFile),

    /// TGS animation with the sticker, uploaded using multipart/form-data.
    ///
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    Tgs(InputFile),
}
