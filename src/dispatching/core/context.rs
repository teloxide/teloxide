use crate::dispatching::dispatcher_context::DispatcherContext;
use crate::dispatching::core::{ParserOut, RecombineFrom};

pub trait Context {
    type Upd;

    fn get_upd(&self) -> &Self::Upd;
}

pub trait ContextWith<Elem>: Context {
    type Context: Context<Upd = Elem>;
}

pub trait ParseContext<To>: ContextWith<To> + Sized {
    fn parse<Rest>(self, f: impl Fn(Self::Upd) -> Result<ParserOut<To, Rest>, Self::Upd>) -> Result<(Self::Context, Rest), Self>;
    fn recombine<Parser, Rest>(info: ParserOut<Self::Context, Rest>) -> Self
    where
        Self::Upd: RecombineFrom<Parser, To, Rest>;
}

pub trait FromContext<Ctx>: Sized {
    fn from_context(context: &Ctx) -> Option<Self>;
}

pub trait FromContextOwn<Ctx, RequireCtx=Ctx>: Sized {
    fn from_context(context: Ctx) -> Self;
}

impl<Upd> FromContextOwn<DispatcherContext<Upd>> for Upd {
    fn from_context(context: DispatcherContext<Upd>) -> Self {
        context.upd
    }
}

pub trait GetCtx<Ctx> {
    fn get(&self) -> &Ctx;
    fn get_own(self) -> Ctx;
}
