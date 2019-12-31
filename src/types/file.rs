use serde::{Deserialize, Serialize};

/// This object represents a file ready to be downloaded. The file can be
/// downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour. When the
/// link expires, a new one can be requested by calling [`Bot::get_file`].
///
/// [The official docs](https://core.telegram.org/bots/api#file).
///
/// [`Bot::get_file`]: crate::Bot::get_file
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct File {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// File size, if known.
    pub file_size: u32,

    // TODO: chacge "Use ..." to use bot.download...
    /// File path. Use `https://api.telegram.org/file/bot<token>/<file_path>`
    /// to get the file.
    pub file_path: String,
}
