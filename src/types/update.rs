use crate::types::{CallbackQuery, ChosenInlineResult, Message};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Update<'a> {
    #[serde(rename = "update_id")]
    pub id: i32,
    #[serde(flatten)]
    pub kind: UpdateKind<'a>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind<'a> {
    Message(Message<'a>),
    EditedMessage(Message<'a>),
    ChannelPost(Message<'a>),
    EditedChannelPost(Message<'a>),
    InlineQuery(()),
    // TODO
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery<'a>),
}

// TODO: tests for deserialization
