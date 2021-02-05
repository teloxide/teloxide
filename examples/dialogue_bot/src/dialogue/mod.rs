mod states;

use crate::dialogue::states::{
    ReceiveAgeState, ReceiveFullNameState, ReceiveLocationState, StartState,
};
use derive_more::From;
use teloxide::dispatching::dialogue::Dialogue;
use teloxide_macros::Transition;
use std::convert::Infallible;

#[derive(Debug, Clone, Transition, From)]
pub enum MyDialogue {
    Start(StartState),
    ReceiveFullName(ReceiveFullNameState),
    ReceiveAge(ReceiveAgeState),
    ReceiveLocation(ReceiveLocationState),
}

type SubIn<T> = Dialogue<MyDialogue, Infallible, T>;

impl Default for MyDialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
