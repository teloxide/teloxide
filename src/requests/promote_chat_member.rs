use crate::requests::RequestContext;

///TODO: add implementation
#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
