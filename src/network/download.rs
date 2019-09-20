use reqwest::r#async::{Client, Chunk};
use tokio::{
    stream::Stream,
    io::{AsyncWrite, AsyncWriteExt},
};

use crate::{
    DownloadError,
    network::{TELEGRAM_API_URL, file_url},
};


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
        destination.write_all(chunk?.bytes()).await?;
    }

    Ok(())
}

pub(crate) async fn download_file_stream(
    client: &Client,
    token: &str,
    path: &str,
) -> Result<impl Stream<Item = Result<Chunk, reqwest::Error>>, reqwest::Error> {
    let url = file_url(TELEGRAM_API_URL, token, path);
    let resp = client.get(&url).send().await?.error_for_status()?;
    Ok(resp.into_body())
}