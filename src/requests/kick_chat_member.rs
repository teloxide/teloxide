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

    chat_id 	Integer or String 	Yes 	Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    user_id 	Integer 	Yes 	Unique identifier of the target user
    until_date 	Integer 	Optional 	Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
}
