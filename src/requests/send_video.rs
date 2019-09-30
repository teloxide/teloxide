use crate::requests::RequestContext;
///TODO: add implementation
#[derive(Debug, Clone, Serialize)]
pub struct SendVideo<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
