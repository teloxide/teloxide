use crate::dispatching::dialogue::TransitionOut;
use frunk::coproduct::CoprodInjector;

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
pub fn next<Dialogue, State, Index>(new_state: State) -> TransitionOut<Dialogue>
where
    Dialogue: CoprodInjector<State, Index>,
{
    Ok(DialogueStage::Next(Dialogue::inject(new_state)))
}

/// Exits a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn exit<D>() -> TransitionOut<D> {
    Ok(DialogueStage::Exit)
}
