use chrono::{DateTime, Utc};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

use crate::types::{
    AcceptedGiftTypes, Birthdate, BusinessIntro, BusinessLocation, BusinessOpeningHours, Chat,
    ChatId, ChatLocation, ChatPermissions, ChatPhoto, Message, ReactionType, Seconds, User,
};

/// Custom emoji identifier.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct CustomEmojiId(pub String);

/// This object contains full information about a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfo {
    /// A unique identifier for this chat.
    pub id: ChatId,

    #[serde(flatten)]
    pub kind: ChatFullInfoKind,

    /// A chat photo.
    pub photo: Option<ChatPhoto>,

    /// The most recent pinned message (by sending date).
    pub pinned_message: Option<Box<Message>>,

    /// The time after which all messages sent to the chat will be automatically
    /// deleted; in seconds.
    pub message_auto_delete_time: Option<Seconds>,

    /// `true`, if non-administrators can only get the list of bots and
    /// administrators in the chat.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_hidden_members: bool,

    /// `true`, if aggressive anti-spam checks are enabled in the supergroup.
    /// The field is only available to chat administrators.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_aggressive_anti_spam_enabled: bool,

    /// Information about types of gifts that are accepted by the chat or by the
    /// corresponding user for private chats
    pub accepted_gift_types: AcceptedGiftTypes,

    /// Identifier of the accent color for the chat name and backgrounds of the
    /// chat photo, reply header, and link preview. See [accent colors] for more
    /// details.
    ///
    /// [accent colors]: https://core.telegram.org/bots/api#accent-colors
    pub accent_color_id: Option<u8>,

    /// Custom emoji identifier of the emoji chosen by the chat for the reply
    /// header and link preview background
    pub background_custom_emoji_id: Option<CustomEmojiId>,

    /// Identifier of the accent color for the chat's profile background. See
    /// [profile accent colors] for more details.
    ///
    /// [profile accent colors]: https://core.telegram.org/bots/api#profile-accent-colors
    pub profile_accent_color_id: Option<u8>,

    /// Custom emoji identifier of the emoji chosen by the chat for its profile
    /// background
    pub profile_background_custom_emoji_id: Option<CustomEmojiId>,

    /// Custom emoji identifier of emoji status of the other party in a private
    /// chat.
    pub emoji_status_custom_emoji_id: Option<CustomEmojiId>,

    /// Expiration date of the emoji status of the chat or the other party in a
    /// private chat, in Unix time, if any
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub emoji_status_expiration_date: Option<DateTime<Utc>>,

    /// True, if new chat members will have access to old messages; available
    /// only to chat administrators.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_visible_history: bool,

    /// The maximum number of reactions that can be set on a message in the
    /// chat
    pub max_reaction_count: u8,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatFullInfoKind {
    Public(Box<ChatFullInfoPublic>),
    Private(Box<ChatFullInfoPrivate>),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfoPublic {
    /// A title, for supergroups, channels and group chats.
    pub title: Option<String>,

    #[serde(flatten)]
    pub kind: ChatFullInfoPublicKind,

    /// A description, for groups, supergroups and channel chats.
    pub description: Option<String>,

    /// A chat invite link, for groups, supergroups and channel chats. Each
    /// administrator in a chat generates their own invite links, so the
    /// bot must first generate the link using
    /// [`ExportChatInviteLink`].
    ///
    /// [`ExportChatInviteLink`]:
    /// crate::payloads::ExportChatInviteLink
    pub invite_link: Option<String>,

    /// `true`, if messages from the chat can't be forwarded to other chats.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_protected_content: bool,

    /// List of available reactions allowed in the chat. If omitted, then all
    /// emoji reactions are allowed.
    pub available_reactions: Option<Vec<ReactionType>>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "serde_helper::ChatPrivateFullInfo", into = "serde_helper::ChatPrivateFullInfo")]
pub struct ChatFullInfoPrivate {
    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// A first name of the other party in a private chat.
    pub first_name: Option<String>,

    /// A last name of the other party in a private chat.
    pub last_name: Option<String>,

    /// Bio of the other party in a private chat.
    pub bio: Option<String>,

