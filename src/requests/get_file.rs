use async_trait::async_trait;

use crate::{
    network,
    requests::{Request,  ResponseResult},
    types::File,
};
use crate::bot::Bot;

/// Use this method to get basic info about a file and prepare it for
/// downloading. For the moment, bots can download files of up to 20MB in size.
/// On success, a File object is returned.
/// The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>,
/// where <file_path> is taken from the response.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling getFile again.
#[derive(Debug, Clone, Serialize)]
pub struct GetFile<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    /// File identifier to get info about
    pub file_id: String,
}

#[async_trait]
impl Request for GetFile<'_> {
    type Output = File;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl GetFile<'_> {
    pub async fn send(self) -> ResponseResult<File> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getFile",
            &self,
        )
        .await
    }
}

impl<'a> GetFile<'a> {
    pub(crate) fn new<F>(bot: &'a Bot, value: F) -> Self
    where
        F: Into<String>,
    {
        Self {
            bot,
            file_id: value.into(),
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
