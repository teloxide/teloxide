use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::File,
};

/// Use this method to get basic info about a file and prepare it for
/// downloading.
///
/// For the moment, bots can download files of up to `20MB` in size.
///
/// On success, a [`File`] object is returned.
///
/// The file can then be downloaded via the link
/// `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>`
/// is taken from the response. It is guaranteed that the link will be valid
/// for at least `1` hour. When the link expires, a new one can be requested by
/// calling [`GetFile`] again.
///
/// **Note**: This function may not preserve the original file name and MIME
/// type. You should save the file's MIME type and name (if available) when the
/// [`File`] object is received.
///
/// [`File`]: crate::types::file
/// [`GetFile`]: self::GetFile
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

impl Method for GetFile {
    type Output = File;

    const NAME: &'static str = "getFile";
}

impl json::Payload for GetFile {}

impl dynamic::Payload for GetFile {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetFile {
    pub fn new<F>(file_id: F) -> Self
    where
        F: Into<String>,
    {
        Self {
            file_id: file_id.into(),
        }
    }
}

impl json::Request<'_, GetFile> {
    pub fn file_id<F>(mut self, value: F) -> Self
    where
        F: Into<String>,
    {
        self.payload.file_id = value.into();
        self
    }
}
