use crate::core::requests::RequestContext;

#[derive(Debug, Clone, Serialize)]
struct KickChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

}
