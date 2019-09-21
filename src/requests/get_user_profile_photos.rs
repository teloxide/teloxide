use crate::network;
use crate::requests::{Request, RequestContext, RequestFuture, ResponseResult};
use crate::types::UserProfilePhotos;

///Use this method to get a list of profile pictures for a user. Returns a
/// UserProfilePhotos object.
#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier of the target user
    user_id: i32,
    /// Sequential number of the first photo to be returned. By default, all
    /// photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    ///Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl<'a> Request<'a> for GetUserProfilePhotos<'a> {
    type ReturnValue = UserProfilePhotos;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "getUserProfilePhotos",
                &self,
            )
            .await
        })
    }
}

impl<'a> GetUserProfilePhotos<'a> {
    pub fn new(ctx: RequestContext<'a>, user_id: i32) -> Self {
        Self {
            ctx,
            user_id,
            offset: None,
            limit: None,
        }
    }

    pub fn user_id<T>(mut self, user_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.user_id = user_id.into();
        self
    }

    pub fn offset<T>(mut self, offset: T) -> Self
    where
        T: Into<i64>,
    {
        self.offset = Some(offset.into());
        self
    }

    pub fn limit<T>(mut self, limit: T) -> Self
        where
            T: Into<i64>,
    {
        self.limit = Some(limit.into());
        self
    }

}
