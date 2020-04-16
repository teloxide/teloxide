use crate::{
    net,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};

use std::sync::Arc;

/// Use this method to send a dice, which will have a random value from 1 to 6.
/// On success, the sent Message is returned.
/// (Yes, we're aware of the "proper" singular of die.
/// But it's awkward, and we decided to help it change. One dice at a time!)
#[derive(Debug, Clone)]
pub struct SendDice {
    bot: Arc<Bot>,
    chat_id: ChatId,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>
}

#[async_trait::async_trait]
impl Request for SendDice {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendDice",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .await
                .add("disable_notification", &self.disable_notification)
                .await
                .add("reply_to_message_id", &self.reply_to_message_id)
                .await
                .add("reply_markup", &self.reply_markup)
                .await
                .build(),
        )
        .await
    }
}

impl SendDice {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    /// Additional interface options.
    ///
    /// A JSON-serialized object for an [inline keyboard], [custom reply
    /// keyboard], instructions to remove reply keyboard or to force a reply
    /// from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
