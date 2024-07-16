use serde::{Deserialize, Serialize};

use crate::types::{
    ChatFullInfo, ChatId, ChatLocation, ChatPermissions, ChatPhoto, Message, Seconds, True, User,
};

/// This object represents a chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chat).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    /// A unique identifier for this chat.
    pub id: ChatId,

    #[serde(flatten)]
    pub kind: ChatKind,

    /// A chat photo. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub photo: Option<ChatPhoto>,

    /// The most recent pinned message (by sending date). Returned only in
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub pinned_message: Option<Box<Message>>,

    /// The time after which all messages sent to the chat will be automatically
    /// deleted; in seconds. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub message_auto_delete_time: Option<Seconds>,

    /// `true`, if non-administrators can only get the list of bots and
    /// administrators in the chat. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_hidden_members: bool,

    /// `true`, if aggressive anti-spam checks are enabled in the supergroup.
    /// The field is only available to chat administrators. Returned only in
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_aggressive_anti_spam_enabled: bool,

    #[serde(flatten)]
    pub chat_full_info: ChatFullInfo,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatKind {
    Public(ChatPublic),
    Private(ChatPrivate),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatPublic {
    /// A title, for supergroups, channels and group chats.
    pub title: Option<String>,

    #[serde(flatten)]
    pub kind: PublicChatKind,

    /// A description, for groups, supergroups and channel chats. Returned
    /// only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub description: Option<String>,

    /// A chat invite link, for groups, supergroups and channel chats. Each
    /// administrator in a chat generates their own invite links, so the
    /// bot must first generate the link using
    /// [`ExportChatInviteLink`]. Returned only in
    /// [`GetChat`].
    ///
    /// [`ExportChatInviteLink`]:
    /// crate::payloads::ExportChatInviteLink
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub invite_link: Option<String>,

    /// `True`, if messages from the chat can't be forwarded to other chats.
    /// Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub has_protected_content: Option<True>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "serde_helper::ChatPrivate", into = "serde_helper::ChatPrivate")]
pub struct ChatPrivate {
    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// A first name of the other party in a private chat.
    pub first_name: Option<String>,

    /// A last name of the other party in a private chat.
    pub last_name: Option<String>,

    /// Bio of the other party in a private chat. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub bio: Option<String>,

    /// `True`, if privacy settings of the other party in the private chat
    /// allows to use `tg://user?id=<user_id>` links only in chats with the
    /// user. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub has_private_forwards: Option<True>,

    /// `True`, if the privacy settings of the other party restrict sending
    /// voice and video note messages in the private chat. Returned only in
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub has_restricted_voice_and_video_messages: Option<True>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PublicChatKind {
    Channel(PublicChatChannel),
    Group(PublicChatGroup),
    Supergroup(PublicChatSupergroup),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PublicChatChannel {
    /// A username, for private chats, supergroups and channels if available.
    pub username: Option<String>,

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub linked_chat_id: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PublicChatGroup {
    /// A default chat member permissions, for groups and supergroups. Returned
    /// only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub permissions: Option<ChatPermissions>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicChatSupergroup {
    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// If non-empty, the list of all active chat usernames; for private chats,
    /// supergroups and channels. Returned only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub active_usernames: Option<Vec<String>>,

    /// `true`, if the supergroup chat is a forum (has topics enabled).
    #[serde(default)]
    pub is_forum: bool,

    /// For supergroups, name of group sticker set. Returned only from
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub sticker_set_name: Option<String>,

    /// `true`, if the bot can change the group sticker set. Returned only
    /// from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub can_set_sticker_set: Option<bool>,

    /// A default chat member permissions, for groups and supergroups.
    /// Returned only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub permissions: Option<ChatPermissions>,

    /// The minimum allowed delay between consecutive messages sent by each
    /// unpriviledged user. Returned only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub slow_mode_delay: Option<Seconds>,

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub linked_chat_id: Option<i64>,

    /// The location to which the supergroup is connected. Returned only in
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub location: Option<ChatLocation>,

    /// True, if users need to join the supergroup before they can send
    /// messages. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub join_to_send_messages: Option<True>,

    /// True, if all users directly joining the supergroup need to be approved
    /// by supergroup administrators. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub join_by_request: Option<True>,
}

impl Chat {
    #[must_use]
    pub fn is_private(&self) -> bool {
        matches!(self.kind, ChatKind::Private(_))
    }

    #[must_use]
    pub fn is_group(&self) -> bool {
        matches!(self.kind, ChatKind::Public(ChatPublic { kind: PublicChatKind::Group(_), .. }))
    }

    #[must_use]
    pub fn is_supergroup(&self) -> bool {
        matches!(
            self.kind,
            ChatKind::Public(ChatPublic { kind: PublicChatKind::Supergroup(_), .. })
        )
    }

    #[must_use]
    pub fn is_channel(&self) -> bool {
        matches!(self.kind, ChatKind::Public(ChatPublic { kind: PublicChatKind::Channel(_), .. }))
    }

    #[must_use]
    pub fn is_chat(&self) -> bool {
        self.is_private() || self.is_group() || self.is_supergroup()
    }
}

/// Getters
impl Chat {
    /// A title, for supergroups, channels and group chats.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Public(this) => this.title.as_deref(),
            _ => None,
        }
    }

    /// A username, for private chats, supergroups and channels if available.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Public(this) => match &this.kind {
                PublicChatKind::Channel(PublicChatChannel { username, .. })
                | PublicChatKind::Supergroup(PublicChatSupergroup { username, .. }) => {
                    username.as_deref()
                }
                PublicChatKind::Group(_) => None,
            },
            ChatKind::Private(this) => this.username.as_deref(),
        }
    }

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn linked_chat_id(&self) -> Option<i64> {
        match &self.kind {
            ChatKind::Public(this) => match &this.kind {
                PublicChatKind::Channel(PublicChatChannel { linked_chat_id, .. })
                | PublicChatKind::Supergroup(PublicChatSupergroup { linked_chat_id, .. }) => {
                    *linked_chat_id
                }
                PublicChatKind::Group(_) => None,
            },
            _ => None,
        }
    }

    /// A default chat member permissions, for groups and supergroups. Returned
    /// only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn permissions(&self) -> Option<ChatPermissions> {
        if let ChatKind::Public(this) = &self.kind {
            if let PublicChatKind::Group(PublicChatGroup { permissions })
            | PublicChatKind::Supergroup(PublicChatSupergroup { permissions, .. }) = &this.kind
            {
                return *permissions;
            }
        }

        None
    }

    /// For supergroups, name of group sticker set. Returned only from
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn sticker_set_name(&self) -> Option<&str> {
        if let ChatKind::Public(this) = &self.kind {
            if let PublicChatKind::Supergroup(this) = &this.kind {
                return this.sticker_set_name.as_deref();
            }
        }

        None
    }

    /// `true`, if the bot can change the group sticker set. Returned only
    /// from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn can_set_sticker_set(&self) -> Option<bool> {
        if let ChatKind::Public(this) = &self.kind {
            if let PublicChatKind::Supergroup(this) = &this.kind {
                return this.can_set_sticker_set;
            }
        }

        None
    }

    /// The minimum allowed delay between consecutive messages sent by each
    /// unpriviledged user. Returned only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn slow_mode_delay(&self) -> Option<Seconds> {
        if let ChatKind::Public(this) = &self.kind {
            if let PublicChatKind::Supergroup(this) = &this.kind {
                return this.slow_mode_delay;
            }
        }

        None
    }

    /// The location to which the supergroup is connected. Returned only in
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn location(&self) -> Option<&ChatLocation> {
        if let ChatKind::Public(this) = &self.kind {
            if let PublicChatKind::Supergroup(this) = &this.kind {
                return this.location.as_ref();
            }
        }

        None
    }

    /// True, if users need to join the supergroup before they can send
    /// messages. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn join_to_send_messages(&self) -> Option<True> {
        if let ChatKind::Public(this) = &self.kind {
            if let PublicChatKind::Supergroup(this) = &this.kind {
                return this.join_to_send_messages;
            }
        }

        None
    }

    /// True, if all users directly joining the supergroup need to be approved
    /// by supergroup administrators. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn join_by_request(&self) -> Option<True> {
        if let ChatKind::Public(this) = &self.kind {
            if let PublicChatKind::Supergroup(this) = &this.kind {
                return this.join_by_request;
            }
        }

        None
    }

    /// A description, for groups, supergroups and channel chats. Returned
    /// only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Public(this) => this.description.as_deref(),
            _ => None,
        }
    }

    /// A chat invite link, for groups, supergroups and channel chats. Each
    /// administrator in a chat generates their own invite links, so the
    /// bot must first generate the link using
    /// [`ExportChatInviteLink`]. Returned only in
    /// [`GetChat`].
    ///
    /// [`ExportChatInviteLink`]:
    /// crate::payloads::ExportChatInviteLink
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn invite_link(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Public(this) => this.invite_link.as_deref(),
            _ => None,
        }
    }

    /// `True`, if messages from the chat can't be forwarded to other chats.
    /// Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn has_protected_content(&self) -> Option<True> {
        match &self.kind {
            ChatKind::Public(this) => this.has_protected_content,
            _ => None,
        }
    }

    /// A first name of the other party in a private chat.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Private(this) => this.first_name.as_deref(),
            _ => None,
        }
    }

    /// A last name of the other party in a private chat.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Private(this) => this.last_name.as_deref(),
            _ => None,
        }
    }

    /// Bio of the other party in a private chat. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn bio(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Private(this) => this.bio.as_deref(),
            _ => None,
        }
    }

    /// `True`, if privacy settings of the other party in the private chat
    /// allows to use tg://user?id=<user_id> links only in chats with the
    /// user. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[must_use]
    pub fn has_private_forwards(&self) -> Option<True> {
        match &self.kind {
            ChatKind::Private(this) => this.has_private_forwards,
            _ => None,
        }
    }

    /// Returns all users that are "contained" in this `Chat`
    /// structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        crate::util::flatten(self.pinned_message.as_ref().map(|m| m.mentioned_users()))
    }

    /// `{Message, Chat}::mentioned_users` are mutually recursive, as such we
    /// can't use `->impl Iterator` everywhere, as it would make an infinite
    /// type. So we need to box somewhere.
    pub(crate) fn mentioned_users_rec(&self) -> impl Iterator<Item = &User> {
        crate::util::flatten(self.pinned_message.as_ref().map(|m| m.mentioned_users_rec()))
    }
}

