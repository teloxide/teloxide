use serde::{de::DeserializeOwned, Serialize};

use crate::{Bot, network};
use super::{ResponseResult, Method};

pub trait Payload: Serialize + Method {}

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

    pub async fn send(&self) -> ResponseResult<P::Output> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            P::METHOD,
            &self.payload,
        ).await
    }
}
