/// Continue or terminate a dialogue.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DialogueStage<D> {
    Next(D),
    Exit,
}

/// A shortcut for `Ok(DialogueStage::Next(dialogue))`.
pub fn next<E, D>(dialogue: D) -> Result<DialogueStage<D>, E> {
    Ok(DialogueStage::Next(dialogue))
}

/// A shortcut for `Ok(DialogueStage::Exit)`.
pub fn exit<E, D>() -> Result<DialogueStage<D>, E> {
    Ok(DialogueStage::Exit)
}
