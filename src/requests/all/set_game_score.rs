use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{Message, TargetMessage},
    Bot,
};

/// Use this method to set the score of the specified user in a game.
///
/// On success, if the message was sent by the bot, returns the edited
/// [`Message`], otherwise returns [`True`]. Returns an error, if the new score
/// is not greater than the user's current score in the chat and force is
/// `false`.
///
/// [The official docs](https://core.telegram.org/bots/api#setgamescore).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetGameScore {
    #[serde(skip_serializing)]
    bot: Bot,
    #[serde(flatten)]
    target: TargetMessage,
    user_id: i32,
    score: i32,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
}

#[async_trait::async_trait]
impl Request for SetGameScore {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "setGameScore", &self).await
    }
}

impl SetGameScore {
    pub(crate) fn new<T>(bot: Bot, target: T, user_id: i32, score: i32) -> Self
    where
        T: Into<TargetMessage>,
    {
        let target = target.into();
        Self { bot, target, user_id, score, force: None, disable_edit_message: None }
    }

    /// Target message, either chat id and message id or inline message id.
    pub fn target<T>(mut self, val: T) -> Self
    where
        T: Into<TargetMessage>,
    {
        self.target = val.into();
        self
    }

    /// User identifier.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// New score, must be non-negative.
    pub fn score(mut self, val: i32) -> Self {
        self.score = val;
        self
    }

    /// Pass `true`, if the high score is allowed to decrease.
    ///
    /// This can be useful when fixing mistakes or banning cheaters.
    pub fn force(mut self, val: bool) -> Self {
        self.force = Some(val);
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_edit_message(mut self, val: bool) -> Self {
        self.disable_edit_message = Some(val);
        self
    }
}
