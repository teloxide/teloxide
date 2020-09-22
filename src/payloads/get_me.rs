use serde::{Deserialize, Serialize};

use crate::{
    requests::{HasPayload, Payload},
    types::User,
};

/// A filter method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
///
/// [`User`]: crate::types::User
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize, Serialize)]
pub struct GetMe {}

impl GetMe {
    pub const fn new() -> Self {
        GetMe {}
    }
}

impl Payload for GetMe {
    type Output = User;

    const NAME: &'static str = "getMe";
}

pub trait GetMeSetters: HasPayload<Payload = GetMe> + Sized {}

impl<P> GetMeSetters for P where P: HasPayload<Payload = GetMe> {}
