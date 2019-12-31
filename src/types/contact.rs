use serde::{Deserialize, Serialize};

/// This object represents a phone contact.
///
/// [The official docs](https://core.telegram.org/bots/api#contact).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Contact {
    /// A contact's phone number.
    pub phone_number: String,

    /// A contact's first name.
    pub first_name: String,

    /// A contact's last name.
    pub last_name: Option<String>,

    /// A contact's user identifier in Telegram.
    pub user_id: Option<i32>,

    /// Additional data about the contact in the form of a [vCard].
    ///
    /// [vCard]: https://en.wikipedia.org/wiki/VCard
    pub vcard: Option<String>,
}
