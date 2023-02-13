use serde::{Deserialize, Serialize};
use std::ops::Not;

bitflags::bitflags! {
    /// Describes actions that a non-administrator user is allowed to take in a
    /// chat.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#chatpermissions).
    ///
    /// ## Examples
    ///
    /// ```
    /// use teloxide_core::types::ChatPermissions;
    ///
    /// // No permissions, nothing is allowed
    /// let _ = ChatPermissions::empty();
    ///
    /// // All permissions, everything is allowed
    /// let _ = ChatPermissions::all();
    ///
    /// // One particular permission
    /// let permissions_v0 = ChatPermissions::INVITE_USERS;
    ///
    /// // Check what is permitted
    /// assert!(permissions_v0.contains(ChatPermissions::INVITE_USERS));
    /// assert!(!permissions_v0.contains(ChatPermissions::SEND_MESSAGES));
    ///
    /// // Union, add permissions
    /// let permissions_v1 = permissions_v0 | ChatPermissions::SEND_MEDIA_MESSAGES;
    /// assert!(permissions_v1.contains(ChatPermissions::INVITE_USERS));
    /// assert!(permissions_v1.contains(ChatPermissions::SEND_MEDIA_MESSAGES));
    ///
    /// // Implied by `SEND_MEDIA_MESSAGES`
    /// assert!(permissions_v1.contains(ChatPermissions::SEND_MESSAGES));
    ///
    /// // Difference, remove permissions
    /// let permissions_v2 = permissions_v1 - ChatPermissions::SEND_MEDIA_MESSAGES;
    /// assert!(!permissions_v2.contains(ChatPermissions::SEND_MEDIA_MESSAGES));
    ///
    /// // Removing `SEND_MEDIA_MESSAGES` also removes `SEND_MESSAGES` and vice versa
    /// // because `SEND_MESSAGES` is implied by `SEND_MEDIA_MESSAGES`
    /// assert!(!permissions_v2.contains(ChatPermissions::SEND_MESSAGES));
    ///
    /// let permissions_v3 = permissions_v1 - ChatPermissions::SEND_MESSAGES;
    /// assert!(!permissions_v3.contains(ChatPermissions::SEND_MEDIA_MESSAGES));
    /// ```
    #[derive(Serialize, Deserialize)]
    #[serde(from = "ChatPermissionsRaw", into = "ChatPermissionsRaw")]
    pub struct ChatPermissions: u16 {
        /// Set if the user is allowed to send text messages, contacts,
        /// locations and venues.
        const SEND_MESSAGES = 1;

        /// Set if the user is allowed to send audios, documents,
        /// photos, videos, video notes and voice notes, implies
        /// `SEND_MESSAGES`.
        const SEND_MEDIA_MESSAGES = (1 << 1) | Self::SEND_MESSAGES.bits;

        /// Set if the user is allowed to send polls, implies
        /// `SEND_MESSAGES`.
        const SEND_POLLS = (1 << 2) | Self::SEND_MESSAGES.bits;

        /// Set if the user is allowed to send animations, games, stickers and
        /// use inline bots, implies `SEND_MEDIA_MESSAGES`.
        const SEND_OTHER_MESSAGES = (1 << 3) | Self::SEND_MEDIA_MESSAGES.bits;

        /// Set if the user is allowed to add web page previews to
        /// their messages, implies `SEND_MEDIA_MESSAGES`.
        const ADD_WEB_PAGE_PREVIEWS = (1 << 4) | Self::SEND_MEDIA_MESSAGES.bits;

        /// Set if the user is allowed to change the chat title, photo and
        /// other settings. Ignored in public supergroups.
        const CHANGE_INFO = (1 << 5);

        /// Set if the user is allowed to invite new users to the chat.
        const INVITE_USERS = (1 << 6);

        /// Set if the user is allowed to pin messages. Ignored in public
        /// supergroups.
        const PIN_MESSAGES = (1 << 7);

        /// Set if the user is allowed to create, rename, close, and reopen forum topics.
        const MANAGE_TOPICS = (1 << 8);
    }
}

impl ChatPermissions {
    /// Checks for [`SEND_MESSAGES`] permission.
    ///
    /// [`SEND_MESSAGES`]: ChatPermissions::SEND_MESSAGES
    pub fn can_send_messages(&self) -> bool {
        self.contains(ChatPermissions::SEND_MESSAGES)
    }

    /// Checks for [`SEND_MEDIA_MESSAGES`] permission.
    ///
    /// [`SEND_MEDIA_MESSAGES`]: ChatPermissions::SEND_MEDIA_MESSAGES
    pub fn can_send_media_messages(&self) -> bool {
        self.contains(ChatPermissions::SEND_MEDIA_MESSAGES)
    }

    /// Checks for [`SEND_POLLS`] permission.
    ///
    /// [`SEND_POLLS`]: ChatPermissions::SEND_POLLS
    pub fn can_send_polls(&self) -> bool {
        self.contains(ChatPermissions::SEND_POLLS)
    }

