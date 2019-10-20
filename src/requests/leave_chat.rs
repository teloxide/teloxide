use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::ChatId,
};

/// Use this method for your bot to leave a group, supergroup or channel.
/// Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct LeaveChat<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username
    /// of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait]
impl Request for LeaveChat<'_> {
    type Output = bool;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl LeaveChat<'_> {
    pub async fn send(self) -> ResponseResult<bool> {
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
    pub(crate) fn new<F>(bot: &'a Bot, value: F) -> Self
    where
        F: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: value.into(),
        }
    }
    pub fn chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }
}
