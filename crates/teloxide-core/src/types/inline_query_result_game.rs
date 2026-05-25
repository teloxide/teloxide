use serde::{Deserialize, Serialize};

use crate::types::InlineKeyboardMarkup;

/// Represents a [game].
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultgame).
///
/// [game]: https://core.telegram.org/bots/api#games
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
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

impl InlineQueryResultGame {
    pub fn new<S1, S2>(id: S1, game_short_name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self { id: id.into(), game_short_name: game_short_name.into(), reply_markup: None }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn game_short_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.game_short_name = val.into();
        self
    }

    #[must_use]
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
