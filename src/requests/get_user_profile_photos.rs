use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::UserProfilePhotos,
};

///Use this method to get a list of profile pictures for a user. Returns a
/// UserProfilePhotos object.
#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// Sequential number of the first photo to be returned. By default, all
    /// photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    ///Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[async_trait]
impl Request for GetUserProfilePhotos<'_> {
    type ReturnValue = UserProfilePhotos;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl GetUserProfilePhotos<'_> {
    async fn send(self) -> ResponseResult<UserProfilePhotos> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "getUserProfilePhotos",
            &self,
        )
        .await
    }
}

impl<'a> GetUserProfilePhotos<'a> {
    pub fn new<U>(ctx: RequestContext<'a>, user_id: U) -> Self
    where
        U: Into<i32>,
    {
        Self {
            ctx,
            user_id: user_id.into(),
            offset: None,
            limit: None,
        }
    }

    pub fn user_id<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.user_id = value.into();
        self
    }

    pub fn offset<T>(mut self, value: T) -> Self
    where
        T: Into<i64>,
    {
        self.offset = Some(value.into());
        self
    }

    pub fn limit<T>(mut self, value: T) -> Self
    where
        T: Into<i64>,
    {
        self.limit = Some(value.into());
        self
    }
}
