use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};
use std::sync::Arc;

/// Use this method to send answers to callback queries sent from [inline
/// keyboards].
///
/// The answer will be displayed to the user as a notification at
/// the top of the chat screen or as an alert.
///
/// [The official docs](https://core.telegram.org/bots/api#answercallbackquery).
///
/// [inline keyboards]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct AnswerCallbackQuery {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<i32>,
}

#[async_trait::async_trait]
impl Request for AnswerCallbackQuery {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "answerCallbackQuery",
            &self,
        )
        .await
    }
}

impl AnswerCallbackQuery {
    pub(crate) fn new<C>(bot: Arc<Bot>, callback_query_id: C) -> Self
    where
        C: Into<String>,
    {
        let callback_query_id = callback_query_id.into();
        Self {
            bot,
            callback_query_id,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    /// Unique identifier for the query to be answered.
    pub fn callback_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.callback_query_id = val.into();
        self
    }

    /// Text of the notification. If not specified, nothing will be shown to the
    /// user, 0-200 characters.
    pub fn text<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.text = Some(val.into());
        self
    }

    /// If `true`, an alert will be shown by the client instead of a
    /// notification at the top of the chat screen. Defaults to `false`.
    pub fn show_alert(mut self, val: bool) -> Self {
        self.show_alert = Some(val);
        self
    }

    /// URL that will be opened by the user's client. If you have created a
    /// [`Game`] and accepted the conditions via [@Botfather], specify the
    /// URL that opens your game – note that this will only work if the
    /// query comes from a [`callback_game`] button.
    ///
    /// Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open
    /// your bot with a parameter.
    ///
    /// [@Botfather]: https://t.me/botfather
    /// [`callback_game`]: crate::types::InlineKeyboardButton
    /// [`Game`]: crate::types::Game
    pub fn url<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.url = Some(val.into());
        self
    }

    /// The maximum amount of time in seconds that the result of the callback
    /// query may be cached client-side. Telegram apps will support caching
    /// starting in version 3.14. Defaults to 0.
    pub fn cache_time(mut self, val: i32) -> Self {
        self.cache_time = Some(val);
        self
    }
}
