use crate::{
    dispatching::{
        core::{Context, GetCtx, Guard, IntoGuard},
        dispatcher_context::DispatcherContext,
        handlers::common::UpdateKindHandlerBuilder,
    },
    types::{ChosenInlineResult, Location, User},
};
use futures::future::BoxFuture;

pub type ChosenInlineResultsHandlerBuilder<Ctx, Parser, Err> =
    UpdateKindHandlerBuilder<ChosenInlineResult, Ctx, Parser, Err>;

impl ChosenInlineResult {
    fn get_result_id(&self) -> Option<&str> {
        Some(&self.result_id)
    }
    fn get_from(&self) -> Option<&User> {
        Some(&self.from)
    }
    fn get_location(&self) -> Option<&Location> {
        self.location.as_ref()
    }
    fn get_query(&self) -> Option<&str> {
        Some(&self.query)
    }
    fn get_inline_message_id(&self) -> Option<&str> {
        self.inline_message_id.as_ref().map(|s| s.as_str())
    }
}

macro_rules! impl_with_and_or {
    ($(($ident:ident, $item:ty, $get_field:expr),)*) => {$(const _: () = {
        struct Checker<G> {
            guard: G,
        }

        impl<Ctx, G: Guard<$item>> Guard<Ctx> for Checker<G>
        where
            Ctx: GetCtx<DispatcherContext<ChosenInlineResult>>
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
            Ctx: GetCtx<DispatcherContext<ChosenInlineResult>>
        {
            fn into_guard(self) -> Self {
                self
            }
        }
        paste::paste! {
        impl<Ctx, UpdateParser, Err: Send + 'static> ChosenInlineResultsHandlerBuilder<Ctx, UpdateParser, Err>
         where
            Ctx: Context<Upd = ChosenInlineResult> + GetCtx<DispatcherContext<ChosenInlineResult>> + Send + Sync + 'static,
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
    (result_id, str, ChosenInlineResult::get_result_id),
    (from, User, ChosenInlineResult::get_from),
    (location, Location, ChosenInlineResult::get_location),
    (query, str, ChosenInlineResult::get_query),
    (inline_message_id, str, ChosenInlineResult::get_inline_message_id),
}

impl<Ctx, UpdateParser, Err: Send + 'static>
    ChosenInlineResultsHandlerBuilder<Ctx, UpdateParser, Err>
where
    Ctx: Context<Upd = ChosenInlineResult> + Send + Sync + 'static,
{
    pub fn has_location(self) -> Self {
        self.with_guard(|query: &ChosenInlineResult| query.location.is_some())
    }
    pub fn no_has_location(self) -> Self {
        self.with_guard(|query: &ChosenInlineResult| query.location.is_none())
    }
}
