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

/// A dialogue wrapper to bypass orphan rules.
pub trait DialogueWrapper<D> {
    fn new(dialogue: D) -> Self;
}

/// Returns a new dialogue state.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn next<Dialogue, State, Index, DWrapper>(
    new_state: State,
) -> TransitionOut<DWrapper>
where
    Dialogue: CoprodInjector<State, Index>,
    DWrapper: DialogueWrapper<Dialogue>,
{
    Ok(DialogueStage::Next(DWrapper::new(Dialogue::inject(new_state))))
}

/// Exits a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn exit<DWrapper>() -> TransitionOut<DWrapper> {
    Ok(DialogueStage::Exit)
}
