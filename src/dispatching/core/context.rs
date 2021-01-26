use crate::dispatching::dispatcher_context::DispatcherContext;

pub trait FromContext<Ctx>: Sized {
    fn from_context(context: &Ctx) -> Option<Self>;
}

pub trait FromContextOwn<Ctx>: Sized {
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