    /// `true`, if privacy settings of the other party in the private chat
    /// allows to use `tg://user?id=<user_id>` links only in chats with the
    /// user.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_private_forwards: bool,

    /// `true`, if the privacy settings of the other party restrict sending
    /// voice and video note messages in the private chat.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_restricted_voice_and_video_messages: bool,

    /// For private chats, the personal channel of the user.
    pub personal_chat: Option<Box<Chat>>,

    /// For private chats, the date of birth of the user.
    pub birthdate: Option<Birthdate>,

    /// For private chats with business accounts, the intro of the business.
    pub business_intro: Option<BusinessIntro>,

    /// For private chats with business accounts, the location of the business.
    pub business_location: Option<BusinessLocation>,

    /// For private chats with business accounts, the opening hours of the
    /// business.
    pub business_opening_hours: Option<BusinessOpeningHours>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ChatFullInfoPublicKind {
    Channel(ChatFullInfoPublicChannel),
    Group(ChatFullInfoPublicGroup),
    Supergroup(ChatFullInfoPublicSupergroup),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfoPublicChannel {
    /// A username, for private chats, supergroups and channels if available.
    pub username: Option<String>,

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa.
    // SMELL: TBA uses here Integer instead of ChatId so we do that too :c
    pub linked_chat_id: Option<i64>,

    /// `true`, if paid media messages can be sent or forwarded to the channel
    /// chat. The field is available only for channel chats.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_send_paid_media: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfoPublicGroup {
    /// A default chat member permissions, for groups and supergroups.
    pub permissions: Option<ChatPermissions>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfoPublicSupergroup {
    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// If non-empty, the list of all active chat usernames; for private chats,
    /// supergroups and channels.
    pub active_usernames: Option<Vec<String>>,

    /// `true`, if the supergroup chat is a forum (has topics enabled).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_forum: bool,

    /// `true`, if the chat is the direct messages chat of a channel
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_direct_messages: bool,

    /// Information about the corresponding channel chat; for direct messages
    /// chats only
    pub parent_chat: Option<Chat>,

    /// For supergroups, name of group sticker set.
    pub sticker_set_name: Option<String>,

    /// `true`, if the bot can change the group sticker set.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_set_sticker_set: bool,

    /// For supergroups, the name of the group's custom emoji sticker set.
    /// Custom emoji from this set can be used by all users and bots in the
    /// group.
    pub custom_emoji_sticker_set_name: Option<String>,

    /// A default chat member permissions, for groups and supergroups.
    pub permissions: Option<ChatPermissions>,

    /// For supergroups, the minimum allowed delay between consecutive messages
    /// sent by each unprivileged user.
    pub slow_mode_delay: Option<Seconds>,

    /// For supergroups, the minimum number of boosts that a non-administrator
    /// user needs to add in order to ignore slow mode and chat permissions.
    pub unrestrict_boost_count: Option<u16>,

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa.
    pub linked_chat_id: Option<i64>,

    /// The location to which the supergroup is connected.
    pub location: Option<ChatLocation>,

    /// `true`, if users need to join the supergroup before they can send
    /// messages.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub join_to_send_messages: bool,

    /// `true`, if all users directly joining the supergroup without using an
    /// invite link need to be approved by supergroup administrators.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub join_by_request: bool,
}

impl ChatFullInfo {
    #[must_use]
    pub fn is_private(&self) -> bool {
        matches!(self.kind, ChatFullInfoKind::Private(_))
    }

    /// Note that Group and Supergroup are two similar but still different types
    /// of chat groups! Use [`is_group_chat`] to check if it's any kind of group
    /// chat.
    ///
    /// See [blog post](https://telegram.org/blog/supergroups#supergroups)
    ///
    /// [`is_group_chat`]: Self::is_group_chat
    #[must_use]
    pub fn is_group(&self) -> bool {
        if let ChatFullInfoKind::Public(chat_pub) = &self.kind {
            matches!(**chat_pub, ChatFullInfoPublic { kind: ChatFullInfoPublicKind::Group(_), .. })
        } else {
            false
        }
    }

    /// Note that Group and Supergroup are two similar but still different types
    /// of chat groups! Use [`is_group_chat`] to check if it's any kind of group
    /// chat.
    ///
    /// See [blog post](https://telegram.org/blog/supergroups#supergroups)
    ///
    /// [`is_group_chat`]: Self::is_group_chat
    #[must_use]
    pub fn is_supergroup(&self) -> bool {
        if let ChatFullInfoKind::Public(chat_pub) = &self.kind {
            matches!(
                **chat_pub,
                ChatFullInfoPublic { kind: ChatFullInfoPublicKind::Supergroup(_), .. }
            )
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_channel(&self) -> bool {
        if let ChatFullInfoKind::Public(chat_pub) = &self.kind {
            matches!(
                **chat_pub,
                ChatFullInfoPublic { kind: ChatFullInfoPublicKind::Channel(_), .. }
            )
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_chat(&self) -> bool {
        self.is_private() || self.is_group() || self.is_supergroup()
    }

    #[must_use]
    pub fn is_group_chat(&self) -> bool {
        self.is_group() || self.is_supergroup()
    }
}

/// Getters
impl ChatFullInfo {
    /// A title, for supergroups, channels and group chats.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match &self.kind {
            ChatFullInfoKind::Public(this) => this.title.as_deref(),
            _ => None,
        }
    }

    /// A username, for private chats, supergroups and channels if available.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match &self.kind {
            ChatFullInfoKind::Public(this) => match &this.kind {
                ChatFullInfoPublicKind::Channel(ChatFullInfoPublicChannel { username, .. })
                | ChatFullInfoPublicKind::Supergroup(ChatFullInfoPublicSupergroup {
                    username,
                    ..
                }) => username.as_deref(),
                ChatFullInfoPublicKind::Group(_) => None,
            },
            ChatFullInfoKind::Private(this) => this.username.as_deref(),
        }
    }

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa.
    #[must_use]
    pub fn linked_chat_id(&self) -> Option<i64> {
        match &self.kind {
            ChatFullInfoKind::Public(this) => match &this.kind {
                ChatFullInfoPublicKind::Channel(ChatFullInfoPublicChannel {
                    linked_chat_id,
                    ..
                })
                | ChatFullInfoPublicKind::Supergroup(ChatFullInfoPublicSupergroup {
                    linked_chat_id,
                    ..
                }) => *linked_chat_id,
                ChatFullInfoPublicKind::Group(_) => None,
            },
            _ => None,
        }
    }

    /// A default chat member permissions, for groups and supergroups.
    #[must_use]
    pub fn permissions(&self) -> Option<ChatPermissions> {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Group(ChatFullInfoPublicGroup { permissions })
            | ChatFullInfoPublicKind::Supergroup(ChatFullInfoPublicSupergroup {
                permissions,
                ..
            }) = &this.kind
            {
                return permissions.clone();
            }
        }

        None
    }

    /// For supergroups, name of group sticker set.
    #[must_use]
    pub fn sticker_set_name(&self) -> Option<&str> {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.sticker_set_name.as_deref();
            }
        }

        None
    }

