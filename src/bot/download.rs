use tokio::io::AsyncWrite;

#[cfg(feature = "unstable-stream")]
use ::{bytes::Bytes, tokio::stream::Stream};

#[cfg(feature = "unstable-stream")]
use crate::network::download_file_stream;
use crate::{bot::Bot, network::download_file, DownloadError};

impl Bot {
    /// Download a file from Telegram into `destination`.
    /// `path` can be obtained from [`get_file`] method.
    ///
    /// For downloading as Stream of Chunks see [`download_file_stream`].
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use teloxide::types::File as TgFile;
    /// use tokio::fs::File;
    /// # use teloxide::RequestError;
    /// use teloxide::{requests::Request, Bot};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = Bot::new("TOKEN");
    /// let mut file = File::create("/home/waffle/Pictures/test.png").await?;
    ///
    /// let TgFile { file_path, .. } = bot.get_file("*file_id*").send().await?;
    /// bot.download_file(&file_path, &mut file).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`get_file`]: crate::bot::Bot::get_file
    /// [`download_file_stream`]: crate::bot::Bot::download_file_stream
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
    /// `path` can be obtained from the [`get_file`] method.
    ///
    /// For downloading into [`AsyncWrite`] (e.g. [`tokio::fs::File`])
    /// see  [`download_file`].
    ///
    /// [`get_file`]: crate::bot::Bot::get_file
    /// [`AsyncWrite`]: tokio::io::AsyncWrite
    /// [`tokio::fs::File`]: tokio::fs::File
    /// [`download_file`]: crate::bot::Bot::download_file
    #[cfg(feature = "unstable-stream")]
    pub async fn download_file_stream(
        &self,
        path: &str,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, reqwest::Error>
    {
        download_file_stream(&self.client, &self.token, path).await
    }
}
