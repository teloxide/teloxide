use crate::core::types::ShippingOption;
use crate::core::requests::{RequestContext, Request, RequestFuture, ResponseResult};
use crate::core::network;

#[derive(Debug, Clone, Serialize)]
/// If you sent an invoice requesting a shipping address and the parameter
/// is_flexible was specified, the Bot API will send an [`Update`] with a
/// shipping_query field to the bot. Use this method to reply to shipping
/// queries. On success, True is returned.
pub struct AnswerShippingQuery<'a> {
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

impl<'a> Request<'a> for AnswerShippingQuery<'a> {
    type ReturnValue = bool;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "answerShippingQuery",
                &self
            )
        })
    }
}

impl<'a> AnswerShippingQuery<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        shipping_query_id: String,
        ok: bool
    ) -> Self {
        Self {
            ctx,
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None
        }
    }

    pub fn shipping_query_id<T>(mut self, shipping_query_id: T) -> Self
        where T: Into<String>
    {
        self.shipping_query_id = shipping_query_id.into();
        self
    }

    pub fn ok<T>(mut self, ok: T) -> Self
        where T: Into<bool>
    {
        self.ok = ok.into();
        self
    }

    pub fn shipping_options<T>(mut self, shipping_options: T) -> Self
        where T: Into<Vec<ShippingOption>>
    {
        self.shipping_options = shipping_options;
        self
    }

    pub fn error_message<T>(mut self, error_message: T) -> Self
        where T: Into<String>
    {
        self.error_message = Some(error_message.into());
        self
    }
}
