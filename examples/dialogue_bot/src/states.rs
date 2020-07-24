use teloxide::prelude::*;

use super::transitions::{
    receive_10x5_answer, receive_days_of_week,
    receive_gandalf_alternative_name, start,
};

#[derive(BotDialogue, SmartDefault, From)]
pub enum Dialogue {
    #[default]
    #[handler(start)]
    Start(StartState),

    #[handler(receive_days_of_week)]
    ReceiveDaysOfWeek(ReceiveDaysOfWeekState),

    #[handler(receive_10x5_answer)]
    Receive10x5Answer(Receive10x5AnswerState),

    #[handler(receive_gandalf_alternative_name)]
    ReceiveGandalfAlternativeName(ReceiveGandalfAlternativeNameState),
}

#[derive(Default)]
pub struct StartState;

pub struct ReceiveDaysOfWeekState {
    rest: StartState,
}

pub struct Receive10x5AnswerState {
    rest: ReceiveDaysOfWeekState,
    days_of_week: u8,
}

pub struct ReceiveGandalfAlternativeNameState {
    rest: Receive10x5AnswerState,
    _10x5_answer: u8,
}

pub struct ExitState {
    rest: ReceiveGandalfAlternativeNameState,
    gandalf_alternative_name: String,
}

up!(
    StartState -> ReceiveDaysOfWeekState,
    ReceiveDaysOfWeekState + [days_of_week: u8] -> Receive10x5AnswerState,
    Receive10x5AnswerState + [_10x5_answer: u8] -> ReceiveGandalfAlternativeNameState,
    ReceiveGandalfAlternativeNameState + [gandalf_alternative_name: String] -> ExitState,
);
