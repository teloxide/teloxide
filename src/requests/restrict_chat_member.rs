use crate::requests::RequestContext;
//TODO:: need implementation

#[derive(Debug, Clone, Serialize)]
struct RestrictChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
