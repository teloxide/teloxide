use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::True,
};

/// Use this method to send answers to callback queries sent from inline
/// keyboards. The answer will be displayed to the user as a notification at the
/// top of the chat screen or as an alert. On success, True is returned.
///
/// Alternatively, the user can be redirected to the specified Game URL. For
/// this option to work, you must first create a game for your bot via
/// @Botfather and accept the terms. Otherwise, you may use links like
/// t.me/your_bot?start=XXXX that open your bot with a parameter.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerCallbackQuery<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the query to be answered.
    callback_query_id: String,

    /// Text of the notification. If not specified, nothing will be shown to
    /// the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    /// If true, an alert will be shown by the client instead of a notification
    /// at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,

    /// URL that will be opened by the user's client. If you have created a
    /// Game and accepted the conditions via @Botfather, specify the URL that
    /// opens your game – note that this will only work if the query comes from
    /// a callback_game button.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    /// The maximum amount of time in seconds that the result of the callback
    /// query may be cached client-side. Telegram apps will support caching
    /// starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<i32>,
}

#[async_trait]
impl Request for AnswerCallbackQuery<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl AnswerCallbackQuery<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "answerCallbackQuery",
            &self,
        )
        .await
    }
}

impl<'a> AnswerCallbackQuery<'a> {
    pub(crate) fn new<S>(bot: &'a Bot, callback_query_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            bot,
            callback_query_id: callback_query_id.into(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    pub fn callback_query_id<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.callback_query_id = value.into();
        self
    }

    pub fn text<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.text = Some(value.into());
        self
    }

    pub fn show_alert<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.show_alert = Some(value.into());
        self
    }

    pub fn url<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.url = Some(value.into());
        self
    }

    pub fn cache_time<I>(mut self, value: I) -> Self
    where
        I: Into<i32>,
    {
        self.cache_time = Some(value.into());
        self
    }
}
