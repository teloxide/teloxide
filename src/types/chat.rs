use crate::types::{ChatPermissions, ChatPhoto, Message};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Chat {
    pub id: i64,
    #[serde(flatten)]
    pub kind: ChatKind,
    pub photo: Option<ChatPhoto>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ChatKind {
    NonPrivate {
        title: Option<String>,
        #[serde(flatten)]
        kind: NonPrivateChatKind,
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

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum NonPrivateChatKind {
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

struct PrivateChatKindVisitor;

impl<'de> serde::de::Visitor<'de> for PrivateChatKindVisitor {
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

fn assert_private_field<'de, D>(des: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    des.deserialize_str(PrivateChatKindVisitor)
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use serde_json::from_str;

    #[test]
    fn channel_de() {
        let expected = Chat {
            id: -1,
            kind: ChatKind::NonPrivate {
                title: None,
                kind: NonPrivateChatKind::Channel {
                    username: Some("channelname".into()),
                },
                description: None,
                invite_link: None,
                pinned_message: None,
            },
            photo: None,
        };
        let actual =
            from_str(r#"{"id":-1,"type":"channel","username":"channelname"}"#)
                .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn private_chat_de() {
        assert_eq!(
            Chat {
                id: 0,
                kind: ChatKind::Private {
                    type_: (),
                    username: Some("username".into()),
                    first_name: Some("Anon".into()),
                    last_name: None
                },
                photo: None
            },
            from_str(
                r#"{"id":0,"type":"private","username":"username","first_name":"Anon"}"#
            ).unwrap());
    }

    #[test]
    fn private_chat_de_wrong_type_field() {
        assert!(from_str::<Chat>(r#"{"id":0,"type":"WRONG"}"#).is_err());
    }
}
