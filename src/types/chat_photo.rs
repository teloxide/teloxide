use serde::{Deserialize, Serialize};

/// This object represents a chat photo.
///
/// [The official docs](https://core.telegram.org/bots/api#chatphoto).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChatPhoto {
    /// A file identifier of small (160x160) chat photo. This file_id can be
    /// used only for photo download and only for as long as the photo is
    /// not changed.
    pub small_file_id: String,

    /// Unique file identifier of small (160x160) chat photo, which is supposed
    /// to be the same over time and for different bots. Can't be used to
    /// download or reuse the file.
    pub small_file_unique_id: String,

    /// A file identifier of big (640x640) chat photo. This file_id can be used
    /// only for photo download and only for as long as the photo is not
    /// changed.
    pub big_file_id: String,

    /// Unique file identifier of big (640x640) chat photo, which is supposed
    /// to be the same over time and for different bots. Can't be used to
    /// download or reuse the file.
    pub big_file_unique_id: String,
}
