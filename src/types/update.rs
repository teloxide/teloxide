#![allow(clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

use crate::types::{CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll, PreCheckoutQuery, ShippingQuery, User, Sender, Chat};

/// This [object] represents an incoming update.
///
/// [The official docs](https://core.telegram.org/bots/api#update).
///
/// [object]: https://core.telegram.org/bots/api#available-types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially. This ID becomes especially
    /// handy if you’re using [Webhooks], since it allows you to ignore
    /// repeated updates or to restore the correct update sequence, should
    /// they get out of order. If there are no new updates for at least a
    /// week, then identifier of the next update will be chosen randomly
    /// instead of sequentially.
    ///
    /// [Webhooks]: crate::Bot::set_webhook
    #[serde(rename = "update_id")]
    pub id: i32,

    #[serde(flatten)]
    pub kind: UpdateKind,
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
}

impl Update {
    pub fn user(&self) -> Option<&User> {
        match &self.kind {
            UpdateKind::Message(m) => {
                match m.from() {
                    Some(Sender::User(user)) => Some(user),
                    _ => None,
                }
            }
            UpdateKind::EditedMessage(m) => {
                match m.from() {
                    Some(Sender::User(user)) => Some(user),
                    _ => None,
                }
            }
            UpdateKind::CallbackQuery(query) => {
                Some(&query.from)
            }
            UpdateKind::ChosenInlineResult(chosen) => {
                Some(&chosen.from)
            }
            UpdateKind::InlineQuery(query) => {
                Some(&query.from)
            }
            UpdateKind::ShippingQuery(query) => {
                Some(&query.from)
            }
            UpdateKind::PreCheckoutQuery(query) => {
                Some(&query.from)
            }
            _ => None
        }
    }

    pub fn chat(&self) -> Option<&Chat> {
        match &self.kind {
            UpdateKind::Message(m) => {
                Some(&m.chat)
            }
            UpdateKind::EditedMessage(m) => {
                Some(&m.chat)
            }
            UpdateKind::ChannelPost(p) => {
                Some(&p.chat)
            }
            UpdateKind::EditedChannelPost(p) => {
                Some(&p.chat)
            }
            UpdateKind::CallbackQuery(q) => {
                Some(&q.message.as_ref()?.chat)
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types::{
        Chat, ChatKind, ForwardKind, LanguageCode, MediaKind, Message,
        MessageKind, Sender, Update, UpdateKind, User,
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
                id: 6557,
                date: 1_569_518_342,
                chat: Chat {
                    id: 218_485_655,
                    kind: ChatKind::Private {
                        type_: (),
                        username: Some(String::from("WaffleLapkin")),
                        first_name: Some(String::from("Waffle")),
                        last_name: None,
                    },
                    photo: None,
                },
                kind: MessageKind::Common {
                    from: Sender::User(User {
                        id: 218_485_655,
                        is_bot: false,
                        first_name: String::from("Waffle"),
                        last_name: None,
                        username: Some(String::from("WaffleLapkin")),
                        language_code: Some(LanguageCode::EN),
                    }),
                    forward_kind: ForwardKind::Origin {
                        reply_to_message: None,
                    },
                    edit_date: None,
                    media_kind: MediaKind::Text {
                        text: String::from("hello there"),
                        entities: vec![],
                    },
                    reply_markup: None,
                },
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }
}
