use crate::dispatching::{dialogue::GetChatId, UpdateWithCx};
use std::fmt::Debug;

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
#[derive(Debug)]
pub struct DialogueWithCx<Upd, D, E> {
    pub cx: UpdateWithCx<Upd>,
    pub dialogue: Result<D, E>,
}

impl<Upd, D, E> DialogueWithCx<Upd, D, E> {
    /// Creates a new instance with the provided fields.
    pub fn new(cx: UpdateWithCx<Upd>, dialogue: D) -> Self {
        Self { cx, dialogue: Ok(dialogue) }
    }
}

impl<Upd, D, E> GetChatId for DialogueWithCx<Upd, D, E>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.cx.update.chat_id()
    }
}
