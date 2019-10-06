use bytes::Buf;
use futures::StreamExt;
use reqwest::r#async::{Chunk, Client};
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    stream::Stream,
};

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
    let mut stream = download_file_stream(client, token, path).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        destination.write_all(chunk.bytes()).await?;
    }

    Ok(())
}

pub async fn download_file_stream(
    client: &Client,
    token: &str,
    path: &str,
) -> Result<impl Stream<Item = Result<Chunk, reqwest::Error>>, reqwest::Error> {
    Ok(client
        .get(&super::file_url(TELEGRAM_API_URL, token, path))
        .send()
        .await?
        .error_for_status()?
        .into_body())
}
