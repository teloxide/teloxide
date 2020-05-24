use teloxide::prelude::*;

pub struct StartState;

pub struct ReceiveAttemptState {
    pub rest: StartState,
    pub secret: u8,
}

up!(
    StartState + [secret: u8] -> ReceiveAttemptState,
);

pub type Dialogue = Coprod!(StartState, ReceiveAttemptState);

wrap_dialogue!(
    Wrapper(Dialogue),
    default Self(Dialogue::inject(StartState)),
);
