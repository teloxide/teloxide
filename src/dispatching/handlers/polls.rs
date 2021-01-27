use crate::{
    dispatching::{
        core::{Context, GetCtx, Guard, IntoGuard},
        dispatcher_context::DispatcherContext,
        handlers::common::UpdateKindHandlerBuilder,
    },
    types::{MessageEntity, Poll, PollOption, PollType},
};
use futures::future::BoxFuture;

pub type PollsHandlerBuilder<Ctx, Parser, Err> = UpdateKindHandlerBuilder<Poll, Ctx, Parser, Err>;

// TODO: add open_period and close_date guards after they become public

impl Poll {
    fn get_id(&self) -> Option<&str> {
        Some(&self.id)
    }
    fn get_question(&self) -> Option<&str> {
        Some(&self.question)
    }
    fn get_options(&self) -> Option<&[PollOption]> {
        Some(self.options.as_ref())
    }
    fn get_poll_type(&self) -> Option<&PollType> {
        Some(&self.poll_type)
    }
    fn get_correct_option_id(&self) -> Option<&i32> {
        self.correct_option_id.as_ref()
    }
    fn get_correct_option(&self) -> Option<&PollOption> {
        let id = self.correct_option_id.as_ref()?;
        Some(&self.options[*id as usize])
    }
    fn get_explanation(&self) -> Option<&str> {
        self.explanation.as_ref().map(|x| x.as_str())
    }
    fn get_explanation_entities(&self) -> Option<&[MessageEntity]> {
        self.explanation_entities.as_ref().map(|x| x.as_slice())
    }
}

macro_rules! impl_with_and_or {
    ($(($ident:ident, $item:ty, $get_field:expr),)*) => {$(const _: () = {
        struct Checker<G> {
            guard: G,
        }

        impl<Ctx, G: Guard<$item>> Guard<Ctx> for Checker<G>
        where
            Ctx: GetCtx<DispatcherContext<Poll>>
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
            Ctx: GetCtx<DispatcherContext<Poll>>
        {
            fn into_guard(self) -> Self {
                self
            }
        }
        paste::paste! {
        impl<Ctx, UpdateParser, Err: Send + 'static> PollsHandlerBuilder<Ctx, UpdateParser, Err>
         where
            Ctx: Context<Upd = Poll> + GetCtx<DispatcherContext<Poll>> + Send + Sync + 'static,
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
    (id, str, Poll::get_id),
    (question, str, Poll::get_question),
    (options, [PollOption], Poll::get_options),
    (poll_type, PollType, Poll::get_poll_type),
    (correct_option_id, i32, Poll::get_correct_option_id),
    (correct_option, PollOption, Poll::get_correct_option),
    (explanation, str, Poll::get_explanation),
    (explanation_entities, [MessageEntity], Poll::get_explanation_entities),
}

impl<Ctx, UpdateParser, Err: Send + 'static> PollsHandlerBuilder<Ctx, UpdateParser, Err>
where
    Ctx: Context<Upd = Poll> + Send + Sync + 'static,
{
    pub fn when_is_anonymous(self) -> Self {
        self.with_guard(|poll: &Poll| poll.is_anonymous)
    }
    pub fn when_is_not_anonymous(self) -> Self {
        self.with_guard(|poll: &Poll| !poll.is_anonymous)
    }
    pub fn when_allows_multiple_answers(self) -> Self {
        self.with_guard(|poll: &Poll| poll.allows_multiple_answers)
    }
    pub fn when_not_allows_multiple_answers(self) -> Self {
        self.with_guard(|poll: &Poll| !poll.allows_multiple_answers)
    }
    pub fn when_is_closed(self) -> Self {
        self.with_guard(|poll: &Poll| poll.allows_multiple_answers)
    }
    pub fn when_is_not_closed(self) -> Self {
        self.with_guard(|poll: &Poll| !poll.allows_multiple_answers)
    }
}
