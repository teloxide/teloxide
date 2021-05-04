#![allow(clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

use crate::types::{
    CallbackQuery, Chat, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll,
    PollAnswer, PreCheckoutQuery, ShippingQuery, User,
};
use serde_json::Value;

/// This [object] represents an incoming update.
///
/// [The official docs](https://core.telegram.org/bots/api#update).
///
/// [object]: https://core.telegram.org/bots/api#available-types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially. This ID becomes especially
    /// handy if you’re using webhooks, since it allows you to ignore
    /// repeated updates or to restore the correct update sequence, should
    /// they get out of order. If there are no new updates for at least a
    /// week, then identifier of the next update will be chosen randomly
    /// instead of sequentially.
    #[serde(rename = "update_id")]
    pub id: i32,

    #[serde(flatten)]
    pub kind: UpdateKind,
}

impl Update {
    /// Tries to parse `value` into `Update`, logging an error on failure.
    ///
    /// It is used to implement update listeners.
    pub fn try_parse(value: &Value) -> Result<Self, serde_json::Error> {
        match serde_json::from_value(value.clone()) {
            Ok(update) => Ok(update),
            Err(error) => {
                log::error!(
                    "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                    This is a bug in teloxide-core, please open an issue here: \
                    https://github.com/teloxide/teloxide-core/issues.",
                    error,
                    value
                );
                Err(error)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),

    /// New version of a message that is known to the bot and was edited.
    EditedMessage(Message),

    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),

    /// New version of a channel post that is known to the bot and was edited.
    EditedChannelPost(Message),

    /// New incoming [inline] query.
    ///
    /// [inline]: https://core.telegram.org/bots/api#inline-mode
    InlineQuery(InlineQuery),

    /// The result of an [inline] query that was chosen by a user and sent to
    /// their chat partner. Please see our documentation on the [feedback
    /// collecting] for details on how to enable these updates for your bot.
    ///
    /// [inline]: https://core.telegram.org/bots/api#inline-mode
    /// [feedback collecting]: https://core.telegram.org/bots/inline#collecting-feedback
    ChosenInlineResult(ChosenInlineResult),

    /// New incoming callback query.
    CallbackQuery(CallbackQuery),

    /// New incoming shipping query. Only for invoices with flexible price.
    ShippingQuery(ShippingQuery),

    /// New incoming pre-checkout query. Contains full information about
    /// checkout.
    PreCheckoutQuery(PreCheckoutQuery),

    /// New poll state. Bots receive only updates about stopped polls and
    /// polls, which are sent by the bot.
    Poll(Poll),

    /// A user changed their answer in a non-anonymous poll. Bots receive new
    /// votes only in polls that were sent by the bot itself.
    PollAnswer(PollAnswer),

    /// The bot's chat member status was updated in a chat. For private chats,
    /// this update is received only when the bot is blocked or unblocked by the
    /// user.
    MyChatMember(ChatMemberUpdated),

    /// A chat member's status was updated in a chat. The bot must be an
    /// administrator in the chat and must explicitly specify
    /// [`AllowedUpdate::ChatMember`] in the list of `allowed_updates` to
    /// receive these updates.
    ///
    /// [`AllowedUpdate::ChatMember`]: crate::types::AllowedUpdate::ChatMember
    ChatMember(ChatMemberUpdated),
}

impl Update {
    pub fn user(&self) -> Option<&User> {
        match &self.kind {
            UpdateKind::Message(m) => m.from(),
            UpdateKind::EditedMessage(m) => m.from(),
            UpdateKind::CallbackQuery(query) => Some(&query.from),
            UpdateKind::ChosenInlineResult(chosen) => Some(&chosen.from),
            UpdateKind::InlineQuery(query) => Some(&query.from),
            UpdateKind::ShippingQuery(query) => Some(&query.from),
            UpdateKind::PreCheckoutQuery(query) => Some(&query.from),
            UpdateKind::PollAnswer(answer) => Some(&answer.user),
            _ => None,
        }
    }

