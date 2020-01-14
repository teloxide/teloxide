use serde::{Deserialize, Serialize};

use crate::types::{CallbackQuery, Message};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatUpdate {
    pub id: i32,

    pub kind: ChatUpdateKind,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatUpdateKind {
    Message(Message),
    EditedMessage(Message),
    CallbackQuery(CallbackQuery),
}
