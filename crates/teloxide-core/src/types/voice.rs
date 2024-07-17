use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, Seconds};

/// This object represents a voice note.
///
/// [The official docs](https://core.telegram.org/bots/api#voice).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Voice {
    /// Metadata of the voice file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Duration of the audio in seconds as defined by sender.
    pub duration: Seconds,

    /// MIME type of the file as defined by sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    pub mime_type: Option<Mime>,
}
