use serde::{Deserialize, Serialize};
use crate::{
    requests::{dynamic, json, Method},
    types::User,
};

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize, Serialize,
)]
/// A filter method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
///
/// [`User`]: crate::types::User
pub struct GetMe {}

impl Method for GetMe {
    type Output = User;

    const NAME: &'static str = "getMe";
}

impl json::Payload for GetMe {}

impl dynamic::Payload for GetMe {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetMe {
    pub fn new() -> Self {
        GetMe {}
    }
}
