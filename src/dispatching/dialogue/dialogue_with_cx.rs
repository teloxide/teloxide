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
    /// Returns the inner `UpdateWithCx<Upd>` and an unwrapped dialogue.
    pub fn unpack(self) -> (UpdateWithCx<Upd>, D)
    where
        E: Debug,
    {
        (self.cx, self.dialogue.unwrap())
    }
}

impl<Upd, D, E> DialogueWithCx<Upd, D, E> {
    /// Creates a new instance with the provided fields.
    pub fn new(cx: UpdateWithCx<Upd>, dialogue: D) -> Self {
        Self { cx, dialogue: Ok(dialogue) }
    }

    /// Creates a new instance by substituting a dialogue and preserving
    /// `self.bot` and `self.update`.
    pub fn with_new_dialogue<Nd, Ne>(
        self,
        new_dialogue: Result<Nd, Ne>,
    ) -> DialogueWithCx<Upd, Nd, Ne> {
        DialogueWithCx { cx: self.cx, dialogue: new_dialogue }
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
