use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::True,
};

/// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via @Botfather and accept the terms. Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    text: Option<String>,
    /// If true, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @Botfather, specify the URL that opens your game – note that this will only work if the query comes from a callback_game button.Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
    url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    cache_time: Option<i32>,
}

impl Method for AnswerCallbackQuery {
    type Output = True;

    const NAME: &'static str = "answerCallbackQuery";
}

impl json::Payload for AnswerCallbackQuery {}

impl dynamic::Payload for AnswerCallbackQuery {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl AnswerCallbackQuery {
    pub fn new<C>(callback_query_id: C) -> Self
    where
        C: Into<String>
    {
        let callback_query_id = callback_query_id.into();
        Self {
            callback_query_id,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }
}

impl json::Request<'_, AnswerCallbackQuery> {
    pub fn callback_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.callback_query_id = val.into();
        self
    }

    pub fn text<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.text = Some(val.into());
        self
    }

    pub fn show_alert(mut self, val: bool) -> Self {
        self.payload.show_alert = Some(val);
        self
    }

    pub fn url<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.url = Some(val.into());
        self
    }

    pub fn cache_time(mut self, val: i32) -> Self {
        self.payload.cache_time = Some(val);
        self
    }
}
                 