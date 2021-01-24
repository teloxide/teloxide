use crate::dispatching::{core::from_upd::FromUpd, dispatcher_context::DispatcherContext};

pub trait FromContext<Upd>: Sized {
    fn from_context(context: &DispatcherContext<Upd>) -> Option<Self>;
}

impl<Upd, T> FromContext<Upd> for T
where
    T: FromUpd<Upd>,
{
    fn from_context(context: &DispatcherContext<Upd>) -> Option<Self> {
        T::from_upd(&context.upd)
    }
}
