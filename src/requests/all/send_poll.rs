use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, PollType, ReplyMarkup},
    Bot,
};

/// Use this method to send a native poll.
///
/// [The official docs](https://core.telegram.org/bots/api#sendpoll).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendPoll {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
    question: String,
    options: Vec<String>,
    is_anonymous: Option<bool>,
    poll_type: Option<PollType>,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<i32>,
    is_closed: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request for SendPoll {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendPoll",
            &self,
        )
        .await
    }
}

impl SendPoll {
    pub(crate) fn new<C, Q, O>(
        bot: Bot,
        chat_id: C,
        question: Q,
        options: O,
    ) -> Self
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: Into<Vec<String>>,
    {
        let chat_id = chat_id.into();
        let question = question.into();
        let options = options.into();
        Self {
            bot,
            chat_id,
            question,
            options,
            is_anonymous: None,
            poll_type: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            is_closed: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    ///
    /// A native poll can't be sent to a private chat.
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Poll question, 1-255 characters.
    pub fn question<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.question = val.into();
        self
    }

    /// List of answer options, 2-10 strings 1-100 characters each.
    pub fn options<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<String>>,
    {
        self.options = val.into();
        self
    }

    /// `true`, if the poll needs to be anonymous, defaults to `true`.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_anonymous<T>(mut self, val: T) -> Self
    where
        T: Into<bool>,
    {
        self.is_anonymous = Some(val.into());
        self
    }

    /// Poll type, `quiz` or `regular`, defaults to `regular`.
    pub fn poll_type(mut self, val: PollType) -> Self {
        self.poll_type = Some(val);
        self
    }

    /// `true`, if the poll allows multiple answers, ignored for polls in quiz
    /// mode.
    ///
    /// Defaults to `false`.
    pub fn allows_multiple_answers<T>(mut self, val: T) -> Self
    where
        T: Into<bool>,
    {
        self.allows_multiple_answers = Some(val.into());
        self
    }

    /// 0-based identifier of the correct answer option, required for polls in
    /// quiz mode.
    pub fn correct_option_id<T>(mut self, val: T) -> Self
    where
        T: Into<i32>,
    {
        self.correct_option_id = Some(val.into());
        self
    }

    /// Pass `true`, if the poll needs to be immediately closed.
    ///
    /// This can be useful for poll preview.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_closed<T>(mut self, val: T) -> Self
    where
        T: Into<bool>,
    {
        self.is_closed = Some(val.into());
        self
    }

    /// Sends the message [silently].
    ///
    /// Users will receive a notification with no sound.
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
