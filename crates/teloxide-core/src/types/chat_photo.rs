use serde::{Deserialize, Serialize};

use crate::types::{FileId, FileUniqueId};

/// This object represents a chat photo.
///
/// [The official docs](https://core.telegram.org/bots/api#chatphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatPhoto {
    /// A file identifier of small (160x160) chat photo. This file_id can be
    /// used only for photo download and only for as long as the photo is
    /// not changed.
    pub small_file_id: FileId,

    /// Unique file identifier of small (160x160) chat photo, which is supposed
    /// to be the same over time and for different bots. Can't be used to
    /// download or reuse the file.
    pub small_file_unique_id: FileUniqueId,

    /// A file identifier of big (640x640) chat photo. This file_id can be used
    /// only for photo download and only for as long as the photo is not
    /// changed.
    pub big_file_id: FileId,

    /// Unique file identifier of big (640x640) chat photo, which is supposed
    /// to be the same over time and for different bots. Can't be used to
    /// download or reuse the file.
    pub big_file_unique_id: FileUniqueId,
}
