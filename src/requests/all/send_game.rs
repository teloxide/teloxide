use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to send a game. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendGame<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat
    chat_id: i32,
    /// Short name of the game, serves as the unique identifier for the game.
    /// Set up your games via Botfather.
    game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// A JSON-serialized object for an inline keyboard. If empty, one ‘Play
    /// game_title’ button will be shown. If not empty, the first button must
    /// launch the game.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for SendGame<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendGame",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SendGame<'a> {
    pub(crate) fn new<G>(bot: &'a Bot, chat_id: i32, game_short_name: G) -> Self
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

    pub fn chat_id(mut self, val: i32) -> Self {
        self.chat_id = val;
        self
    }

    pub fn game_short_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.game_short_name = val.into();
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
