use serde::{Deserialize, Serialize};

use crate::types::user::User;

/// This object represents one row of the high scores table for a game.
///
/// [The official docs](https://core.telegram.org/bots/api#gamehighscore).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GameHighScore {
    /// Position in high score table for the game.
    pub position: u32,

    /// User.
    pub user: User,

    /// Score.
    pub score: u32,
}
