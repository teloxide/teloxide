use serde::Serialize;

use crate::types::{InputFile, MaskPosition};

/// This object describes a sticker to be added to a sticker set.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputSticker {
    /// The added sticker. Pass a file_id as a String to send a file that
    /// already exists on the Telegram servers, pass an HTTP URL as a String
    /// for Telegram to get a file from the Internet, upload a new one using
    /// multipart/form-data, or pass “attach://<file_attach_name>” to upload a
    /// new one using multipart/form-data under <file_attach_name> name.
    /// Animated and video stickers can't be uploaded via HTTP URL.
    ///
    /// More information on Sending Files <https://core.telegram.org/bots/api#sending-files>
    pub sticker: InputFile,

    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,

    /// Position where the mask should be placed on faces. For “mask” stickers
    /// only.
    pub mask_position: Option<MaskPosition>,

    /// List of 0-20 search keywords for the sticker with total length of up to
    /// 64 characters. For “regular” and “custom_emoji” stickers only.
    pub keywords: Vec<String>,
}
