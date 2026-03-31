use serde::{Deserialize, Serialize};

use crate::types::PhotoSize;

/// This object represent a user's profile pictures.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofilephotos).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: u32,

    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<PhotoSize>>,
}
