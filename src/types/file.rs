use serde::{Deserialize, Serialize};

/// This object represents a file ready to be downloaded.
///
/// The file can be downloaded via the [`Bot::download_file(file_path, dst)`]
/// method. It is guaranteed that the path from [`GetFile`] will be valid for at
/// least 1 hour. When the path expires, a new one can be requested by calling
/// [`GetFile`].
///
/// [The official docs](https://core.telegram.org/bots/api#file).
///
/// [`GetFile`]: crate::payloads::GetFile
/// [`Bot::download_file(file_path, dst)`]: crate::net::Download::download_file
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// File size, if known.
    pub file_size: u32,

    /// File path. Use [`Bot::download_file(file_path, dst)`] to get the file.
    ///
    /// [`Bot::download_file(file_path, dst)`]:
    /// crate::net::Download::download_file
    pub file_path: String,
}
