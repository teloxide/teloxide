use crate::core::{
    types::User,
    network::{
        request, ResponseResult,
    },
    requests::{
        Request, RequestInfo, RequestFuture,
    }
};


#[derive(Debug, Constructor)]
pub struct GetMe {
    info: RequestInfo
}

impl Request for GetMe {
    type ReturnValue = User;

    fn send(self) -> RequestFuture<ResponseResult<Self::ReturnValue>> {
        Box::new(async {
            request(&self.info.client, &self.info.token, "getMe", None).await
        })
    }
}
