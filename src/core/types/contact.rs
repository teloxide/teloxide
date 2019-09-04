use serde::Deserialization;

#[derive(Debug, Deserialization)]
/// This object represents a phone contact.
struct Contact {
    /// Contact's phone number
    phone_number: String,
    /// Contact's first name
    first_name: String,
    /// Optional. Contact's last name
    last_name: Option<String>,
    /// Optional. Contact's user identifier in Telegram
    user_id: Option<i64>,
    /// Optional. Additional data about the contact in the form of a
    /// [vCard](https://en.wikipedia.org/wiki/VCard)
    vcard: Option<String>,
}