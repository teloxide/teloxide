use serde::de::DeserializeOwned;
use reqwest::multipart;

use crate::{Bot, network};
use super::{ResponseResult, Method};
use std::marker::PhantomData;

/// Ready-to-send telegram request without params.
///
/// NOTE: Currently where is only one request without params - [GetMe]
///
/// [GetMe]: // TODO
#[must_use = "requests do nothing until sent"]
pub struct Request<'b, M> {
    bot: &'b Bot,
    marker: PhantomData<M>,
}

impl<'b, M> Request<'b, M>
where
    M: Method,
    M::Output: DeserializeOwned,
{
    pub fn new(bot: &'b Bot) -> Self {
        Self { bot, marker: PhantomData }
    }

    /// Send request to telegram
    pub async fn send(&self) -> ResponseResult<M::Output> {
        network::request_simple(
            self.bot.client(),
            self.bot.token(),
            M::METHOD,
        ).await
    }
}
