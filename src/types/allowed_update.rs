use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
}
