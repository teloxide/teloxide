use std::path::Path;

use bytes::Bytes;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWrite, AsyncWriteExt},
};

use crate::{
    bot::Bot,
    net::{self, Download},
    DownloadError,
};

impl Download for Bot {
    type Err<'dst> = DownloadError;

    // I would like to unbox this, but my coworkers will kill me if they'll see yet
    // another hand written `Future`. (waffle)
    type Fut<'dst> = BoxFuture<'dst, Result<(), Self::Err<'dst>>>;

    fn download_file<'dst>(
        &self,
        path: &'dst str,
        destination: &'dst mut (dyn AsyncWrite + Unpin + Send),
    ) -> Self::Fut<'dst> {
        let is_localhost = match &self.api_url.host_str() {
            Some(host) => ["localhost", "127.0.0.1"].contains(host),
            None => false,
        };
        // If path is absolute and api_url contains localhost, it is pretty clear there
        // is a local TBA server with --local option, we can just copy the file
        if is_localhost && Path::new(&path).is_absolute() {
            return copy_file(path, destination).boxed();
        }

        net::download_file(
            &self.client,
            reqwest::Url::clone(&*self.api_url),
            &self.token,
            path,
            destination,
        )
        .boxed()
    }

    type StreamErr = reqwest::Error;

    type Stream = BoxStream<'static, Result<Bytes, Self::StreamErr>>;

    fn download_file_stream(&self, path: &str) -> Self::Stream {
        net::download_file_stream(
            &self.client,
            reqwest::Url::clone(&*self.api_url),
            &self.token,
            path,
        )
        .map(|res| res.map_err(crate::errors::hide_token))
        .boxed()
    }
}

async fn copy_file<'o, D>(path: &'o str, dst: &'o mut D) -> Result<(), DownloadError>
where
    D: ?Sized + AsyncWrite + Unpin,
{
    let mut src_file = File::open(path).await?;

    let mut buffer = [0; 1024];
    loop {
        let n = src_file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        dst.write_all(&buffer[..n]).await?;
    }

    dst.flush().await?;
    Ok(())
}