    /// Checks for [`SEND_OTHER_MESSAGES`] permission.
    ///
    /// [`SEND_OTHER_MESSAGES`]: ChatPermissions::SEND_OTHER_MESSAGES
    pub fn can_send_other_messages(&self) -> bool {
        self.contains(ChatPermissions::SEND_OTHER_MESSAGES)
    }

    /// Checks for [`ADD_WEB_PAGE_PREVIEWS`] permission.
    ///
    /// [`ADD_WEB_PAGE_PREVIEWS`]: ChatPermissions::ADD_WEB_PAGE_PREVIEWS
    pub fn can_add_web_page_previews(&self) -> bool {
        self.contains(ChatPermissions::ADD_WEB_PAGE_PREVIEWS)
    }

    /// Checks for [`CHANGE_INFO`] permission.
    ///
    /// [`CHANGE_INFO`]: ChatPermissions::CHANGE_INFO
    pub fn can_change_info(&self) -> bool {
        self.contains(ChatPermissions::CHANGE_INFO)
    }

    /// Checks for [`INVITE_USERS`] permission.
    ///
    /// [`INVITE_USERS`]: ChatPermissions::INVITE_USERS
    pub fn can_invite_users(&self) -> bool {
        self.contains(ChatPermissions::INVITE_USERS)
    }

    /// Checks for [`PIN_MESSAGES`] permission.
    ///
    /// [`PIN_MESSAGES`]: ChatPermissions::PIN_MESSAGES
    pub fn can_pin_messages(&self) -> bool {
        self.contains(ChatPermissions::PIN_MESSAGES)
    }

    /// Checks for [`MANAGE_TOPICS`] permission.
    ///
    /// [`MANAGE_TOPICS`]: ChatPermissions::MANAGE_TOPICS
    pub fn can_manage_topics(&self) -> bool {
        self.contains(ChatPermissions::MANAGE_TOPICS)
    }
}

/// Helper for (de)serialization
#[derive(Serialize, Deserialize)]
struct ChatPermissionsRaw {
    #[serde(default, skip_serializing_if = "Not::not")]
    can_send_messages: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    can_send_media_messages: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    can_send_polls: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    can_send_other_messages: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    can_add_web_page_previews: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    can_change_info: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    can_invite_users: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    can_pin_messages: bool,

    // HACK: do not `skip_serializing_if = "Not::not"`, from tg docs:
    //       > If omitted defaults to the value of `can_pin_messages`
    //       but we don't have two different values for "absent" and "false"...
    //       or did they mean that `can_pin_messages` implies `can_manage_topics`?..
    #[serde(default)]
    can_manage_topics: bool,
}

impl From<ChatPermissions> for ChatPermissionsRaw {
    fn from(this: ChatPermissions) -> Self {
        Self {
            can_send_messages: this.can_send_messages(),
            can_send_media_messages: this.can_send_media_messages(),
            can_send_polls: this.can_send_polls(),
            can_send_other_messages: this.can_send_other_messages(),
            can_add_web_page_previews: this.can_add_web_page_previews(),
            can_change_info: this.can_change_info(),
            can_invite_users: this.can_invite_users(),
            can_pin_messages: this.can_pin_messages(),
            can_manage_topics: this.can_manage_topics(),
        }
    }
}

impl From<ChatPermissionsRaw> for ChatPermissions {
    fn from(
        ChatPermissionsRaw {
            can_send_messages,
            can_send_media_messages,
            can_send_polls,
            can_send_other_messages,
            can_add_web_page_previews,
            can_change_info,
            can_invite_users,
            can_pin_messages,
            can_manage_topics,
        }: ChatPermissionsRaw,
    ) -> Self {
        let mut this = Self::empty();

        if can_send_messages {
            this |= Self::SEND_MESSAGES;
        }
        if can_send_media_messages {
            this |= Self::SEND_MEDIA_MESSAGES
        }
        if can_send_polls {
            this |= Self::SEND_POLLS;
        }
        if can_send_other_messages {
            this |= Self::SEND_OTHER_MESSAGES;
        }
        if can_add_web_page_previews {
            this |= Self::ADD_WEB_PAGE_PREVIEWS;
        }
        if can_change_info {
            this |= Self::CHANGE_INFO;
        }
        if can_invite_users {
            this |= Self::INVITE_USERS;
        }
        if can_pin_messages {
            this |= Self::PIN_MESSAGES;
        }
        // FIXME: should we do `|| can_pin_messages` here? (the same tg doc weirdness)
        if can_manage_topics {
            this |= Self::MANAGE_TOPICS
        }

        this
    }
}

#[cfg(test)]
mod tests {
    use super::ChatPermissions;

    #[test]
    fn serialization() {
        let permissions = ChatPermissions::SEND_MEDIA_MESSAGES | ChatPermissions::PIN_MESSAGES;
        let expected = r#"{"can_send_messages":true,"can_send_media_messages":true,"can_pin_messages":true,"can_manage_topics":false}"#;
        let actual = serde_json::to_string(&permissions).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialization() {
        let json =
            r#"{"can_send_messages":true,"can_send_media_messages":true,"can_pin_messages":true}"#;
        let expected = ChatPermissions::SEND_MEDIA_MESSAGES | ChatPermissions::PIN_MESSAGES;
        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }
}
