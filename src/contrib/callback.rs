use crate::contrib::parser::{Parser};
use crate::dispatching::UpdateWithCx;
use crate::contrib::handler::Handler;


#[async_trait::async_trait]
pub trait Callback {
    type Update;
    type Err;
    async fn try_handle(&self, input: UpdateWithCx<Self::Update>) -> Result<Result<(), Self::Err>, UpdateWithCx<Self::Update>>;
}

#[async_trait::async_trait]
impl<T> Callback for T 
    where 
        T: Parser + Handler<Data = <Self as Parser>::Output, Update = <Self as Parser>::Update> + Sync,
        <T as Parser>::Update: Send,
        <T as Parser>::Output: Send,
{
    type Update = <Self as Parser>::Update;
    type Err = <Self as Handler>::Err;

    async fn try_handle(&self, input: UpdateWithCx<<Self as Parser>::Update>) -> Result<Result<(), Self::Err>, UpdateWithCx<<Self as Parser>::Update>> {
        match self.parse(input) {
            Ok(d) => {
                Ok(self.handle(d).await)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Alternative<C1, C2> 
    where 
        C1: Callback,
        C2: Callback<Update=C1::Update, Err=C1::Err>,
{
    left: C1,
    right: C2,
}

impl<C1, C2> Alternative<C1, C2> 
    where
        C1: Callback,
        C2: Callback<Update=C1::Update, Err=C1::Err>, 
{
    pub fn new(left: C1, right: C2) -> Self {
        Alternative { left, right }
    }
}

#[async_trait::async_trait]
impl<C1, C2> Callback for Alternative<C1, C2>
    where
        C1: Callback + Send + Sync,
        C2: Callback<Update=C1::Update, Err=C1::Err> + Send + Sync,
        <C1 as Callback>::Update: Send,
        <C1 as Callback>::Err: Send
{
    type Update = C1::Update;
    type Err = C1::Err;

    async fn try_handle(&self, input: UpdateWithCx<Self::Update>) -> Result<Result<(), Self::Err>, UpdateWithCx<Self::Update>> {
        match self.left.try_handle(input).await {
            Ok(res) => Ok(res),
            Err(input) => self.right.try_handle(input).await
        }
    }
}
