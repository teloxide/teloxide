use serde::{Deserialize, Serialize};

use crate::types::BackgroundType;

/// This object represents a chat background.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatBackground {
    /// Type of the background
    pub r#type: BackgroundType,
}
