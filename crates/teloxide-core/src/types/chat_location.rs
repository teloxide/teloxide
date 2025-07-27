use serde::{Deserialize, Serialize};

use crate::types::Location;

/// Represents a location to which a chat is connected.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live
    /// location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner.
    pub address: String,
}
