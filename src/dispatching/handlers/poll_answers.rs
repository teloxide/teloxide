use crate::{
    dispatching::{
        core::{Context, GetCtx, Guard, IntoGuard},
        dispatcher_context::DispatcherContext,
        handlers::common::UpdateKindHandlerBuilder,
    },
};
use futures::future::BoxFuture;
use crate::types::{PollAnswer, User};

pub type PollAnswersHandlerBuilder<Ctx, Parser, Err> =
    UpdateKindHandlerBuilder<PollAnswer, Ctx, Parser, Err>;

impl PollAnswer {
    fn get_poll_id(&self) -> Option<&str> {
        Some(&self.poll_id)
    }
    fn get_user(&self) -> Option<&User> {
        Some(&self.user)
    }
    fn get_option_ids(&self) -> Option<&[i32]> {
        Some(self.option_ids.as_ref())
    }
}

macro_rules! impl_with_and_or {
    ($(($ident:ident, $item:ty, $get_field:expr),)*) => {$(const _: () = {
        struct Checker<G> {
            guard: G,
        }

        impl<Ctx, G: Guard<$item>> Guard<Ctx> for Checker<G>
        where
            Ctx: GetCtx<DispatcherContext<PollAnswer>>
        {
            fn check<'a>(&self, cx: &'a Ctx) -> BoxFuture<'a, bool> {
                let cx = cx.get();
                match $get_field(&cx.upd) {
                    Some(x) => self.guard.check(x),
                    None => Box::pin(futures::future::ready(false)) as _,
                }
            }
        }

        impl<Ctx, G: Guard<$item>> IntoGuard<Ctx, Checker<G>> for Checker<G>
        where
            Ctx: GetCtx<DispatcherContext<PollAnswer>>
        {
            fn into_guard(self) -> Self {
                self
            }
        }
        paste::paste! {
        impl<Ctx, UpdateParser, Err: Send + 'static> PollAnswersHandlerBuilder<Ctx, UpdateParser, Err>
         where
            Ctx: Context<Upd = PollAnswer> + GetCtx<DispatcherContext<PollAnswer>> + Send + Sync + 'static,
         {
            pub fn [<with_ $ident>]<G: Guard<$item> + Send + Sync + 'static>(self, guard: impl IntoGuard<$item, G> + 'static) -> Self {
                let checker = Checker { guard: guard.into_guard() };
                self.with_guard(checker)
            }
            pub fn [<or_with_ $ident>]<G: Guard<$item> + Send + Sync + 'static>(self, guard: impl IntoGuard<$item, G> + 'static) -> Self {
                let checker = Checker { guard: guard.into_guard() };
                self.or_with_guard(checker)
            }
        }
        }
    };)*}
}

impl_with_and_or! {
    (poll_id, str, PollAnswer::get_poll_id),
    (user, User, PollAnswer::get_user),
    (option_ids, [i32], PollAnswer::get_option_ids),
}

impl<Ctx, UpdateParser, Err: Send + 'static> PollAnswersHandlerBuilder<Ctx, UpdateParser, Err>
where
    Ctx: Context<Upd = PollAnswer> + Send + Sync + 'static,
{
    /// Guard that return `true` if the user restrict vote (`option_ids` is empty).
    pub fn when_user_restrict_vote(self) -> Self {
        self.with_guard(|a: &PollAnswer| a.option_ids.is_empty())
    }
}
