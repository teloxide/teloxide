use crate::dispatching::dialogue::Dialogue;

/// Continue or terminate a dialogue.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DialogueStage<State, T> {
    Next(Dialogue<State, T>),
    Exit,
}

/// A shortcut for `Ok(DialogueStage::Next(dialogue))`.
pub fn next<E, State, T>(
    dialogue: Dialogue<State, T>,
) -> Result<DialogueStage<State, T>, E> {
    Ok(DialogueStage::Next(dialogue))
}

/// A shortcut for `Ok(DialogueStage::Exit)`.
pub fn exit<E, State, T>() -> Result<DialogueStage<State, T>, E> {
    Ok(DialogueStage::Exit)
}
