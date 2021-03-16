use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatInviteLink, ChatMember, User};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat; for
    /// joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
}
