use std::ops::Deref;

use derive_more::derive::From;
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
    /// Metadata of the file.
    ///
    /// Note that [`FileMeta`]'s fields are available on `File` too (via deref
    /// coercion).
    #[serde(flatten)]
    pub meta: FileMeta,

    /// File path. Use [`Bot::download_file(file_path, dst)`] to get the file.
    ///
    /// [`Bot::download_file(file_path, dst)`]: crate::net::Download::download_file
    #[serde(rename = "file_path")]
    pub path: String,
}

/// Identifier for a file
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[serde(transparent)]
#[from(&'static str)]
pub struct FileId(pub String);

/// Unique identifier for a file, which is supposed to be the same over
/// time and for different bots. Can't be used to download or reuse the
/// file.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[serde(transparent)]
#[from(&'static str)]
pub struct FileUniqueId(pub String);

/// Metadata of a [`File`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct FileMeta {
    /// Identifier for this file.
    #[serde(rename = "file_id")]
    pub id: FileId,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    #[serde(rename = "file_unique_id")]
    pub unique_id: FileUniqueId,

    /// File size in bytes.
    #[serde(rename = "file_size")]
    // This fallback should never be necessary in practice,
    // but just in case something goes wrong with the TBA server
    // (see the test below)
    #[serde(default = "file_size_fallback")]
    pub size: u32,
}

pub(crate) const fn file_size_fallback() -> u32 {
    u32::MAX
}

/// Allows access to [`FileMeta`]'s fields for [`File`].
///
/// ## Examples
///
/// ```rust
/// use teloxide_core::types::{File, FileId, FileUniqueId};
/// #
/// # let get_file = || File { meta: teloxide_core::types::FileMeta { id: FileId(String::new()), unique_id: FileUniqueId(String::new()), size: 0 }, path: String::new() };
/// let file: File = get_file();
///
/// let file_id: &FileId = &file.id;
/// let file_unique_id: &FileUniqueId = &file.unique_id;
/// let file_size: u32 = file.size;
/// #
/// # let _ = (file_id, file_unique_id, file_size);
/// ```
impl Deref for File {
    type Target = FileMeta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{File, FileId, FileMeta, FileUniqueId};

    // As per <https://github.com/tdlib/telegram-bot-api/issues/192> file size is **not** optional,
    // But <https://github.com/tdlib/telegram-bot-api/issues/294> suggests that it can be missing in case Telegram servers are going insane.
    // To be safe, we use a placeholder value.
    #[test]
    fn no_file_size() {
        let json =
            r#"{"file_id":"FILE_ID","file_unique_id":"FILE_UNIQUE_ID","file_path":"FILE_PATH"}"#;
        let file: File = serde_json::from_str(json).unwrap();

        assert_eq!(
            file,
            File {
                meta: FileMeta {
                    id: FileId("FILE_ID".to_owned()),
                    unique_id: FileUniqueId("FILE_UNIQUE_ID".to_owned()),
                    size: u32::MAX,
                },
                path: "FILE_PATH".to_owned(),
            }
        );
    }

    // In some places file metadata w/o path is returned. Make sure that we can
    // deserialize it.
    #[test]
    fn no_file_path() {
        let json = r#"{"file_id":"FILE_ID","file_unique_id":"FILE_UNIQUE_ID","file_size":42}"#;
        let file: FileMeta = serde_json::from_str(json).unwrap();

        assert_eq!(
            file,
            FileMeta {
                id: FileId("FILE_ID".to_owned()),
                unique_id: FileUniqueId("FILE_UNIQUE_ID".to_owned()),
                size: 42
            }
        );
    }

    #[test]
    fn full_file() {
        let json = r#"{"file_id":"FILE_ID","file_unique_id":"FILE_UNIQUE_ID","file_size":42,"file_path":"FILE_PATH"}"#;
        let file: File = serde_json::from_str(json).unwrap();

        assert_eq!(
            file,
            File {
                meta: FileMeta {
                    id: FileId("FILE_ID".to_owned()),
                    unique_id: FileUniqueId("FILE_UNIQUE_ID".to_owned()),
                    size: 42,
                },
                path: "FILE_PATH".to_owned(),
            }
        );
    }
}
