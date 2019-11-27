use reqwest::multipart;
use serde::de::DeserializeOwned;

use super::{Method, ResponseResult};
use crate::{network, Bot};

pub trait Payload: Method {
    fn payload(&self) -> multipart::Form;
}

/// Ready-to-send telegram request.
///
/// Note: params will be sent to telegram using [`multipart/form-data`]
///
/// See [SendAnimation] for reference implementation.
///
/// [`multipart/form-data`]: https://core.telegram.org/bots/api#making-requests
/// [SendAnimation]: crate::requests::payloads::SendAnimation
#[must_use = "requests do nothing until sent"]
pub struct Request<'b, P> {
    bot: &'b Bot,
    pub(crate) payload: P,
}

impl<'b, P> Request<'b, P>
where
    P: Payload,
    P::Output: DeserializeOwned,
{
    pub fn new(bot: &'b Bot, payload: P) -> Self {
        Self { bot, payload }
    }

    /// Send request to telegram
    pub async fn send(&self) -> ResponseResult<P::Output> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            P::NAME,
            self.payload.payload(),
        )
        .await
    }
}
