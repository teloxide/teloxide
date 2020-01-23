use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::UserProfilePhotos,
    Bot,
};

/// Use this method to get a list of profile pictures for a user.
///
/// [The official docs](https://core.telegram.org/bots/api#getuserprofilephotos).
#[serde_with_macros::skip_serializing_none]
#[derive(Copy, Eq, PartialEq, Debug, Clone, Serialize)]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    user_id: i32,
    offset: Option<i32>,
    limit: Option<i32>,
}

#[async_trait::async_trait]
impl Request for GetUserProfilePhotos<'_> {
    type Output = UserProfilePhotos;

    async fn send(&self) -> ResponseResult<UserProfilePhotos> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getUserProfilePhotos",
            &self,
        )
        .await
    }
}

impl<'a> GetUserProfilePhotos<'a> {
    pub(crate) fn new(bot: &'a Bot, user_id: i32) -> Self {
        Self {
            bot: BotWrapper(bot),
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Unique identifier of the target user.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// Sequential number of the first photo to be returned. By default, all
    /// photos are returned.
    pub fn offset(mut self, val: i32) -> Self {
        self.offset = Some(val);
        self
    }

    /// Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted.
    ///
    /// Defaults to 100.
    pub fn limit(mut self, val: i32) -> Self {
        self.limit = Some(val);
        self
    }
}
