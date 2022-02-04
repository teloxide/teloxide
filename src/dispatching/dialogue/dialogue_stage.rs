use crate::dispatching::dialogue::TransitionOut;

/// Continue or terminate a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
#[deprecated(note = "Use dispatching2 instead")]
pub enum DialogueStage<D> {
    Next(D),
    Exit,
}

/// Returns a new dialogue state.
///
/// Note the `Dialogue: From<State>` constraint. It means that you don't need to
/// pass `Dialogue` -- you can just pass one of it's states. [`From`] can be
/// conveniently derived by [derive-more].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`From`]: std::convert::From
/// [derive-more]: https://crates.io/crates/derive_more
#[deprecated(note = "Use dispatching2 instead")]
pub fn next<Dialogue, State, E>(new_state: State) -> TransitionOut<Dialogue, E>
where
    Dialogue: From<State>,
{
    Ok(DialogueStage::Next(Dialogue::from(new_state)))
}

/// Exits a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
#[deprecated(note = "Use dispatching2 instead")]
pub fn exit<D, E>() -> TransitionOut<D, E> {
    Ok(DialogueStage::Exit)
}
