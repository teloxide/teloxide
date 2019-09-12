use crate::core::requests::RequestContext;

#[derive(Debug, Clone, Serialize)]
struct RestrictChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
