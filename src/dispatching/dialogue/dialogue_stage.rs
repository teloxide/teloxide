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
pub fn next<StateInjector, State, Index, E, Wrapper>(
    new_state: State,
) -> Result<DialogueStage<Wrapper>, E>
where
    StateInjector: CoprodInjector<State, Index>,
    Wrapper: DialogueWrapper<StateInjector>,
{
    Ok(DialogueStage::Next(Wrapper::new(StateInjector::inject(new_state))))
}

/// Exits a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn exit<E, D>() -> Result<DialogueStage<D>, E> {
    Ok(DialogueStage::Exit)
}
