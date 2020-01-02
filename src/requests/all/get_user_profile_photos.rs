use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::UserProfilePhotos,
    Bot,
};

/// Use this method to get a list of profile pictures for a user. Returns a
/// UserProfilePhotos object.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier of the target user
    user_id: i32,
    /// Sequential number of the first photo to be returned. By default, all
    /// photos are returned.
    offset: Option<i32>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted. Defaults to 100.
    limit: Option<i32>,
}

#[async_trait::async_trait]
impl Request<UserProfilePhotos> for GetUserProfilePhotos<'_> {
    async fn send(&self) -> ResponseResult<UserProfilePhotos> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getUserProfilePhotos",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> GetUserProfilePhotos<'a> {
    pub(crate) fn new(bot: &'a Bot, user_id: i32) -> Self {
        Self {
            bot,
            user_id,
            offset: None,
            limit: None,
        }
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    pub fn offset(mut self, val: i32) -> Self {
        self.offset = Some(val);
        self
    }

    pub fn limit(mut self, val: i32) -> Self {
        self.limit = Some(val);
        self
    }
}
