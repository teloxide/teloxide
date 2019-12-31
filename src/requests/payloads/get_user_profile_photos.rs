use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::UserProfilePhotos,
};

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetUserProfilePhoto {
    /// Unique identifier of the target user
    user_id: i32,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    offset: Option<i32>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    limit: Option<i32>,
}

impl Method for GetUserProfilePhoto {
    type Output = UserProfilePhotos;

    const NAME: &'static str = "getUserProfilePhotos";
}

impl json::Payload for GetUserProfilePhoto {}

impl dynamic::Payload for GetUserProfilePhoto {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetUserProfilePhoto {
    pub fn new(user_id: i32) -> Self {
        
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }
}

impl json::Request<'_, GetUserProfilePhoto> {
    pub fn user_id(mut self, val: i32) -> Self {
        self.payload.user_id = val;
        self
    }

    pub fn offset(mut self, val: i32) -> Self {
        self.payload.offset = Some(val);
        self
    }

    pub fn limit(mut self, val: i32) -> Self {
        self.payload.limit = Some(val);
        self
    }
}
                 