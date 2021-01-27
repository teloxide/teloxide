use serde::{Deserialize, Serialize};

/// Describes actions that a non-administrator user is allowed to take in a
/// chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatpermissions).
#[serde_with_macros::skip_serializing_none]
#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {
    /// `true`, if the user is allowed to send text messages, contacts,
    /// locations and venues.
    pub can_send_messages: Option<bool>,

    /// `true`, if the user is allowed to send audios, documents,
    /// photos, videos, video notes and voice notes, implies
    /// `can_send_messages`.
    pub can_send_media_messages: Option<bool>,

    /// `true`, if the user is allowed to send polls, implies
    /// `can_send_messages`.
    pub can_send_polls: Option<bool>,

    /// `true`, if the user is allowed to send animations, games, stickers and
    /// use inline bots, implies `can_send_media_messages`.
    pub can_send_other_messages: Option<bool>,

    /// `true`, if the user is allowed to add web page previews to
    /// their messages, implies `can_send_media_messages`.
    pub can_add_web_page_previews: Option<bool>,

    /// `true`, if the user is allowed to change the chat title, photo and
    /// other settings. Ignored in public supergroups.
    pub can_change_info: Option<bool>,

    /// `true`, if the user is allowed to invite new users to the chat.
    pub can_invite_users: Option<bool>,

    /// `true`, if the user is allowed to pin messages. Ignored in public
    /// supergroups.
    pub can_pin_messages: Option<bool>,
}

impl ChatPermissions {
    pub const fn new() -> Self {
        Self {
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
    }

    pub const fn can_send_messages(mut self, val: bool) -> Self {
        self.can_send_messages = Some(val);
        self
    }

    pub const fn can_send_media_messages(mut self, val: bool) -> Self {
        self.can_send_media_messages = Some(val);
        self
    }

    pub const fn can_send_polls(mut self, val: bool) -> Self {
        self.can_send_polls = Some(val);
        self
    }

    pub const fn can_send_other_messages(mut self, val: bool) -> Self {
        self.can_send_other_messages = Some(val);
        self
    }

    pub const fn can_add_web_page_previews(mut self, val: bool) -> Self {
        self.can_add_web_page_previews = Some(val);
        self
    }

    pub const fn can_change_info(mut self, val: bool) -> Self {
        self.can_change_info = Some(val);
        self
    }

    pub const fn can_invite_users(mut self, val: bool) -> Self {
        self.can_invite_users = Some(val);
        self
    }

    pub const fn can_pin_messages(mut self, val: bool) -> Self {
        self.can_pin_messages = Some(val);
        self
    }
}
