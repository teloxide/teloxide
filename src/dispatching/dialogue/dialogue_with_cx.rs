use crate::dispatching::{dialogue::GetChatId, UpdateWithCx};
use std::fmt::Debug;
use teloxide_core::requests::Requester;

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
#[derive(Debug)]
#[deprecated(note = "Use dispatching2 instead")]
pub struct DialogueWithCx<R, Upd, D, E> {
    pub cx: UpdateWithCx<R, Upd>,
    pub dialogue: Result<D, E>,
}

impl<Upd, R, D, E> DialogueWithCx<R, Upd, D, E> {
    /// Creates a new instance with the provided fields.
    pub fn new(cx: UpdateWithCx<R, Upd>, dialogue: D) -> Self {
        Self { cx, dialogue: Ok(dialogue) }
    }
}

impl<Upd, R, D, E> GetChatId for DialogueWithCx<R, Upd, D, E>
where
    Upd: GetChatId,
    R: Requester,
{
    fn chat_id(&self) -> i64 {
        self.cx.update.chat_id()
    }
}