mod serde_helper {
    use crate::types::True;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    enum Type {
        #[allow(non_camel_case_types)]
        private,
    }

    #[derive(Serialize, Deserialize)]
    pub(super) struct ChatPrivate {
        /// A dummy field. Used to ensure that the `type` field is equal to
        /// `private`.
        r#type: Type,

        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        bio: Option<String>,
        has_private_forwards: Option<True>,
        has_restricted_voice_and_video_messages: Option<True>,
    }

    impl From<ChatPrivate> for super::ChatPrivate {
        fn from(
            ChatPrivate {
                r#type: _,
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
            }: ChatPrivate,
        ) -> Self {
            Self {
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
            }
        }
    }

    impl From<super::ChatPrivate> for ChatPrivate {
        fn from(
            super::ChatPrivate {
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
            }: super::ChatPrivate,
        ) -> Self {
            Self {
                r#type: Type::private,
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use crate::types::*;

    #[test]
    fn channel_de() {
        let expected = Chat {
            id: ChatId(-1),
            kind: ChatKind::Public(ChatPublic {
                title: None,
                kind: PublicChatKind::Channel(PublicChatChannel {
                    username: Some("channel_name".into()),
                    linked_chat_id: None,
                }),
                description: None,
                invite_link: None,
                has_protected_content: None,
            }),
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
            chat_full_info: ChatFullInfo::default(),
        };
        let actual = from_str(r#"{"id":-1,"type":"channel","username":"channel_name"}"#).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn private_chat_de() {
        assert_eq!(
            Chat {
                id: ChatId(0),
                kind: ChatKind::Private(ChatPrivate {
                    username: Some("username".into()),
                    first_name: Some("Anon".into()),
                    last_name: None,
                    bio: None,
                    has_private_forwards: None,
                    has_restricted_voice_and_video_messages: None,
                }),
                photo: None,
                pinned_message: None,
                message_auto_delete_time: None,
                has_hidden_members: false,
                has_aggressive_anti_spam_enabled: false,
                chat_full_info: ChatFullInfo::default()
            },
            from_str(r#"{"id":0,"type":"private","username":"username","first_name":"Anon"}"#)
                .unwrap()
        );
    }

    #[test]
    fn private_roundtrip() {
        let chat = Chat {
            id: ChatId(0),
            kind: ChatKind::Private(ChatPrivate {
                username: Some("username".into()),
                first_name: Some("Anon".into()),
                last_name: None,
                bio: None,
                has_private_forwards: None,
                has_restricted_voice_and_video_messages: None,
            }),
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
            chat_full_info: ChatFullInfo::default(),
        };

        let json = to_string(&chat).unwrap();
        let chat2 = from_str::<Chat>(&json).unwrap();

        assert_eq!(chat, chat2);
    }

    #[test]
    fn private_chat_de_wrong_type_field() {
        assert!(from_str::<Chat>(r#"{"id":0,"type":"WRONG"}"#).is_err());
    }
}
