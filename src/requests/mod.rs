//! API requests.

mod has_payload;
mod payload;
mod request;

pub use self::{has_payload::HasPayload, payload::Payload, request::Request};

mod all;
mod json;
mod multipart;
mod utils;

pub use all::*;
pub use json::JsonRequest;
pub use multipart::MultipartRequest;

/// A type that is returned after making a request to Telegram.
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// Output of a [`Payload`] in [`HasPayload`]. Alias to
/// `<<T as HasPayload>::Payload as Payload>::Output`.
pub type Output<T> = <<T as HasPayload>::Payload as Payload>::Output;

/// Designates an API request.
#[async_trait::async_trait]
pub trait RequestOld {
    /// A data structure returned if success.
    type Output;

    /// Asynchronously sends this request to Telegram and returns the result.
    async fn send(&self) -> ResponseResult<Self::Output>;
}
