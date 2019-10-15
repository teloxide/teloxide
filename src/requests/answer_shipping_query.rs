use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::{ShippingOption, True},
};

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
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False
    /// if there are any problems (for example, if delivery to the specified
    /// address is not possible)
    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is True. A JSON-serialized array of available shipping
    /// options.
    pub shipping_options: Option<Vec<ShippingOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is False. Error message in human readable form that
    /// explains why it is impossible to complete the order (e.g. "Sorry,
    /// delivery to your desired address is unavailable'). Telegram will
    /// display this message to the user.
    pub error_message: Option<String>,
}

#[async_trait]
impl Request for AnswerShippingQuery<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
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
    pub(crate) fn new<S, B>(
        ctx: RequestContext<'a>,
        shipping_query_id: S,
        ok: B,
    ) -> Self
    where
        S: Into<String>,
        B: Into<bool>,
    {
        Self {
            ctx,
            shipping_query_id: shipping_query_id.into(),
            ok: ok.into(),
            shipping_options: None,
            error_message: None,
        }
    }

    pub fn shipping_query_id<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.shipping_query_id = value.into();
        self
    }

    pub fn ok<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.ok = value.into();
        self
    }

    pub fn shipping_options<T>(mut self, value: T) -> Self
    where
        T: Into<Vec<ShippingOption>>,
    {
        self.shipping_options = Some(value.into());
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
