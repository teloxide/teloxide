use reqwest::Client;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::errors::DownloadError;

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
        .get(crate::net::file_url(
            reqwest::Url::parse(crate::net::TELEGRAM_API_URL).expect("failed to parse default url"),
            token,
            path,
        ))
        .send()
        .await?
        .error_for_status()?;

    while let Some(chunk) = res.chunk().await? {
        destination.write_all(&chunk).await?;
    }

    Ok(())
}

pub async fn download_file_stream(
    client: &Client,
    token: &str,
    path: &str,
) -> Result<impl futures::Stream<Item = reqwest::Result<bytes::Bytes>>, reqwest::Error> {
    let res = client
        .get(crate::net::file_url(
            reqwest::Url::parse(crate::net::TELEGRAM_API_URL).expect("failed to parse default url"),
            token,
            path,
        ))
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