    pub fn chat(&self) -> Option<&Chat> {
        match &self.kind {
            UpdateKind::Message(m) => Some(&m.chat),
            UpdateKind::EditedMessage(m) => Some(&m.chat),
            UpdateKind::ChannelPost(p) => Some(&p.chat),
            UpdateKind::EditedChannelPost(p) => Some(&p.chat),
            UpdateKind::CallbackQuery(q) => Some(&q.message.as_ref()?.chat),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types::{
        Chat, ChatKind, ChatPrivate, ForwardKind, ForwardOrigin, MediaKind, MediaText, Message,
        MessageCommon, MessageKind, Update, UpdateKind, User,
    };

    // TODO: more tests for deserialization
    #[test]
    fn message() {
        let json = r#"{
            "update_id":892252934,
            "message":{
                "message_id":6557,
                "from":{
                    "id":218485655,
                    "is_bot": false,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "language_code":"en"
                },
                "chat":{
                    "id":218485655,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "type":"private"
                },
               "date":1569518342,
               "text":"hello there"
            }
        }"#;

        let expected = Update {
            id: 892_252_934,
            kind: UpdateKind::Message(Message {
                via_bot: None,
                id: 6557,
                date: 1_569_518_342,
                chat: Chat {
                    id: 218_485_655,
                    kind: ChatKind::Private(ChatPrivate {
                        type_: (),
                        username: Some(String::from("WaffleLapkin")),
                        first_name: Some(String::from("Waffle")),
                        last_name: None,
                        bio: None,
                    }),
                    photo: None,
                    pinned_message: None,
                    message_auto_delete_time: None,
                },
                kind: MessageKind::Common(MessageCommon {
                    from: Some(User {
                        id: 218_485_655,
                        is_bot: false,
                        first_name: String::from("Waffle"),
                        last_name: None,
                        username: Some(String::from("WaffleLapkin")),
                        language_code: Some(String::from("en")),
                    }),
                    forward_kind: ForwardKind::Origin(ForwardOrigin {
                        reply_to_message: None,
                    }),
                    edit_date: None,
                    media_kind: MediaKind::Text(MediaText {
                        text: String::from("hello there"),
                        entities: vec![],
                    }),
                    reply_markup: None,
                    sender_chat: None,
                    author_signature: None,
                }),
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn de_private_chat_text_message() {
        let text = r#"
  {
    "message": {
      "chat": {
        "first_name": "Hirrolot",
        "id": 408258968,
        "type": "private",
        "username": "hirrolot"
      },
      "date": 1581448857,
      "from": {
        "first_name": "Hirrolot",
        "id": 408258968,
        "is_bot": false,
        "language_code": "en",
        "username": "hirrolot"
      },
      "message_id": 154,
      "text": "4"
    },
    "update_id": 306197398
  }
"#;

        assert!(serde_json::from_str::<Update>(text).is_ok());
    }

    #[test]
    fn pinned_message_works() {
        let json = r#"{
    "message": {
        "chat": {
            "id": -1001276785818,
            "title": "teloxide dev",
            "type": "supergroup",
            "username": "teloxide_dev"
        },
        "date": 1582134655,
        "from": {
            "first_name": "Hirrolot",
            "id": 408258968,
            "is_bot": false,
            "username": "hirrolot"
        },
        "message_id": 20225,
        "pinned_message": {
            "chat": {
                "id": -1001276785818,
                "title": "teloxide dev",
                "type": "supergroup",
                "username": "teloxide_dev"
            },
            "date": 1582134643,
            "from": {
                "first_name": "Hirrolot",
                "id": 408258968,
                "is_bot": false,
                "username": "hirrolot"
            },
            "message_id": 20224,
            "text": "Faster than a bullet"
        }
    },
    "update_id": 845402291
}"#;

        serde_json::from_str::<Update>(json).unwrap();
    }

    #[test]
    fn dice_works() {
        let json = r#"
        {
    "message": {
        "chat": {
            "id": -1001276785818,
            "title": "bla bla bla chat",
            "type": "supergroup",
            "username": "teloxide_dev"
        },
        "date": 1596014550,
        "dice": {
            "emoji": "🎲",
            "value": 2
        },
        "from": {
            "first_name": "Hirrolot",
            "id": 408258968,
            "is_bot": false,
            "language_code": "en",
            "username": "hirrolot"
        },
        "message_id": 35410
    },
    "update_id": 573255266
}
        "#;

        serde_json::from_str::<Update>(json).unwrap();
    }
}
