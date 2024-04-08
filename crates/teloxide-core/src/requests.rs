//! Telegram API requests.

pub use self::{
    has_payload::HasPayload, json::JsonRequest, multipart::MultipartRequest,
    multipart_payload::MultipartPayload, payload::Payload, request::Request, requester::Requester,
    requester_ext::RequesterExt,
};

/// A type that is returned after making a request to Telegram.
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// An output type of [`Payload`] in [`HasPayload`].
pub type Output<T> = <<T as HasPayload>::Payload as Payload>::Output;

mod has_payload;
mod json;
mod multipart;
pub(crate) mod multipart_payload;
mod payload;
mod request;
mod requester;
mod requester_ext;
