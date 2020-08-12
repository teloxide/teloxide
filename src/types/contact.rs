use serde::{Deserialize, Serialize};

/// This object represents a phone contact.
///
/// [The official docs](https://core.telegram.org/bots/api#contact).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl Contact {
    pub fn new<S1, S2>(phone_number: S1, first_name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            user_id: None,
            vcard: None,
        }
    }

    pub fn phone_number<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.phone_number = val.into();
        self
    }

    pub fn first_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.first_name = val.into();
        self
    }

    pub fn last_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.last_name = Some(val.into());
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = Some(val);
        self
    }

    pub fn vcard<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.vcard = Some(val.into());
        self
    }
}
