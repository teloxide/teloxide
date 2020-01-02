use crate::{
    network,
    requests::{Request, ResponseResult},
    types::User,
};
use serde::{Deserialize, Serialize};

/// A filter method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
///
/// [`User`]: crate::types::User
#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize, Serialize,
)]
pub struct GetMe;

#[async_trait::async_trait]
impl Request<User> for GetMe {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<User> {
        network::request_json(
            bot.client(),
            bot.token(),
            "getMe",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl GetMe {
    pub fn new() -> Self {
        GetMe
    }
}
