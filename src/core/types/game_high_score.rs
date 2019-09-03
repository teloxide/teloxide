use serde::Deserialize;

use crate::core::types::user::User;

#[derive(Debug, Deserialize)]
pub struct GameHighScore {
    pub position: i32,
    pub user: User,
    pub score: i32,
}