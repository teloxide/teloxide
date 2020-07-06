use teloxide::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct StartState;

#[derive(Serialize, Deserialize)]
pub struct HaveNumberState {
    rest: StartState,
    pub number: i32,
}

up!(
    StartState + [number: i32] -> HaveNumberState,
);

#[derive(SmartDefault, From, Serialize, Deserialize)]
pub enum Dialogue {
    #[default]
    Start(StartState),
    HaveNumber(HaveNumberState),
}
