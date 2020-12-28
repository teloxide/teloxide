use bytes::Bytes;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use tokio::io::AsyncWrite;

use crate::{
    bot::Bot,
    net::{self, Download},
    DownloadError,
};

impl<'w> Download<'w> for Bot {
    type Err = DownloadError;

    // I would like to unbox this, but my coworkers will kill me if they'll see yet
    // another hand written `Future`. (waffle)
    type Fut = BoxFuture<'w, Result<(), Self::Err>>;

    fn download_file(
        &self,
        path: &str,
        destination: &'w mut (dyn AsyncWrite + Unpin + Send),
    ) -> Self::Fut {
        net::download_file(
            &self.client,
            self.api_url.get(),
            &self.token,
            path,
            destination,
        )
        .boxed()
    }

    type StreamErr = reqwest::Error;

    type Stream = BoxStream<'static, Result<Bytes, Self::StreamErr>>;

    fn download_file_stream(&self, path: &str) -> Self::Stream {
        net::download_file_stream(&self.client, self.api_url.get(), &self.token, path).boxed()
    }
}
