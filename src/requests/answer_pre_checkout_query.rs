use async_trait::async_trait;

use crate::bot::Bot;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
};

#[derive(Debug, Serialize, Clone)]
/// Once the user has confirmed their payment and shipping details, the Bot API
/// sends the final confirmation in the form of an [`Update`] with the field
/// pre_checkout_query. Use this method to respond to such pre-checkout queries.
/// On success, True is returned. Note: The Bot API must receive an answer
/// within 10 seconds after the pre-checkout query was sent.
///
/// [`Update`]: crate::types::Update
pub struct AnswerPreCheckoutQuery<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,

    /// Specify True if everything is alright (goods are available, etc.) and
    /// the bot is ready to proceed with the order. Use False if there are any
    /// problems.
    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is False. Error message in human readable form that
    /// explains the reason for failure to proceed with the checkout (e.g.
    /// "Sorry, somebody just bought the last of our amazing black T-shirts
    /// while you were busy filling out your payment details. Please choose a
    /// different color or garment!"). Telegram will display this message to
    /// the user.
    pub error_message: Option<String>,
}

#[async_trait]
impl Request for AnswerPreCheckoutQuery<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl AnswerPreCheckoutQuery<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "answerPreCheckoutQuery",
            &self,
        )
        .await
    }
}

impl<'a> AnswerPreCheckoutQuery<'a> {
    pub(crate) fn new<S, B>(
        bot: &'a Bot,
        pre_checkout_query_id: S,
        ok: B,
    ) -> Self
    where
        S: Into<String>,
        B: Into<bool>,
    {
        Self {
            bot,
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok: ok.into(),
            error_message: None,
        }
    }

    pub fn pre_checkout_query_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.pre_checkout_query_id = value.into();
        self
    }

    pub fn ok<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.ok = value.into();
        self
    }

    pub fn error_message<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.error_message = Some(value.into());
        self
    }
}
