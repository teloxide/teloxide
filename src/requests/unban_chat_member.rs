use crate::requests::RequestContext;
//TODO:: need implementation

#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
