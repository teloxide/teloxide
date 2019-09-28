use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::Update,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetUpdates<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    pub offset: Option<i32>,
    pub limit: Option<u8>,
    pub timeout: Option<u32>,
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
}

#[async_trait]
impl Request for GetUpdates<'_> {
    type ReturnValue = Vec<Update>;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl GetUpdates<'_> {
    pub async fn send(self) -> ResponseResult<Vec<Update>> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "getUpdates",
            &self,
        )
        .await
    }
}

impl<'a> GetUpdates<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>) -> Self {
        Self {
            ctx,
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    pub fn offset<T>(mut self, offset: T) -> Self
    where
        T: Into<i32>,
    {
        self.offset = Some(offset.into());
        self
    }

    pub fn limit<T>(mut self, limit: T) -> Self
    where
        T: Into<u8>,
    {
        self.limit = Some(limit.into());
        self
    }

    pub fn timeout<T>(mut self, timeout: T) -> Self
    where
        T: Into<u32>,
    {
        self.timeout = Some(timeout.into());
        self
    }

    pub fn allowed_updates<T>(mut self, allowed_updates: T) -> Self
    where
        T: Into<Vec<AllowedUpdate>>,
    {
        self.allowed_updates = Some(allowed_updates.into());
        self
    }
}
