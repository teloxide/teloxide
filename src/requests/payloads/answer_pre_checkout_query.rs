use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::True,
};

/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use False if there are any problems.
    ok: bool,
    /// Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    error_message: Option<String>,
}

impl Method for AnswerPreCheckoutQuery {
    type Output = True;

    const NAME: &'static str = "answerPreCheckoutQuery";
}

impl json::Payload for AnswerPreCheckoutQuery {}

impl dynamic::Payload for AnswerPreCheckoutQuery {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl AnswerPreCheckoutQuery {
    pub fn new<P>(pre_checkout_query_id: P, ok: bool) -> Self
    where
        P: Into<String>
    {
        let pre_checkout_query_id = pre_checkout_query_id.into();
        Self {
            pre_checkout_query_id,
            ok,
            error_message: None,
        }
    }
}

impl json::Request<'_, AnswerPreCheckoutQuery> {
    pub fn pre_checkout_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.pre_checkout_query_id = val.into();
        self
    }

    pub fn ok(mut self, val: bool) -> Self {
        self.payload.ok = val;
        self
    }

    pub fn error_message<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.error_message = Some(val.into());
        self
    }
}
                 