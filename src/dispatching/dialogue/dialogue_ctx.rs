use crate::{
    dispatching::{
        core::{Context, ContextWith, GetCtx, ParseContext, ParserOut, RecombineFrom},
        dispatcher_context::DispatcherContext,
    },
    types::Update,
};
use lockfree::map::Map;
use serde::__private::Formatter;
use std::{fmt::Debug, sync::Arc};
use tokio::sync::mpsc;

/// An context of the [`DialogueDispatcher`].
pub struct DialogueContext<Upd, D, S> {
    pub dispatcher_ctx: DispatcherContext<Upd>,
    /// Storage of the dialogues.
    pub storage: Arc<S>,
    pub dialogue: Option<D>,
    pub senders: Arc<Map<i64, mpsc::UnboundedSender<Update>>>,
    pub chat_id: Option<i64>,
}

impl<Upd, D, S> DialogueContext<Upd, D, S> {
    pub fn new(
        dispatcher_ctx: DispatcherContext<Upd>,
        storage: Arc<S>,
        dialogue: Option<D>,
        senders: Arc<Map<i64, mpsc::UnboundedSender<Update>>>,
        chat_id: Option<i64>,
    ) -> Self {
        DialogueContext { dispatcher_ctx, storage, dialogue, senders, chat_id }
    }
}

impl<Upd: Debug, D: Debug, S> Debug for DialogueContext<Upd, D, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("DialogueContext")
            .field("dispatcher_ctx", &self.dispatcher_ctx)
            .field("dialogue", &self.dialogue)
            .finish()
    }
}

impl<Upd, D, S> Context for DialogueContext<Upd, D, S> {
    type Upd = Upd;

    fn get_upd(&self) -> &Self::Upd {
        &self.dispatcher_ctx.upd
    }
}

impl<Upd1, Upd2, D, S> ContextWith<Upd2> for DialogueContext<Upd1, D, S> {
    type Context = DialogueContext<Upd2, D, S>;
}

impl<Upd1, Upd2, D, S> ParseContext<Upd2> for DialogueContext<Upd1, D, S> {
    fn parse<Rest>(
        self,
        f: impl Fn(Upd1) -> Result<ParserOut<Upd2, Rest>, Upd1>,
    ) -> Result<(Self::Context, Rest), Self> {
        let DialogueContext { dispatcher_ctx, storage, dialogue, senders, chat_id } = self;
        match dispatcher_ctx.parse(f) {
            Ok((cx, rest)) => Ok((
                DialogueContext { dispatcher_ctx: cx, storage, dialogue, senders, chat_id },
                rest,
            )),
            Err(cx) => {
                Err(DialogueContext { dispatcher_ctx: cx, storage, dialogue, senders, chat_id })
            }
        }
    }

    fn recombine<Parser, Rest>(info: ParserOut<Self::Context, Rest>) -> Self
    where
        Upd1: RecombineFrom<Parser, Upd2, Rest>,
    {
        let ParserOut {
            data: DialogueContext { dispatcher_ctx, storage, dialogue, senders, chat_id },
            rest,
        } = info;
        DialogueContext {
            dispatcher_ctx: <DispatcherContext<Upd1> as ParseContext<Upd2>>::recombine(
                ParserOut::new(dispatcher_ctx, rest),
            ),
            storage,
            dialogue,
            senders,
            chat_id,
        }
    }
}

impl<Upd, D, S> GetCtx<DialogueContext<Upd, D, S>> for DialogueContext<Upd, D, S> {
    fn get(&self) -> &DialogueContext<Upd, D, S> {
        self
    }

    fn get_own(self) -> DialogueContext<Upd, D, S> {
        self
    }
}

impl<Upd, D, S> GetCtx<DispatcherContext<Upd>> for DialogueContext<Upd, D, S> {
    fn get(&self) -> &DispatcherContext<Upd> {
        &self.dispatcher_ctx
    }

    fn get_own(self) -> DispatcherContext<Upd> {
        self.dispatcher_ctx
    }
}
