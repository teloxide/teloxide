use teloxide::macros::Transition;

use serde::{Deserialize, Serialize};

#[derive(Transition, From, Serialize, Deserialize)]
pub enum Dialogue {
    Start(StartState),
    HaveNumber(HaveNumberState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StartState;

#[derive(Serialize, Deserialize)]
pub struct HaveNumberState {
    pub number: i32,
}
