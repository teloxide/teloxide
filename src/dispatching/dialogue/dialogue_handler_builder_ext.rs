use crate::dispatching::{
    core::{Context, GetCtx, Guard, HandlerBuilderWithGuards, IntoGuard},
    dialogue::dialogue_ctx::DialogueContext,
};
use futures::future::BoxFuture;
use pin_project::__private::PhantomData;

pub trait DialogueHandlerBuilderExt<Ctx, D, S, Err> {
    fn with_dialogue<G: Guard<D> + Send + Sync + 'static>(
        self,
        guard: impl IntoGuard<D, G> + 'static,
    ) -> Self;

    fn has_dialogue(self) -> Self;
}

impl<Upd, Ctx, D, S, Err, T> DialogueHandlerBuilderExt<Ctx, D, S, Err> for T
where
    Upd: Send + Sync + 'static,
    D: Send + Sync + 'static,
    S: Send + Sync + 'static,
    T: HandlerBuilderWithGuards<Ctx, Err>,
    Ctx: Context<Upd = Upd> + GetCtx<DialogueContext<Upd, D, S>>,
{
    fn with_dialogue<G: Guard<D> + Send + Sync + 'static>(
        self,
        guard: impl IntoGuard<D, G>,
    ) -> Self {
        let guard = guard.into_guard();
        self.with_guard(Checker(guard, PhantomData))
    }

    fn has_dialogue(self) -> Self {
        self.with_dialogue(|_| true)
    }
}

struct Checker<G, D, Upd, S>(G, PhantomData<(D, Upd, S)>);

impl<Ctx, G, D, Upd, S> IntoGuard<Ctx, Checker<G, D, Upd, S>> for Checker<G, D, Upd, S>
where
    Upd: Send + Sync + 'static,
    D: Send + Sync + 'static,
    S: Send + Sync + 'static,
    G: Guard<D>,
    Ctx: GetCtx<DialogueContext<Upd, D, S>>,
{
    fn into_guard(self) -> Checker<G, D, Upd, S> {
        self
    }
}

impl<G, Ctx, D, Upd, S> Guard<Ctx> for Checker<G, D, Upd, S>
where
    Upd: Send + Sync + 'static,
    D: Send + Sync + 'static,
    S: Send + Sync + 'static,
    G: Guard<D>,
    Ctx: GetCtx<DialogueContext<Upd, D, S>>,
{
    fn check<'a>(&self, ctx: &'a Ctx) -> BoxFuture<'a, bool> {
        let ctx = ctx.get();
        match &ctx.dialogue {
            None => Box::pin(futures::future::ready(false)),
            Some(d) => self.0.check(d),
        }
    }
}
