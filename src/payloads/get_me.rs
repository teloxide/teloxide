use serde::{Deserialize, Serialize};

use crate::types::User;

impl_payload! {
    /// A filter method for testing your bot's auth token. Requires no parameters.
    /// Returns basic information about the bot in form of a [`User`] object.
    ///
    /// [`User`]: crate::types::User
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize, Serialize)]
    pub GetMe (GetMeSetters) => User {}
}
