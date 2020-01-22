use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ShippingOption, True},
    Bot,
};

/// If you sent an invoice requesting a shipping address and the parameter
/// is_flexible was specified, the Bot API will send an Update with a
/// shipping_query field to the bot. Use this method to reply to shipping
/// queries. On success, True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct AnswerShippingQuery<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the query to be answered
    shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False
    /// if there are any problems (for example, if delivery to the specified
    /// address is not possible)
    ok: bool,
    /// Required if ok is True. A JSON-serialized array of available shipping
    /// options.
    shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that
    /// explains why it is impossible to complete the order (e.g. "Sorry,
    /// delivery to your desired address is unavailable'). Telegram will
    /// display this message to the user.
    error_message: Option<String>,
}

#[async_trait::async_trait]
impl Request for AnswerShippingQuery<'_> {
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

impl<'a> AnswerShippingQuery<'a> {
    pub(crate) fn new<S>(bot: &'a Bot, shipping_query_id: S, ok: bool) -> Self
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

    pub fn shipping_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.shipping_query_id = val.into();
        self
    }

    pub fn ok(mut self, val: bool) -> Self {
        self.ok = val;
        self
    }

    pub fn shipping_options<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<ShippingOption>>,
    {
        self.shipping_options = Some(val.into());
        self
    }

    pub fn error_message<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.error_message = Some(val.into());
        self
    }
}
