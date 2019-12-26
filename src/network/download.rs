use reqwest::Client;
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[cfg(feature = "unstable-stream")]
use ::{bytes::Bytes, tokio::stream::Stream};

use crate::DownloadError;

use super::TELEGRAM_API_URL;

pub async fn download_file<D>(
    client: &Client,
    token: &str,
    path: &str,
    destination: &mut D,
) -> Result<(), DownloadError>
where
    D: AsyncWrite + Unpin,
{
    let mut res = client
        .get(&super::file_url(TELEGRAM_API_URL, token, path))
        .send()
        .await?
        .error_for_status()?;

    while let Some(chunk) = res.chunk().await? {
        destination.write_all(&chunk).await?;
    }

    Ok(())
}

#[cfg(feature = "unstable-stream")]
pub async fn download_file_stream(
    client: &Client,
    token: &str,
    path: &str,
) -> Result<impl Stream<Item = reqwest::Result<Bytes>>, reqwest::Error> {
    let res = client
        .get(&super::file_url(TELEGRAM_API_URL, token, path))
        .send()
        .await?
        .error_for_status()?;

    Ok(futures::stream::unfold(res, |mut res| async {
        match res.chunk().await {
            Err(err) => Some((Err(err), res)),
            Ok(Some(c)) => Some((Ok(c), res)),
            Ok(None) => None,
        }
    }))
}
