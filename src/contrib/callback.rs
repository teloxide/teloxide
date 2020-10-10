use crate::contrib::parser::{Parser};
use crate::dispatching::UpdateWithCx;
use crate::contrib::handler::Handler;

#[async_trait::async_trait]
pub trait Callback {
    type Update;
    async fn try_handle(&self, input: UpdateWithCx<Self::Update>) -> Result<(), UpdateWithCx<Self::Update>>;
}

#[async_trait::async_trait]
impl<T> Callback for T 
    where 
        T: Parser + Handler<Data = <Self as Parser>::Output, Update = <Self as Parser>::Update> + Send + Sync + 'static,
{
    type Update = <Self as Parser>::Update;

    async fn try_handle(&self, input: UpdateWithCx<<Self as Parser>::Update>) -> Result<(), UpdateWithCx<<Self as Parser>::Update>> {
        match self.parse(input) {
            Ok(d) => {
                self.handle(d).await;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Alternative<C1, C2> 
    where 
        C1: Callback,
        C2: Callback<Update=C1::Update>,
{
    left: C1,
    right: C2,
}

impl<C1, C2> Alternative<C1, C2> 
    where
        C1: Callback,
        C2: Callback<Update=C1::Update>, 
{
    pub fn new(left: C1, right: C2) -> Self {
        Alternative { left, right }
    }
}

#[async_trait::async_trait]
impl<C1, C2> Callback for Alternative<C1, C2>
    where
        C1: Callback + Send + Sync + 'static,
        C2: Callback<Update=C1::Update> + Send + Sync + 'static,
        <C1 as Callback>::Update: Send + Sync + 'static
{
    type Update = C1::Update;

    async fn try_handle(&self, input: UpdateWithCx<Self::Update>) -> Result<(), UpdateWithCx<Self::Update>> {
        match self.left.try_handle(input).await {
            Ok(_) => Ok(()),
            Err(input) => self.right.try_handle(input).await
        }
    }
}
