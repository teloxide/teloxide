mod states;

use crate::dialogue::states::{
    ReceiveAgeState, ReceiveFullNameState, ReceiveLocationState, StartState,
};
use teloxide_macros::Transition;

#[derive(Transition, SmartDefault, From)]
pub enum Dialogue {
    #[default]
    Start(StartState),
    ReceiveFullName(ReceiveFullNameState),
    ReceiveAge(ReceiveAgeState),
    ReceiveLocation(ReceiveLocationState),
}
