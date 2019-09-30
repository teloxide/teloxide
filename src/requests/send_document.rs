use crate::requests::RequestContext;
///TODO: add implementation
#[derive(Debug, Clone, Serialize)]
pub struct SendDocument<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
