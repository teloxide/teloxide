use crate::requests::RequestContext;

//TODO: complete implementation after user_profile_fotos will be added to
// types/mod.rs
///Use this method to get a list of profile pictures for a user. Returns a
/// UserProfilePhotos object.
#[derive(Debug, Clone, Serialize)]
struct GetUserProfilePhotos<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier of the target user
    user_id: i32,
    /// Sequential number of the first photo to be returned. By default, all
    /// photos are returned.
    offset: Option<i64>,
    ///Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted. Defaults to 100.
    limit: Option<i64>,
}
