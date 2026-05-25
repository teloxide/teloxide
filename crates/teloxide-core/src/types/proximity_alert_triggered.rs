use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object represents the content of a service message, sent whenever a
/// user in the chat triggers a proximity alert set by another user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert.
    pub traveler: User,

    /// User that set the alert.
    pub watcher: User,

    /// The distance between the users.
    pub distance: u32,
}
