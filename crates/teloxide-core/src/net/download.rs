use std::future::Future;

use bytes::Bytes;
use futures::{
    future::{ready, Either},
    stream::{once, unfold},
    FutureExt, Stream, StreamExt,
};
use reqwest::{Client, Response, Url};
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::{errors::DownloadError, net::file_url};

/// A trait for downloading files from Telegram.
pub trait Download<'w>
/* FIXME(waffle): ideally, this lifetime ('w) shouldn't be here, but we can't help it without
 * GATs */
{
    /// An error returned from [`download_file`](Self::download_file).
    type Err;

    /// A future returned from [`download_file`](Self::download_file).
    type Fut: Future<Output = Result<(), Self::Err>> + Send;

    /// Download a file from Telegram into `destination`.
    ///
    /// `path` can be obtained from [`GetFile`].
    ///
    /// To download as a stream of chunks, see [`download_file_stream`].
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use teloxide_core::{
    ///     net::Download,
    ///     requests::{Request, Requester},
    ///     types::File,
    ///     Bot,
    /// };
    /// use tokio::fs;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = Bot::new("TOKEN");
    ///
    /// let file = bot.get_file("*file_id*").await?;
    /// let mut dst = fs::File::create("/tmp/test.png").await?;
    /// bot.download_file(&file.path, &mut dst).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`GetFile`]: crate::payloads::GetFile
    /// [`download_file_stream`]: Self::download_file_stream
    fn download_file(
        &self,
        path: &str,
        destination: &'w mut (dyn AsyncWrite + Unpin + Send),
    ) -> Self::Fut;

    /// An error returned from
    /// [`download_file_stream`](Self::download_file_stream).
    type StreamErr;

    /// A stream returned from [`download_file_stream`].
    ///
    ///[`download_file_stream`]: (Self::download_file_stream)
    type Stream: Stream<Item = Result<Bytes, Self::StreamErr>> + Send;

    /// Download a file from Telegram as [`Stream`].
    ///
    /// `path` can be obtained from the [`GetFile`].
    ///
    /// To download into an [`AsyncWrite`] (e.g. [`tokio::fs::File`]), see
    /// [`download_file`].
    ///
    /// [`GetFile`]: crate::payloads::GetFile
    /// [`AsyncWrite`]: tokio::io::AsyncWrite
    /// [`tokio::fs::File`]: tokio::fs::File
    /// [`download_file`]: Self::download_file
    fn download_file_stream(&self, path: &str) -> Self::Stream;
}

/// Download a file from Telegram into `dst`.
///
/// Note: if you don't need to use a different (from you're bot) client and
/// don't need to get *all* performance (and you don't, c'mon it's very io-bound
/// job), then it's recommended to use [`Download::download_file`].
pub fn download_file<'o, D>(
    client: &Client,
    api_url: Url,
    token: &str,
    path: &str,
    dst: &'o mut D,
) -> impl Future<Output = Result<(), DownloadError>> + 'o
where
    D: ?Sized + AsyncWrite + Unpin,
{
    client.get(file_url(api_url, token, path)).send().then(move |r| async move {
        let mut res = r?.error_for_status()?;

        while let Some(chunk) = res.chunk().await? {
            dst.write_all(&chunk).await?;
        }

        Ok(())
    })
}

/// Download a file from Telegram as [`Stream`].
///
/// Note: if you don't need to use a different (from you're bot) client and
/// don't need to get *all* performance (and you don't, c'mon it's very io-bound
/// job), then it's recommended to use [`Download::download_file_stream`].
pub fn download_file_stream(
    client: &Client,
    api_url: Url,
    token: &str,
    path: &str,
) -> impl Stream<Item = reqwest::Result<Bytes>> + 'static {
    client.get(file_url(api_url, token, path)).send().into_stream().flat_map(|res| {
        match res.and_then(Response::error_for_status) {
            Ok(res) => Either::Left(unfold(res, |mut res| async {
                match res.chunk().await {
                    Err(err) => Some((Err(err), res)),
                    Ok(Some(c)) => Some((Ok(c), res)),
                    Ok(None) => None,
                }
            })),
            Err(err) => Either::Right(once(ready(Err(err)))),
        }
    })
}
