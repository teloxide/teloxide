use bytes::Bytes;
use tokio::{io::AsyncWrite, stream::Stream};

use crate::{
    bot::Bot,
    net::{download_file, download_file_stream},
    DownloadError,
};

impl Bot {
    /// Download a file from Telegram into `destination`.
    ///
    /// `path` can be obtained from [`GetFile`].
    ///
    /// To download as a stream of chunks, see [`Bot::download_file_stream`].
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use teloxide_core::{
    ///     requests::{Request, Requester},
    ///     types::File as TgFile,
    ///     Bot,
    /// };
    /// use tokio::fs::File;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = Bot::new("TOKEN");
    ///
    /// let TgFile { file_path, .. } = bot.get_file("*file_id*").send().await?;
    /// let mut file = File::create("/home/waffle/Pictures/test.png").await?;
    /// bot.download_file(&file_path, &mut file).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`GetFile`]: crate::payloads::GetFile
    /// [`Bot::download_file_stream`]: crate::Bot::download_file_stream
    pub async fn download_file<D>(
        &self,
        path: &str,
        destination: &mut D,
    ) -> Result<(), DownloadError>
    where
        D: AsyncWrite + Unpin,
    {
        download_file(&self.client, &self.token, path, destination).await
    }

    /// Download a file from Telegram.
    ///
    /// `path` can be obtained from the [`GetFile`].
    ///
    /// To download into [`AsyncWrite`] (e.g. [`tokio::fs::File`]), see
    /// [`Bot::download_file`].
    ///
    /// [`GetFile`]: crate::payloads::GetFile
    /// [`AsyncWrite`]: tokio::io::AsyncWrite
    /// [`tokio::fs::File`]: tokio::fs::File
    /// [`Bot::download_file`]: crate::Bot::download_file
    pub async fn download_file_stream(
        &self,
        path: &str,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, reqwest::Error> {
        download_file_stream(&self.client, &self.token, path).await
    }
}
