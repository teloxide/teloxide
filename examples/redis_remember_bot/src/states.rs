use teloxide_macros::Transition;

use serde::{Deserialize, Serialize};

#[derive(Transition, SmartDefault, From, Serialize, Deserialize)]
pub enum Dialogue {
    #[default]
    Start(StartState),
    HaveNumber(HaveNumberState),
}

#[derive(Default, Serialize, Deserialize)]
pub struct StartState;

#[derive(Serialize, Deserialize)]
pub struct HaveNumberState {
    pub number: i32,
}
