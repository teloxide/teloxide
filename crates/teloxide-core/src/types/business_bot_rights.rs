use serde::{Deserialize, Serialize};

/// Represents the rights of a business bot.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BusinessBotRights {
    /// `true`, if the bot can send and edit messages in the private chats that
    /// had incoming messages in the last 24 hours
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_reply: bool,

    /// `true`, if the bot can mark incoming private messages as read
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_read_messages: bool,

    /// `true`, if the bot can delete messages sent by the bot
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_delete_sent_messages: bool,

    /// `true`, if the bot can delete all private messages in managed chats
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_delete_all_messages: bool,

    /// `true`, if the bot can edit the first and last name of the business
    /// account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_edit_name: bool,

    /// `true`, if the bot can edit the bio of the business account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_edit_bio: bool,

    /// `true`, if the bot can edit the profile photo of the business account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_edit_profile_photo: bool,

    /// `true`, if the bot can edit the username of the business account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_edit_username: bool,

    /// `true`, if the bot can change the privacy settings pertaining to gifts
    /// for the business account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_change_gift_settings: bool,

    /// `true`, if the bot can view gifts and the amount of Telegram Stars owned
    /// by the business account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_view_gifts_and_stars: bool,

    /// `true`, if the bot can convert regular gifts owned by the business
    /// account to Telegram Stars
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_convert_gifts_to_stars: bool,

    /// `true`, if the bot can transfer and upgrade gifts owned by the business
    /// account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_transfer_and_upgrade_gifts: bool,

    /// `true`, if the bot can transfer Telegram Stars received by the business
    /// account to its own account, or use them to upgrade and transfer gifts
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_transfer_stars: bool,

    /// `true`, if the bot can post, edit and delete stories on behalf of the
    /// business account
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_manage_stories: bool,
}
