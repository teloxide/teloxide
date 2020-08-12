use serde::{Deserialize, Serialize};

/// This object represents a chat photo.
///
/// [The official docs](https://core.telegram.org/bots/api#chatphoto).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl ChatPhoto {
    pub fn new<S1, S2, S3, S4>(
        small_file_id: S1,
        small_file_unique_id: S2,
        big_file_id: S3,
        big_file_unique_id: S4,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            small_file_id: small_file_id.into(),
            small_file_unique_id: small_file_unique_id.into(),
            big_file_id: big_file_id.into(),
            big_file_unique_id: big_file_unique_id.into(),
        }
    }

    pub fn small_file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.small_file_id = val.into();
        self
    }

    pub fn small_file_unique_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.small_file_unique_id = val.into();
        self
    }

    pub fn big_file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.big_file_id = val.into();
        self
    }

    pub fn big_file_unique_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.big_file_unique_id = val.into();
        self
    }
}
