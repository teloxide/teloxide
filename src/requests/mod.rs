//! API requests.

mod all;
mod form_builder;
mod utils;

pub use all::*;

/// A type that is returned after making a request to Telegram.
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// A shortcut for `ResponseResult::Ok(val)`.
pub fn respond<T>(val: T) -> ResponseResult<T> {
    ResponseResult::Ok(val)
}

/// Designates an API request.
#[async_trait::async_trait]
pub trait Request {
    /// A data structure returned if success.
    type Output;

    /// Asynchronously sends this request to Telegram and returns the result.
    async fn send(&self) -> ResponseResult<Self::Output>;
}

/// Designates an API request with possibly sending file.
#[async_trait::async_trait]
pub trait RequestWithFile {
    /// A data structure returned if success.
    type Output;

    /// Asynchronously sends this request to Telegram and returns the result.
    /// Returns `tokio::io::Result::Err` when trying to send file which does not
    /// exists.
    async fn send(&self) -> tokio::io::Result<ResponseResult<Self::Output>>;
}
