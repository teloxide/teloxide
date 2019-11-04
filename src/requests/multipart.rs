use serde::de::DeserializeOwned;
use reqwest::multipart;

use crate::{Bot, network};
use super::{ResponseResult, Method};

pub trait Payload: Method {
    fn payload(&self) -> multipart::Form;
}

/// Ready-to-send telegram request.
///
/// Note: params will be sent to telegram using [`multipart/form-data`]
///
/// [`multipart/form-data`]: // TODO: libk to tgdoc
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
            P::METHOD,
            self.payload.payload(),
        ).await
    }
}
