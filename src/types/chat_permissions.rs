/// Describes actions that a non-administrator user is allowed to take in a
/// chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatpermissions).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChatPermissions {
    /// `true`, if the user is allowed to send text messages, contacts,
    /// locations and venues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,

    /// `true`, if the user is allowed to send audios, documents,
    /// photos, videos, video notes and voice notes, implies
    /// `can_send_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,

    /// `true`, if the user is allowed to send polls, implies
    /// `can_send_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,

    /// `true`, if the user is allowed to send animations, games, stickers and
    /// use inline bots, implies `can_send_media_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,

    /// `true`, if the user is allowed to add web page previews to
    /// their messages, implies `can_send_media_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,

    /// `true`, if the user is allowed to change the chat title, photo and
    /// other settings. Ignored in public supergroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,

    /// `true`, if the user is allowed to invite new users to the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,

    /// `true`, if the user is allowed to pin messages. Ignored in public
    /// supergroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}
