use teloxide::prelude::*;
use teloxide_macros::Transition;

#[derive(Transition, SmartDefault, From)]
pub enum Dialogue {
    #[default]
    Start(StartState),
    ReceiveDaysOfWeek(ReceiveDaysOfWeekState),
    Receive10x5Answer(Receive10x5AnswerState),
    ReceiveGandalfAlternativeName(ReceiveGandalfAlternativeNameState),
}

#[derive(Generic, Default)]
pub struct StartState;

#[derive(Generic)]
pub struct ReceiveDaysOfWeekState;

#[derive(Generic)]
pub struct Receive10x5AnswerState {
    days_of_week: u8,
}

pub struct ReceiveGandalfAlternativeNameState {
    days_of_week: u8,
    _10x5_answer: u8,
}
