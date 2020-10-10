use crate::contrib::parser::DataWithUWC;
use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait Handler {
    type Data;
    type Update;
    async fn handle(&self, data: DataWithUWC<Self::Data, Self::Update>);
}

/// Wrapper for `Fn(DataWithUWC<D, U>)`. Needed for bypass `Rust` compiler restrictions on implements traits.
pub struct HandlerWrapper<F, D, U>
    where
        F: Fn(DataWithUWC<D, U>)
{
    f: F,
    phantom1: PhantomData<D>,
    phantom2: PhantomData<U>,
}
impl<F, D, U> From<F> for HandlerWrapper<F, D, U>
    where
        F: Fn(DataWithUWC<D, U>)
{
    fn from(f: F) -> Self {
        Self {
            f,
            phantom1: PhantomData,
            phantom2: PhantomData
        }
    }
}

#[async_trait::async_trait]
impl<F, D, U> Handler for HandlerWrapper<F, D, U>
    where
        F: Fn(DataWithUWC<D, U>) + Send + Sync + 'static,
        D: Send + Sync + 'static,
        U: Send + Sync + 'static,
{
    type Data = D;
    type Update = U;

    async fn handle(&self, data: DataWithUWC<Self::Data, Self::Update>) {
        (self.f)(data)
    }
}
