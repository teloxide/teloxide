use crate::core::requests::RequestContext;

#[derive(Debug, Clone, Serialize)]
struct SendChatAction<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
