use teloxide::prelude::*;
use teloxide_macros::BotDialogue;

use super::transitions::{have_number, start};

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

#[derive(BotDialogue, SmartDefault, From, Serialize, Deserialize)]
pub enum Dialogue {
    #[default]
    #[transition(start)]
    Start(StartState),

    #[transition(have_number)]
    HaveNumber(HaveNumberState),
}
