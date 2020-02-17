/// Continue or terminate a dialogue.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DialogueStage<D> {
    Next(D),
    Exit,
}

/// A shortcut for `Ok(DialogueStage::Next(dialogue))`.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn next<E, D>(dialogue: D) -> Result<DialogueStage<D>, E> {
    Ok(DialogueStage::Next(dialogue))
}

/// A shortcut for `Ok(DialogueStage::Exit)`.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
pub fn exit<E, D>() -> Result<DialogueStage<D>, E> {
    Ok(DialogueStage::Exit)
}
