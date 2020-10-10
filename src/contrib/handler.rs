use crate::contrib::parser::DataWithUWC;
use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait Handler {
    type Data;
    type Update;
    type Err;
    async fn handle(&self, data: DataWithUWC<Self::Data, Self::Update>) -> Result<(), Self::Err>;
}

/// Wrapper for `Fn(DataWithUWC<D, U>) -> Result<(), E>`. Needed for bypass `Rust` compiler restrictions on implements traits.
pub struct HandlerWrapper<F, D, U, E>
    where
        F: Fn(DataWithUWC<D, U>) -> Result<(), E>
{
    f: F,
    phantom1: PhantomData<D>,
    phantom2: PhantomData<U>,
    phantom3: PhantomData<E>
}
impl<F, D, U, E> From<F> for HandlerWrapper<F, D, U, E>
    where
        F: Fn(DataWithUWC<D, U>) -> Result<(), E>
{
    fn from(f: F) -> Self {
        Self {
            f,
            phantom1: PhantomData,
            phantom2: PhantomData,
            phantom3: PhantomData
        }
    }
}

#[async_trait::async_trait]
impl<F, D, U, E> Handler for HandlerWrapper<F, D, U, E>
    where
        F: Fn(DataWithUWC<D, U>) -> Result<(), E> + Sync,
        D: Send + Sync,
        U: Send + Sync,
        E: Sync,
{
    type Data = D;
    type Update = U;
    type Err = E;

    async fn handle(&self, data: DataWithUWC<Self::Data, Self::Update>) -> Result<(), Self::Err> {
        (self.f)(data)
    }
}
