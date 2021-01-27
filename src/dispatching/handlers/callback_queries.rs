use crate::{
    dispatching::{
        core::{Context, GetCtx, Guard, IntoGuard},
        dispatcher_context::DispatcherContext,
        handlers::common::UpdateKindHandlerBuilder,
    },
    types::{User},
};
use futures::future::BoxFuture;
use crate::types::{CallbackQuery, Message};

pub type CallbackQueriesHandlerBuilder<Ctx, Parser, Err> =
    UpdateKindHandlerBuilder<CallbackQuery, Ctx, Parser, Err>;

impl CallbackQuery {
    fn get_id(&self) -> Option<&str> {
        Some(&self.id)
    }
    fn get_from(&self) -> Option<&User> {
        Some(&self.from)
    }
    fn get_message(&self) -> Option<&Message> {
        self.message.as_ref()
    }
    fn get_inline_message_id(&self) -> Option<&str> {
        self.inline_message_id.as_ref().map(|s| s.as_str())
    }
    fn get_chat_instance(&self) -> Option<&str> {
        Some(&self.chat_instance)
    }
    fn get_game_short_name(&self) -> Option<&str> {
        self.game_short_name.as_ref().map(|x| x.as_str())
    }
}

macro_rules! impl_with_and_or {
    ($(($ident:ident, $item:ty, $get_field:expr),)*) => {$(const _: () = {
        struct Checker<G> {
            guard: G,
        }

        impl<Ctx, G: Guard<$item>> Guard<Ctx> for Checker<G>
        where
            Ctx: GetCtx<DispatcherContext<CallbackQuery>>
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
            Ctx: GetCtx<DispatcherContext<CallbackQuery>>
        {
            fn into_guard(self) -> Self {
                self
            }
        }
        paste::paste! {
        impl<Ctx, UpdateParser, Err: Send + 'static> CallbackQueriesHandlerBuilder<Ctx, UpdateParser, Err>
         where
            Ctx: Context<Upd = CallbackQuery> + GetCtx<DispatcherContext<CallbackQuery>> + Send + Sync + 'static,
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
    (id, str, CallbackQuery::get_id),
    (from, User, CallbackQuery::get_from),
    (message, Message, CallbackQuery::get_message),
    (inline_message_id, str, CallbackQuery::get_inline_message_id),
    (chat_instance, str, CallbackQuery::get_chat_instance),
    (game_short_name, str, CallbackQuery::get_game_short_name),
}