    /// `true`, if the bot can change the group sticker set.
    #[must_use]
    pub fn can_set_sticker_set(&self) -> bool {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.can_set_sticker_set;
            }
        }

        false
    }

    /// For supergroups, the name of the group's custom emoji sticker set.
    /// Custom emoji from this set can be used by all users and bots in the
    /// group.
    #[must_use]
    pub fn custom_emoji_sticker_set_name(&self) -> Option<&str> {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.custom_emoji_sticker_set_name.as_deref();
            }
        }

        None
    }

    /// The minimum allowed delay between consecutive messages sent by each
    /// unpriviledged user.
    #[must_use]
    pub fn slow_mode_delay(&self) -> Option<Seconds> {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.slow_mode_delay;
            }
        }

        None
    }

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa.
    #[must_use]
    pub fn unrestrict_boost_count(&self) -> Option<u16> {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.unrestrict_boost_count;
            }
        }

        None
    }

    /// The location to which the supergroup is connected.
    #[must_use]
    pub fn location(&self) -> Option<&ChatLocation> {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.location.as_ref();
            }
        }

        None
    }

    /// `true`, if users need to join the supergroup before they can send
    /// messages.
    #[must_use]
    pub fn join_to_send_messages(&self) -> bool {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.join_to_send_messages;
            }
        }

        false
    }

    /// `true`, if all users directly joining the supergroup need to be approved
    /// by supergroup administrators.
    #[must_use]
    pub fn join_by_request(&self) -> bool {
        if let ChatFullInfoKind::Public(this) = &self.kind {
            if let ChatFullInfoPublicKind::Supergroup(this) = &this.kind {
                return this.join_by_request;
            }
        }

        false
    }

    /// A description, for groups, supergroups and channel chats.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match &self.kind {
            ChatFullInfoKind::Public(this) => this.description.as_deref(),
            _ => None,
        }
    }

    /// A chat invite link, for groups, supergroups and channel chats. Each
    /// administrator in a chat generates their own invite links, so the
    /// bot must first generate the link using
    /// [`ExportChatInviteLink`].
    ///
    /// [`ExportChatInviteLink`]:
    /// crate::payloads::ExportChatInviteLink
    #[must_use]
    pub fn invite_link(&self) -> Option<&str> {
        match &self.kind {
            ChatFullInfoKind::Public(this) => this.invite_link.as_deref(),
            _ => None,
        }
    }

    /// `true`, if messages from the chat can't be forwarded to other chats.
    #[must_use]
    pub fn has_protected_content(&self) -> bool {
        match &self.kind {
            ChatFullInfoKind::Public(this) => this.has_protected_content,
            _ => false,
        }
    }

    /// List of available reactions allowed in the chat. If omitted, then all
    /// emoji reactions are allowed.
    #[must_use]
    pub fn available_reactions(&self) -> Option<&[ReactionType]> {
        match &self.kind {
            ChatFullInfoKind::Public(this) => this.available_reactions.as_deref(),
            _ => None,
        }
    }

    /// A first name of the other party in a private chat.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match &self.kind {
            ChatFullInfoKind::Private(this) => this.first_name.as_deref(),
            _ => None,
        }
    }

    /// A last name of the other party in a private chat.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match &self.kind {
            ChatFullInfoKind::Private(this) => this.last_name.as_deref(),
            _ => None,
        }
    }

    /// Bio of the other party in a private chat.
    #[must_use]
    pub fn bio(&self) -> Option<&str> {
        match &self.kind {
            ChatFullInfoKind::Private(this) => this.bio.as_deref(),
            _ => None,
        }
    }

    /// `true`, if privacy settings of the other party in the private chat
    /// allows to use tg://user?id=<user_id> links only in chats with the
    /// user.
    #[must_use]
    pub fn has_private_forwards(&self) -> bool {
        match &self.kind {
            ChatFullInfoKind::Private(this) => this.has_private_forwards,
            _ => false,
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
}

mod serde_helper {
    use crate::types::{Birthdate, BusinessIntro, BusinessLocation, BusinessOpeningHours, Chat};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum Type {
        Private,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Serialize, Deserialize)]
    pub(super) struct ChatPrivateFullInfo {
        /// A dummy field. Used to ensure that the `type` field is equal to
        /// `private`.
        r#type: Type,

        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        bio: Option<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        has_private_forwards: bool,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        has_restricted_voice_and_video_messages: bool,
        personal_chat: Option<Box<Chat>>,
        birthdate: Option<Birthdate>,
        business_intro: Option<BusinessIntro>,
        business_location: Option<BusinessLocation>,
        business_opening_hours: Option<BusinessOpeningHours>,
    }

    impl From<ChatPrivateFullInfo> for super::ChatFullInfoPrivate {
        fn from(
            ChatPrivateFullInfo {
                r#type: _,
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
                personal_chat,
                birthdate,
                business_intro,
                business_location,
                business_opening_hours,
            }: ChatPrivateFullInfo,
        ) -> Self {
            Self {
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
                personal_chat,
                birthdate,
                business_intro,
                business_location,
                business_opening_hours,
            }
        }
    }

    impl From<super::ChatFullInfoPrivate> for ChatPrivateFullInfo {
        fn from(
            super::ChatFullInfoPrivate {
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
                personal_chat,
                birthdate,
                business_intro,
                business_location,
                business_opening_hours,
            }: super::ChatFullInfoPrivate,
        ) -> Self {
            Self {
                r#type: Type::Private,
                username,
                first_name,
                last_name,
                bio,
                has_private_forwards,
                has_restricted_voice_and_video_messages,
                personal_chat,
                birthdate,
                business_intro,
                business_location,
                business_opening_hours,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn channel_de() {
        let expected = ChatFullInfo {
            id: ChatId(-1),
            kind: ChatFullInfoKind::Public(Box::new(ChatFullInfoPublic {
                title: None,
                kind: ChatFullInfoPublicKind::Channel(ChatFullInfoPublicChannel {
                    username: Some("channel_name".into()),
                    linked_chat_id: None,
                    can_send_paid_media: false,
                }),
                description: None,
                invite_link: None,
                has_protected_content: false,
                available_reactions: Some(vec![ReactionType::Emoji { emoji: "🌭".to_owned() }]),
            })),
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
            accepted_gift_types: AcceptedGiftTypes {
                unlimited_gifts: true,
                limited_gifts: true,
                unique_gifts: true,
                premium_subscription: true,
            },
            accent_color_id: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: DateTime::from_timestamp(1720708004, 0),
            has_visible_history: false,
            max_reaction_count: 0,
        };
        let actual = from_str(
            r#"{
                "id": -1,
                "type": "channel",
                "username": "channel_name",
                "available_reactions": [
                    {
                        "type": "emoji",
                        "emoji": "🌭"
                    }
                ],
                "emoji_status_expiration_date": 1720708004,
                "max_reaction_count": 0,
                "accepted_gift_types": {
                    "unlimited_gifts": true,
                    "limited_gifts": true,
                    "unique_gifts": true,
                    "premium_subscription": true
                }
            }"#,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn private_chat_de() {
        let chat = ChatFullInfo {
            id: ChatId(0),
            kind: ChatFullInfoKind::Private(Box::new(ChatFullInfoPrivate {
                username: Some("username".into()),
                first_name: Some("Anon".into()),
                last_name: None,
                bio: None,
                has_private_forwards: false,
                has_restricted_voice_and_video_messages: false,
                personal_chat: None,
                birthdate: None,
                business_intro: None,
                business_location: None,
                business_opening_hours: None,
            })),
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
            accepted_gift_types: AcceptedGiftTypes {
                unlimited_gifts: true,
                limited_gifts: true,
                unique_gifts: true,
                premium_subscription: true,
            },
            accent_color_id: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: DateTime::from_timestamp(1720708004, 0),
            has_visible_history: false,
            max_reaction_count: 0,
        };
        eprintln!("{}", to_string(&chat).unwrap());
        assert_eq!(
            chat,
            from_str(
                r#"{
                    "id": 0,
                    "type": "private",
                    "username": "username",
                    "first_name": "Anon",
                    "emoji_status_expiration_date": 1720708004,
                    "max_reaction_count": 0,
                    "accepted_gift_types": {
                        "unlimited_gifts": true,
                        "limited_gifts": true,
                        "unique_gifts": true,
                        "premium_subscription": true
                    }
                }"#
            )
            .unwrap()
        );
    }

    #[test]
    fn private_roundtrip() {
        let chat = ChatFullInfo {
            id: ChatId(0),
            kind: ChatFullInfoKind::Private(Box::new(ChatFullInfoPrivate {
                username: Some("username".into()),
                first_name: Some("Anon".into()),
                last_name: None,
                bio: None,
                has_private_forwards: false,
                has_restricted_voice_and_video_messages: false,
                personal_chat: None,
                birthdate: None,
                business_intro: None,
                business_location: None,
                business_opening_hours: None,
            })),
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
            accepted_gift_types: AcceptedGiftTypes {
                unlimited_gifts: true,
                limited_gifts: true,
                unique_gifts: true,
                premium_subscription: true,
            },
            accent_color_id: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            has_visible_history: false,
            max_reaction_count: 0,
        };

        let json = to_string(&chat).unwrap();
        let chat2 = from_str::<ChatFullInfo>(&json).unwrap();

        assert_eq!(chat, chat2);
    }

    #[test]
    fn private_chat_de_wrong_type_field() {
        assert!(from_str::<ChatFullInfo>(r#"{"id":0,"type":"WRONG"}"#).is_err());
    }
}
