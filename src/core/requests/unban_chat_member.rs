use crate::core::requests::RequestContext;

#[derive(Debug, Clone, Serialize)]
struct UnbanChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
