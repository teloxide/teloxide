use crate::dispatching::dispatcher_context::DispatcherContext;

pub trait FromContext<Upd>: Sized {
    fn from_context(context: &DispatcherContext<Upd>) -> Option<Self>;
}

pub trait FromContextOwn<Upd>: Sized {
    fn from_context(context: DispatcherContext<Upd>) -> Self;
}

impl<Upd> FromContextOwn<Upd> for Upd {
    fn from_context(context: DispatcherContext<Upd>) -> Self {
        context.upd
    }
}
