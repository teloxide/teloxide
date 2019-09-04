use crate::core::types::{ChatPermissions, ChatPhoto, Message};

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Chat {
    #[serde(rename = "chat_id")]
    pub id: i32,
    #[serde(flatten)]
    pub type_: ChatType,
    pub photo: Option<ChatPhoto>,
}

struct PrivateChatTypeVisitor;

impl<'de> serde::de::Visitor<'de> for PrivateChatTypeVisitor {
    type Value = ();

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r#"field equal to "private""#)
    }

    fn visit_borrowed_str<E: serde::de::Error>(
        self,
        v: &'de str,
    ) -> Result<Self::Value, E> {
        match v {
            "private" => Ok(()),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Str(v),
                &r#""private""#,
            )),
        }
    }
}

fn assert_private_field<'de, D: serde::Deserializer<'de>>(
    des: D,
) -> Result<(), D::Error> {
    des.deserialize_str(PrivateChatTypeVisitor)
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ChatType {
    NotPrivate {
        title: Option<String>,
        #[serde(flatten)]
        type_: NotPrivateChatType,
        description: Option<String>,
        invite_link: Option<String>,
        pinned_message: Option<Box<Message>>,
    },
    Private {
        /// Dummy field. Used to ensure that "type" field is equal to "private"
        #[serde(rename = "type")]
        #[serde(deserialize_with = "assert_private_field")]
        type_: (),
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
    },
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum NotPrivateChatType {
    Channel {
        username: Option<String>,
    },
    Group {
        permissions: Option<ChatPermissions>,
    },
    Supergroup {
        username: Option<String>,
        sticker_set_name: Option<String>,
        can_set_sticker_set: Option<bool>,
        permissions: Option<ChatPermissions>,
    },
}

#[cfg(test)]
mod tests {
    use crate::core::types::*;
    use serde_json::from_str;

    #[test]
    fn channel_de() {
        assert_eq!(
            Chat {
                id: -1,
                type_: ChatType::NotPrivate {
                    title: None,
                    type_: NotPrivateChatType::Channel {
                        username: Some("channelname".into())
                    },
                    description: None,
                    invite_link: None,
                    pinned_message: None
                },
                photo: None,
            },
            from_str(
                r#"{"chat_id":-1,"type":"channel","username":"channelname"}"#
            )
            .unwrap()
        );
    }

    #[test]
    fn private_chat_de() {
        assert_eq!(
            Chat {
                id: 0,
                type_: ChatType::Private {
                    type_: (),
                    username: Some("username".into()),
                    first_name: Some("Anon".into()),
                    last_name: None
                },
                photo: None
            },
            from_str(
                r#"{"chat_id":0,"type":"private","username":"username","first_name":"Anon"}"#
            ).unwrap());
    }

    #[test]
    fn private_chat_de_wrong_type_field() {
        assert!(from_str::<Chat>(r#"{"chat_id":0,"type":"WRONG"}"#).is_err());
    }
}
