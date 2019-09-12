#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// This object represents a phone contact.
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    pub last_name: Option<String>,
    /// Optional. Contact's user identifier in Telegram
    pub user_id: Option<i64>,
    /// Optional. Additional data about the contact in the form of a
    /// [vCard](https://en.wikipedia.org/wiki/VCard)
    pub vcard: Option<String>,
}