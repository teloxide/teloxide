use crate::core::types::{
    Message, ChosenInlineResult, CallbackQuery,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Update {
    #[serde(rename = "update_id")]
    pub id: i32,
    #[serde(flatten)]
    pub kind: UpdateKind,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(()), // TODO
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
}

// TODO: tests for deserialization
