use bytes::Bytes;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use tokio::io::AsyncWrite;

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

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn download_file<'dst>(
        &self,
        path: &str,
        destination: &'dst mut (dyn AsyncWrite + Unpin + Send),
    ) -> Self::Fut<'dst> {
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

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
