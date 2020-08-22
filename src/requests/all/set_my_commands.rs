use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{BotCommand, True},
    Bot,
};

/// Use this method to change the list of the bot's commands.
///
/// [The official docs](https://core.telegram.org/bots/api#setmycommands).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetMyCommands {
    #[serde(skip_serializing)]
    bot: Bot,

    commands: Vec<BotCommand>,
}

#[async_trait::async_trait]
impl RequestOld for SetMyCommands {
    type Output = True;

    async fn send(&self) -> ResponseResult<Self::Output> {
        net::request_json(self.bot.client(), self.bot.token(), "setMyCommands", &self).await
    }
}

impl SetMyCommands {
    pub(crate) fn new<C>(bot: Bot, commands: C) -> Self
    where
        C: Into<Vec<BotCommand>>,
    {
        Self { bot, commands: commands.into() }
    }

    /// A JSON-serialized list of bot commands to be set as the list of the
    /// bot's commands.
    ///
    /// At most 100 commands can be specified.
    pub fn commands<C>(mut self, commands: C) -> Self
    where
        C: Into<Vec<BotCommand>>,
    {
        self.commands = commands.into();
        self
    }
}
