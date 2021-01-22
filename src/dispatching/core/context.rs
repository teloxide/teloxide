use crate::dispatching::core::from_upd::FromUpd;

pub struct Context<'a, Upd> {
    pub update: &'a Upd,
}

impl<'a, Upd> Context<'a, Upd> {
    pub fn new(update: &'a Upd) -> Self {
        Context { update }
    }
}

pub trait FromContext<Upd>: Sized {
    fn from_context(context: &Context<Upd>) -> Option<Self>;
}

impl<Upd, T> FromContext<Upd> for T
where
    T: FromUpd<Upd>,
{
    fn from_context(context: &Context<Upd>) -> Option<Self> {
        T::from_upd(context.update)
    }
}
