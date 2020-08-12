use serde::{Deserialize, Serialize};

use crate::types::PhotoSize;

/// This object represent a user's profile pictures.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofilephotos).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: u32,

    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<PhotoSize>>,
}

impl UserProfilePhotos {
    pub fn new<P1, P2>(total_count: u32, photos: P1) -> Self
    where
        P1: Into<Vec<P2>>,
        P2: Into<Vec<PhotoSize>>,
    {
        Self { total_count, photos: photos.into().into_iter().map(Into::into).collect() }
    }

    pub fn total_count(mut self, val: u32) -> Self {
        self.total_count = val;
        self
    }

    pub fn photos<P1, P2>(mut self, val: P1) -> Self
    where
        P1: Into<Vec<P2>>,
        P2: Into<Vec<PhotoSize>>,
    {
        self.photos = val.into().into_iter().map(Into::into).collect();
        self
    }
}
