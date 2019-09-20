use crate::requests::RequestContext;
//TODO:: need implementation
/// Use this method to kick a user from a group, a supergroup or a channel. In
/// the case of supergroups and channels, the user will not be able to return to
/// the group on their own using invite links, etc., unless unbanned first. The
/// bot must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct KickChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
}
