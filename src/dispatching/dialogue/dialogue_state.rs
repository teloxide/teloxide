use crate::dispatching::dialogue::Dialogue;

/// Continue or terminate a dialogue.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DialogueStage<State, T> {
    Next(Dialogue<State, T>),
    Exit,
}
