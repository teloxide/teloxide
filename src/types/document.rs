use serde::{Deserialize, Serialize};

use crate::types::PhotoSize;

/// This object represents a general file (as opposed to [photos], [voice
/// messages] and [audio files]).
///
/// [The official docs](https://core.telegram.org/bots/api#document).
///
/// [photos]: https://core.telegram.org/bots/api#photosize
/// [voice messages]: https://core.telegram.org/bots/api#voice
/// [audio files]: https://core.telegram.org/bots/api#audio
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Document {
    /// An identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// A document thumbnail as defined by a sender.
    pub thumb: Option<PhotoSize>,

    /// An original filename as defined by a sender.
    pub file_name: Option<String>,

    /// A MIME type of the file as defined by a sender.
    pub mime_type: Option<String>,

    /// A size of a file.
    pub file_size: Option<u32>,
}
