use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ShippingOption, True},
    Bot,
};
use std::sync::Arc;

/// If you sent an invoice requesting a shipping address and the parameter
/// `is_flexible` was specified, the Bot API will send an [`Update`] with a
/// shipping_query field to the bot. Use this method to reply to shipping
/// queries.
///
/// [The official docs](https://core.telegram.org/bots/api#answershippingquery).
///
/// [`Update`]: crate::types::Update
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct AnswerShippingQuery {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}

#[async_trait::async_trait]
impl Request for AnswerShippingQuery {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "answerShippingQuery",
            &self,
        )
        .await
    }
}

impl AnswerShippingQuery {
    pub(crate) fn new<S>(bot: Arc<Bot>, shipping_query_id: S, ok: bool) -> Self
    where
        S: Into<String>,
    {
        let shipping_query_id = shipping_query_id.into();
        Self {
            bot,
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None,
        }
    }

    /// Unique identifier for the query to be answered.
    pub fn shipping_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.shipping_query_id = val.into();
        self
    }

    /// Specify `true` if delivery to the specified address is possible and
    /// `false` if there are any problems (for example, if delivery to the
    /// specified address is not possible).
    pub fn ok(mut self, val: bool) -> Self {
        self.ok = val;
        self
    }

    /// Required if ok is `true`. A JSON-serialized array of available shipping
    /// options.
    pub fn shipping_options<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<ShippingOption>>,
    {
        self.shipping_options = Some(val.into());
        self
    }

    /// Required if ok is `false`. Error message in human readable form that
    /// explains why it is impossible to complete the order (e.g. "Sorry,
    /// delivery to your desired address is unavailable').
    ///
    /// Telegram will display this message to the user.
    pub fn error_message<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.error_message = Some(val.into());
        self
    }
}
