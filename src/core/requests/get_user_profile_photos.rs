use crate::core::requests::RequestContext;

#[derive(Debug, Clone, Serialize)]
struct GetUserProfilePhotos<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
