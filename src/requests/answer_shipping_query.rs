use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::{ShippingOption, True},
};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
/// If you sent an invoice requesting a shipping address and the parameter
/// is_flexible was specified, the Bot API will send an [`Update`] with a
/// shipping_query field to the bot. Use this method to reply to shipping
/// queries. On success, True is returned.
///
/// [`Update`]: crate::types::Update
pub struct AnswerShippingQuery<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    /// Unique identifier for the query to be answered
    pub shipping_query_id: Cow<'a, str>,
    /// Specify True if delivery to the specified address is possible and False
    /// if there are any problems (for example, if delivery to the specified
    /// address is not possible)
    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is True. A JSON-serialized array of available shipping
    /// options.
    pub shipping_options: Option<Cow<'a, [ShippingOption]>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is False. Error message in human readable form that
    /// explains why it is impossible to complete the order (e.g. "Sorry,
    /// delivery to your desired address is unavailable'). Telegram will
    /// display this message to the user.
    pub error_message: Option<Cow<'a, str>>,
}

#[async_trait]
impl Request for AnswerShippingQuery<'_> {
    type ReturnValue = True;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl AnswerShippingQuery<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "answerShippingQuery",
            &self,
        )
        .await
    }
}

impl<'a> AnswerShippingQuery<'a> {
    pub(crate) fn new<C>(
        ctx: RequestContext<'a>,
        shipping_query_id: C,
        ok: bool,
    ) -> Self where C: Into<Cow<'a, str>> {
        Self {
            ctx,
            shipping_query_id: shipping_query_id.into(),
            ok,
            shipping_options: None,
            error_message: None,
        }
    }

    pub fn shipping_query_id<T>(mut self, shipping_query_id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.shipping_query_id = shipping_query_id.into();
        self
    }

    pub fn ok<T>(mut self, ok: T) -> Self
    where
        T: Into<bool>,
    {
        self.ok = ok.into();
        self
    }

    pub fn shipping_options<T>(mut self, shipping_options: T) -> Self
    where
        T: Into<Vec<Cow<'a, ShippingOption>>>,
    {
        self.shipping_options = Some(shipping_options.into());
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
