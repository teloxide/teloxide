use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to send a game.
///
/// [The official docs](https://core.telegram.org/bots/api#sendgame).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendGame {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: i32,
    game_short_name: String,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for SendGame {
    type Output = Message;

    async fn send(self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "sendGame", &self).await
    }
}

impl SendGame {
    pub(crate) fn new<G>(bot: Bot, chat_id: i32, game_short_name: G) -> Self
    where
        G: Into<String>,
    {
        let game_short_name = game_short_name.into();
        Self {
            bot,
            chat_id,
            game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat.
    pub fn chat_id(mut self, val: i32) -> Self {
        self.chat_id = val;
        self
    }

    /// Short name of the game, serves as the unique identifier for the game.
    /// Set up your games via [@Botfather].
    ///
    /// [@Botfather]: https://t.me/botfather
    pub fn game_short_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.game_short_name = val.into();
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

    /// A JSON-serialized object for an [inline keyboard]. If empty, one `Play
    /// game_title` button will be shown. If not empty, the first button must
    /// launch the game.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
