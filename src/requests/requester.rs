use crate::{
    payloads::{GetMe, SendMessage},
    requests::Request,
    types::ChatId,
};

/// The trait implemented by all bots & bot adaptors.
/// Essentially a request builder factory (?).
///
/// _This trait is included in the crate's [`prelude`](crate::prelude)_.
#[cfg_attr(all(docsrs, feature = "nightly"), doc(spotlight))]
pub trait Requester {
    type Err: std::error::Error + Send;

    type GetMe: Request<Payload = GetMe, Err = Self::Err>;

    /// For telegram documentation of the method see [`GetMe`].
    fn get_me(&self) -> Self::GetMe;

    type SendMessage: Request<Payload = SendMessage, Err = Self::Err>;

    /// For telegram documentation of the method see [`SendMessage`].
    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>;

    // FIXME(waffle): add remaining 68 methods
}
