use crate::{
    requests::{Method, dynamic},
    types::User,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
/// A filter method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
///
/// [`User`]: crate::types::User
pub struct GetMe;

impl Method for GetMe {
    type Output = User;

    const NAME: &'static str = "getMe";
}

impl dynamic::Payload for GetMe {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Simple
    }
}
