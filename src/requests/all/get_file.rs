use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::File,
    Bot,
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
#[derive(Debug, Clone, Serialize)]
pub struct GetFile<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// File identifier to get info about
    pub file_id: String,
}

#[async_trait::async_trait]
impl Request for GetFile<'_> {
    type Output = File;

    async fn send(&self) -> ResponseResult<File> {
        net::request_json(self.bot.client(), self.bot.token(), "getFile", &self)
            .await
    }
}

impl<'a> GetFile<'a> {
    pub(crate) fn new<F>(bot: &'a Bot, file_id: F) -> Self
    where
        F: Into<String>,
    {
        Self {
            bot,
            file_id: file_id.into(),
        }
    }

    pub fn file_id<F>(mut self, value: F) -> Self
    where
        F: Into<String>,
    {
        self.file_id = value.into();
        self
    }
}
