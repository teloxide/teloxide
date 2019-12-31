use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::InlineQueryResult,
    types::True,
};

/// Use this method to send answers to an inline query. On success, True is returned.No more than 50 results per query are allowed.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    cache_time: Option<i32>,
    /// Pass True, if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query
    is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don‘t support pagination. Offset length can’t exceed 64 bytes.
    next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a private chat with the bot and sends the bot a start message with the parameter switch_pm_parameter
    switch_pm_text: Option<String>,
    /// Deep-linking parameter for the /start message sent to the bot when user presses the switch button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.Example: An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a ‘Connect your YouTube account’ button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an oauth link. Once done, the bot can offer a switch_inline button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    switch_pm_parameter: Option<String>,
}

impl Method for AnswerInlineQuery {
    type Output = True;

    const NAME: &'static str = "answerInlineQuery";
}

impl json::Payload for AnswerInlineQuery {}

impl dynamic::Payload for AnswerInlineQuery {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl AnswerInlineQuery {
    pub fn new<I, R>(inline_query_id: I, results: R) -> Self
    where
        I: Into<String>,
        R: Into<Vec<InlineQueryResult>>
    {
        let inline_query_id = inline_query_id.into();
        let results = results.into();
        Self {
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }
}

impl json::Request<'_, AnswerInlineQuery> {
    pub fn inline_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.inline_query_id = val.into();
        self
    }

    pub fn results<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<InlineQueryResult>>
    {
        self.payload.results = val.into();
        self
    }

    pub fn cache_time(mut self, val: i32) -> Self {
        self.payload.cache_time = Some(val);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_personal(mut self, val: bool) -> Self {
        self.payload.is_personal = Some(val);
        self
    }

    pub fn next_offset<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.next_offset = Some(val.into());
        self
    }

    pub fn switch_pm_text<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.switch_pm_text = Some(val.into());
        self
    }

    pub fn switch_pm_parameter<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.switch_pm_parameter = Some(val.into());
        self
    }
}
                 