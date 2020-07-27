use serde::{Deserialize, Serialize};

use crate::types::user::User;

/// This object represents one row of the high scores table for a game.
///
/// [The official docs](https://core.telegram.org/bots/api#gamehighscore).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GameHighScore {
    /// Position in high score table for the game.
    pub position: u32,

    /// User.
    pub user: User,

    /// Score.
    pub score: u32,
}

impl GameHighScore {
    pub fn new(position: u32, user: User, score: u32) -> Self {
        Self { position, user, score }
    }

    pub fn position(mut self, val: u32) -> Self {
        self.position = val;
        self
    }

    pub fn user(mut self, val: User) -> Self {
        self.user = val;
        self
    }

    pub fn score(mut self, val: u32) -> Self {
        self.score = val;
        self
    }
}
