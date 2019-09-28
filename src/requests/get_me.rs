use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::User,
};

#[derive(Debug, Clone)]
/// A simple method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
pub struct GetMe<'a> {
    ctx: RequestContext<'a>,
}

#[async_trait]
impl Request for GetMe<'_> {
    type ReturnValue = User;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl GetMe<'_> {
    pub async fn send(self) -> ResponseResult<User> {
        network::request_multipart(
            self.ctx.client,
            self.ctx.token,
            "getMe",
            None,
        )
        .await
    }
}

impl<'a> GetMe<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>) -> Self {
        GetMe { ctx }
    }
}
