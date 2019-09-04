use crate::core::types::{ChatPermissions, ChatPhoto, Message, Integer};


#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Chat {
    pub id: Integer,
    #[serde(flatten)]
    pub type_: ChatType,
    pub photo: Option<ChatPhoto>,
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
        pinned_message: Option<Box<Message>>
    },
    Private {
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>
    }
}


#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum NotPrivateChatType {
    Channel {
        username: Option<String>
    },
    Group {
        permissions: Option<ChatPermissions>
    },
    Supergroup {
        username: Option<String>,
        sticker_set_name: Option<String>,
        can_set_sticker_set: Option<bool>,
        permissions: Option<ChatPermissions>
    }
}


#[test]
fn test_chat_de() {
    use serde_json::from_str;

    assert_eq!(
        Chat {
            id: 0,
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
        from_str(r#"{"id":0,"type":"channel","username":"channelname"}"#).unwrap()
    );
}
