use crate::core::requests::ResponseResult;
use crate::core::{network::request, network::request, types::User};

#[derive(Debug, Constructor)]
pub struct GetMe {
    info: RequestInfo,
}

impl Request for GetMe {
    type ReturnValue = User;

    fn send(self) -> RequestFuture<ResponseResult<Self::ReturnValue>> {
        Box::new(async move { request(&self.info.client, &self.info.token, "getMe", None).await })
    }
}
