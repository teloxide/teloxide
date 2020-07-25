use teloxide::prelude::*;
use teloxide_macros::Transition;

#[derive(Transition, SmartDefault, From)]
pub enum Dialogue {
    #[default]
    Start(StartState),
    ReceiveFullName(ReceiveFullNameState),
    ReceiveAge(ReceiveAgeState),
    ReceiveLocation(ReceiveLocationState),
}

#[derive(Generic, Default)]
pub struct StartState;

#[derive(Generic)]
pub struct ReceiveFullNameState;

#[derive(Generic)]
pub struct ReceiveAgeState {
    pub full_name: String,
}

pub struct ReceiveLocationState {
    pub full_name: String,
    pub age: u8,
}
