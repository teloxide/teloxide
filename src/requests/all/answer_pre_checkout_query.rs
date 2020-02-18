use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};
use std::sync::Arc;

/// Once the user has confirmed their payment and shipping details, the Bot API
/// sends the final confirmation in the form of an [`Update`] with the field
/// `pre_checkout_query`. Use this method to respond to such pre-checkout
/// queries. Note: The Bot API must receive an answer within 10 seconds after
/// the pre-checkout query was sent.
///
/// [The official docs](https://core.telegram.org/bots/api#answerprecheckoutquery).
///
/// [`Update`]: crate::types::Update
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct AnswerPreCheckoutQuery {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    pre_checkout_query_id: String,
    ok: bool,
    error_message: Option<String>,
}

#[async_trait::async_trait]
impl Request for AnswerPreCheckoutQuery {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "answerPreCheckoutQuery",
            &self,
        )
        .await
    }
}

impl AnswerPreCheckoutQuery {
    pub(crate) fn new<P>(
        bot: Arc<Bot>,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> Self
    where
        P: Into<String>,
    {
        let pre_checkout_query_id = pre_checkout_query_id.into();
        Self { bot, pre_checkout_query_id, ok, error_message: None }
    }

    /// Unique identifier for the query to be answered.
    pub fn pre_checkout_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.pre_checkout_query_id = val.into();
        self
    }

    /// Specify `true` if everything is alright (goods are available, etc.) and
    /// the bot is ready to proceed with the order. Use False if there are any
    /// problems.
    pub fn ok(mut self, val: bool) -> Self {
        self.ok = val;
        self
    }

    /// Required if ok is `false`. Error message in human readable form that
    /// explains the reason for failure to proceed with the checkout (e.g.
    /// "Sorry, somebody just bought the last of our amazing black T-shirts
    /// while you were busy filling out your payment details. Please choose a
    /// different color or garment!").
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
