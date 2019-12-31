use serde::{Deserialize, Serialize};

use crate::types::InlineKeyboardMarkup;

/// Represents a [game].
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultgame).
///
/// [game]: https://core.telegram.org/bots/api#games
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// Short name of the game.
    pub game_short_name: String,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
