use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, PhotoSize};

/// This object represents a general file (as opposed to [photos], [voice
/// messages] and [audio files]).
///
/// [The official docs](https://core.telegram.org/bots/api#document).
///
/// [photos]: https://core.telegram.org/bots/api#photosize
/// [voice messages]: https://core.telegram.org/bots/api#voice
/// [audio files]: https://core.telegram.org/bots/api#audio
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Document {
    /// Metadata of the document file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// A document thumbnail as defined by a sender.
    pub thumbnail: Option<PhotoSize>,

    /// An original filename as defined by a sender.
    pub file_name: Option<String>,

    /// A MIME type of the file as defined by a sender.
    #[serde(default, with = "crate::types::non_telegram_types::mime::opt_deser")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub mime_type: Option<Mime>,
}
