use crate::dispatching::handlers::common::UpdateKindHandlerBuilder;
use crate::types::{InlineQuery, Location, User};
use crate::dispatching::core::{Guard, IntoGuard};
use crate::dispatching::dispatcher_context::DispatcherContext;
use futures::future::BoxFuture;

pub type InlineQueriesHandlerBuilder<Parser, Err> = UpdateKindHandlerBuilder<InlineQuery, Parser, Err>;

impl InlineQuery {
    fn get_id(&self) -> Option<&str> {
       Some(&self.id)
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
    fn get_offset(&self) -> Option<&str> {
        Some(&self.offset)
    }
}

macro_rules! impl_with_and_or {
    ($(($ident:ident, $item:ty, $get_field:expr),)*) => {$(const _: () = {
        struct Checker<G> {
            guard: G,
        }

        impl<G: Guard<$item>> Guard<DispatcherContext<InlineQuery>> for Checker<G> {
            fn check<'a>(&self, cx: &'a DispatcherContext<InlineQuery>) -> BoxFuture<'a, bool> {
                match $get_field(&cx.upd) {
                    Some(x) => self.guard.check(x),
                    None => Box::pin(futures::future::ready(false)) as _,
                }
            }
        }

        impl<G: Guard<$item>> IntoGuard<DispatcherContext<InlineQuery>, Checker<G>> for Checker<G> {
            fn into_guard(self) -> Self {
                self
            }
        }
        paste::paste! {
        impl<UpdateParser, Err: Send + Sync + 'static> InlineQueriesHandlerBuilder<UpdateParser, Err> {
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
    (id, str, InlineQuery::get_id),
    (from, User, InlineQuery::get_from),
    (location, Location, InlineQuery::get_location),
    (query, str, InlineQuery::get_query),
    (get_offset, str, InlineQuery::get_offset),
}

impl<UpdateParser, Err: Send + Sync + 'static> InlineQueriesHandlerBuilder<UpdateParser, Err> {
    pub fn has_location(self) -> Self {
        self.with_guard(|query: &InlineQuery| query.location.is_some())
    }
    pub fn no_has_location(self) -> Self {
        self.with_guard(|query: &InlineQuery| query.location.is_none())
    }
}
