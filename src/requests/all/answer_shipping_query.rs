use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ShippingOption, True},
};

/// If you sent an invoice requesting a shipping address and the parameter
/// is_flexible was specified, the Bot API will send an Update with a
/// shipping_query field to the bot. Use this method to reply to shipping
/// queries. On success, True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct AnswerShippingQuery {
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
impl Request<True> for AnswerShippingQuery {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "answerShippingQuery",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl AnswerShippingQuery {
    pub fn new<S>(shipping_query_id: S, ok: bool) -> Self
    where
        S: Into<String>,
    {
        let shipping_query_id = shipping_query_id.into();
        Self {
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
