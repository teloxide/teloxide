use serde::Deserialize;

use crate::core::types::user::User;

#[derive(Debug, Deserialize, Clone)]
/// This object represents one row of the high scores table for a game.
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: u32,
    /// User
    pub user: User,
    /// Score
    pub score: u32,
}