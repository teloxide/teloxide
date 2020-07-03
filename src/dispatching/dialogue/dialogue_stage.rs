use crate::dispatching::dialogue::TransitionOut;

/// Continue or terminate a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DialogueStage<D> {
    Next(D),
    Exit,
}

/// Returns a new dialogue state.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn next<Dialogue, State>(new_state: State) -> TransitionOut<Dialogue>
where
    Dialogue: From<State>,
{
    Ok(DialogueStage::Next(Dialogue::from(new_state)))
}

/// Exits a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn exit<D>() -> TransitionOut<D> {
    Ok(DialogueStage::Exit)
}
