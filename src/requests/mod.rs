//! API requests.

mod form_builder;
mod utils;

pub mod dynamic;
pub mod json;
pub mod multipart;

/// A type that is returned when making requests to telegram
pub type ResponseResult<T> = Result<T, crate::RequestError>;

///// A request that can be sent to Telegram.
//#[async_trait]
//pub trait Request {
//    /// A type of response.
//    type Output: DeserializeOwned; // TODO: do we need this bound _here_?
//
//    /// Send this request.
//    async fn send_boxed(self) -> ResponseResult<Self::Output>;
//}

/// Signature of telegram method.
pub trait Method {
    /// Return-type of the method.
    type Output;

    /// Name of the method.
    const NAME: &'static str;
}

/// Signature of telegram method.
///
/// Note: this trait is very similar to [`Method`] trait, however it can be used
/// as trait object.
pub trait DynMethod {
    type Output;

    /// Return name of the method.
    fn name(&self) -> &str;
}

impl<T> DynMethod for T
where
    T: Method,
{
    type Output = T::Output;

    fn name(&self) -> &str {
        T::NAME
    }
}

#[rustfmt::skip]
pub mod payloads {
    // payloads are sorted as in tg docs (https://core.telegram.org/bots/api)

    // Getting updates
    mod get_updates;

    pub use get_updates::{AllowedUpdate, GetUpdates};

    // Available methods
    mod get_me;
    mod send_message;

    mod send_animation;

    mod get_file;

    pub use self::{
        get_me::GetMe,
        send_message::SendMessage,

        send_animation::SendAnimation,

        get_file::GetFile,
    };
}
