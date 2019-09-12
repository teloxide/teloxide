use crate::core::requests::RequestContext;

#[derive(Debug, Clone, Serialize)]
struct SendPoll<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
