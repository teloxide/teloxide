use serde::{de::DeserializeOwned, Serialize};

use super::{Method, ResponseResult};
use crate::{network, Bot};

pub trait Payload: Serialize + Method {}

/// Ready-to-send telegram request.
///
/// Note: params will be sent to telegram using [`application/json`]
///
/// See [GetUpdates] and [SendMessage] for reference implementations.
///
/// [`application/json`]: https://core.telegram.org/bots/api#making-requests
/// [GetUpdates]: crate::requests::payloads::GetUpdates
/// [SendMessage]: crate::requests::payloads::SendMessage
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
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            P::NAME,
            &self.payload,
        )
        .await
    }
}
