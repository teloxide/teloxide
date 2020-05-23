use crate::dispatching::{dialogue::GetChatId, DispatcherHandlerCx};

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
#[derive(Debug)]
pub struct DialogueDispatcherHandlerCx<Upd, D, E> {
    pub cx: DispatcherHandlerCx<Upd>,
    pub dialogue: Result<D, E>,
}

impl<Upd, D, E> DialogueDispatcherHandlerCx<Upd, D, E> {
    /// Creates a new instance with the provided fields.
    pub fn new(cx: DispatcherHandlerCx<Upd>, dialogue: D) -> Self {
        Self { cx, dialogue: Ok(dialogue) }
    }

    /// Creates a new instance by substituting a dialogue and preserving
    /// `self.bot` and `self.update`.
    pub fn with_new_dialogue<Nd, Ne>(
        self,
        new_dialogue: Result<Nd, Ne>,
    ) -> DialogueDispatcherHandlerCx<Upd, Nd, Ne> {
        DialogueDispatcherHandlerCx { cx: self.cx, dialogue: new_dialogue }
    }
}

impl<Upd, D, E> GetChatId for DialogueDispatcherHandlerCx<Upd, D, E>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.cx.update.chat_id()
    }
}
