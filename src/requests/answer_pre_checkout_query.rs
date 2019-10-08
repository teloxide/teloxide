use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::True,
};
use std::borrow::Cow;

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
    ctx: RequestContext<'a>,

    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: Cow<'a, str>,

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
    pub error_message: Option<Cow<'a, str>>,
}

#[async_trait]
impl Request for AnswerPreCheckoutQuery<'_> {
    type ReturnValue = True;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl AnswerPreCheckoutQuery<'_> {
    pub async fn send(self) -> ResponseResult<True> {
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
    pub(crate) fn new<C>(
        ctx: RequestContext<'a>,
        pre_checkout_query_id: C,
        ok: bool,
    ) -> Self where C: Into<Cow<'a, str>> {
        Self {
            ctx,
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok,
            error_message: None,
        }
    }

    pub fn pre_checkout_query_id<T>(mut self, pre_checkout_query_id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
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
        T: Into<Cow<'a, str>>,
    {
        self.error_message = Some(error_message.into());
        self
    }
}
