use serde::Deserialize;

use crate::core::types::user::User;

#[derive(Debug, Deserialize)]
struct GameHighScore {
    position: i32,
    user: User,
    score: i32,
}