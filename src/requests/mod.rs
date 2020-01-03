//! API requests.

mod all;
mod form_builder;
mod utils;

pub use all::*;

/// A type that is returned after making a request to Telegram.
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// Designates an API request.
#[async_trait::async_trait]
pub trait Request {
    /// A data structure returned if success.
    type Output;

    /// Asynchronously sends this request to Telegram and returns the result.
    async fn send(&self) -> ResponseResult<Self::Output>;
}
