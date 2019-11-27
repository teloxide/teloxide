use reqwest::multipart;
use serde::de::DeserializeOwned;

use super::{DynMethod, ResponseResult};
use crate::{network, Bot};

/// [`Payload`] kind. Used to determinate the way for sending request.
pub enum Kind {
    Json(String),
    Multipart(multipart::Form),
}

pub trait Payload: DynMethod {
    fn kind(&self) -> Kind;
}

/// Dynamic ready-to-send telegram request.
///
/// This type is useful for storing different requests in one place, however
/// this type has _little_ overhead, so prefer using [json] or [multipart]
/// requests when possible.
///
/// See [GetUpdates], [SendMessage] and [SendAnimation] for reference
/// implementations.
///
/// [json]: crate::requests::json::Request
/// [multipart]: crate::requests::multipart::Request
/// [GetUpdates]: crate::requests::payloads::GetUpdates
/// [SendMessage]: crate::requests::payloads::SendMessage
/// [SendAnimation]: crate::requests::payloads::SendAnimation
#[must_use = "requests do nothing until sent"]
pub struct Request<'b, O> {
    bot: &'b Bot,
    pub(crate) payload: &'b dyn Payload<Output = O>, // TODO: Box?
}

impl<'b, O> Request<'b, O>
where
    O: DeserializeOwned,
{
    pub fn new(bot: &'b Bot, payload: &'b dyn Payload<Output = O>) -> Self {
        Self { bot, payload }
    }

    /// Send request to telegram
    pub async fn send(&self) -> ResponseResult<O> {
        network::request_dynamic(
            self.bot.client(),
            self.bot.token(),
            self.payload.name(),
            self.payload.kind(),
        )
        .await
    }
}
