use crate::core::network;
use crate::core::requests::{Request, RequestFuture, RequestInfo, ResponseResult};
use crate::core::types::User;

#[derive(Debug, Constructor)]
pub struct GetMe {
    info: RequestInfo,
}

impl Request for GetMe {
    type ReturnValue = User;

    fn send(self) -> RequestFuture<ResponseResult<Self::ReturnValue>> {
        Box::new(async move {
            network::request(&self.info.client, &self.info.token, "getMe", None).await
        })
    }
}
