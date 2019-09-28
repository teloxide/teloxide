use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
};

#[derive(Debug, Serialize, Clone)]
/// Once the user has confirmed their payment and shipping details, the Bot API
/// sends the final confirmation in the form of an [`Update`] with the field
/// pre_checkout_query. Use this method to respond to such pre-checkout queries.
/// On success, True is returned. Note: The Bot API must receive an answer
/// within 10 seconds after the pre-checkout query was sent.
pub struct AnswerPreCheckoutQuery<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

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
    type ReturnValue = bool;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl AnswerPreCheckoutQuery<'_> {
    async fn send(self) -> ResponseResult<bool> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "answerPreCheckoutQuery",
            &self,
        )
        .await
    }
}

impl<'a> AnswerPreCheckoutQuery<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        pre_checkout_query_id: String,
        ok: bool,
    ) -> Self {
        Self {
            ctx,
            pre_checkout_query_id,
            ok,
            error_message: None,
        }
    }

    pub fn pre_checkout_query_id<T>(mut self, pre_checkout_query_id: T) -> Self
    where
        T: Into<String>,
    {
        self.pre_checkout_query_id = pre_checkout_query_id.into();
        self
    }

    pub fn ok<T>(mut self, ok: T) -> Self
    where
        T: Into<bool>,
    {
        self.ok = ok.into();
        self
    }

    pub fn error_message<T>(mut self, error_message: T) -> Self
    where
        T: Into<String>,
    {
        self.error_message = Some(error_message.into());
        self
    }
}
