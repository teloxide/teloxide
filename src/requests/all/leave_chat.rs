use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method for your bot to leave a group, supergroup or channel.
///
/// [The official docs](https://core.telegram.org/bots/api#leavechat).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct LeaveChat<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for LeaveChat<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "leaveChat",
            &self,
        )
        .await
    }
}

impl<'a> LeaveChat<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot: BotWrapper(bot),
            chat_id,
        }
    }

    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }
}
