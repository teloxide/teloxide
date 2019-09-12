use crate::core::{
    network,
    requests::{Request, RequestContext, RequestFuture, ResponseResult},
    types::User,
};

#[derive(Debug, Clone)]
/// A simple method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
pub struct GetMe<'a> {
    ctx: RequestContext<'a>,
}

impl<'a> Request<'a> for GetMe<'a> {
    type ReturnValue = User;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(network::request_multipart(
            self.ctx.client,
            self.ctx.token,
            "getMe",
            None,
        ))
    }
}

impl<'a> GetMe<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>) -> Self {
        GetMe { ctx }
    }
}
