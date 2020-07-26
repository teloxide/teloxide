mod states;

use crate::dialogue::states::{
    ReceiveAgeState, ReceiveFullNameState, ReceiveLocationState, StartState,
};
use derive_more::From;
use teloxide_macros::Transition;

#[derive(Transition, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveFullName(ReceiveFullNameState),
    ReceiveAge(ReceiveAgeState),
    ReceiveLocation(ReceiveLocationState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
