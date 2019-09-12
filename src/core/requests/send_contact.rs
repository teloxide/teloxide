use crate::core::requests::RequestContext;

#[derive(Debug, Clone, Serialize)]
struct SendContact<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
