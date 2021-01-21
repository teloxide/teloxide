mod parser_handler;

pub use parser_handler::{MapParser, Parser, ParserHandler, ParserOut, RecombineFrom};

use crate::dispatching::core::{
    context::{Context, FromContext},
    HandleResult,
};
use futures::{future::BoxFuture, FutureExt};
use std::{future::Future, marker::PhantomData};

pub type HandleFuture<Err> = BoxFuture<'static, HandleResult<Err>>;

pub trait Handler<Data, Err, Fut: Future> {
    fn handle(&self, data: Data) -> Result<Fut, Data>;
}

pub trait IntoHandler<T> {
    fn into_handler(self) -> T;
}

pub struct FnHandlerWrapper<F, P, Fut> {
    f: F,
    phantom: PhantomData<(P, Fut)>,
}

impl<F, P, Fut> FnHandlerWrapper<F, P, Fut> {
    pub fn new(f: F) -> Self {
        FnHandlerWrapper { f, phantom: PhantomData }
    }
}

impl<Upd, Err, F, Fut> Handler<Upd, Err, HandleFuture<Err>> for FnHandlerWrapper<F, (), Fut>
where
    F: Fn() -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: Into<HandleResult<Err>> + Send,
{
    fn handle(&self, _: Upd) -> Result<HandleFuture<Err>, Upd> {
        Ok(Box::pin((self.f)().then(|x| async move { x.into() })) as _)
    }
}

impl<Upd, Err, F> Handler<Upd, Err, HandleFuture<Err>> for FnHandlerWrapper<F, (), private::Sealed>
where
    F: Fn(),
{
    fn handle(&self, _: Upd) -> Result<HandleFuture<Err>, Upd> {
        (self.f)();
        Ok(Box::pin(async { HandleResult::Ok }))
    }
}

impl<F, Upd, A, Fut, Err> Handler<Upd, Err, HandleFuture<Err>> for FnHandlerWrapper<F, (A,), Fut>
where
    A: FromContext<Upd>,
    F: Fn(A) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: Into<HandleResult<Err>> + Send,
{
    fn handle(&self, update: Upd) -> Result<HandleFuture<Err>, Upd> {
        let context = Context::new(&update);
        Ok(
            Box::pin(
                (self.f)(FromContext::from_context(&context)).then(|x| async move { x.into() }),
            ) as _,
        )
    }
}
impl<F, Upd, A, Err> Handler<Upd, Err, HandleFuture<Err>>
    for FnHandlerWrapper<F, (A,), private::Sealed>
where
    A: FromContext<Upd>,
    F: Fn(A),
{
    fn handle(&self, update: Upd) -> Result<HandleFuture<Err>, Upd> {
        let context = Context::new(&update);
        (self.f)(FromContext::from_context(&context));
        Ok(Box::pin(async { HandleResult::Ok }))
    }
}
/*
impl<F> HandlerInto<F> for F {
    fn into_handler(self) -> F {
        self
    }
}*/

mod private {
    pub struct Sealed;
}

impl<F, Fut: Future> IntoHandler<FnHandlerWrapper<F, (), Fut>> for F
where
    F: Fn() -> Fut,
{
    fn into_handler(self) -> FnHandlerWrapper<F, (), Fut> {
        FnHandlerWrapper::new(self)
    }
}

impl<F> IntoHandler<FnHandlerWrapper<F, (), private::Sealed>> for F
where
    F: Fn(),
{
    fn into_handler(self) -> FnHandlerWrapper<F, (), private::Sealed> {
        FnHandlerWrapper::new(self)
    }
}

impl<F, A, Fut: Future> IntoHandler<FnHandlerWrapper<F, (A,), Fut>> for F
where
    F: Fn(A) -> Fut,
{
    fn into_handler(self) -> FnHandlerWrapper<F, (A,), Fut> {
        FnHandlerWrapper::new(self)
    }
}

impl<F, A> IntoHandler<FnHandlerWrapper<F, (A,), private::Sealed>> for F
where
    F: Fn(A),
{
    fn into_handler(self) -> FnHandlerWrapper<F, (A,), private::Sealed> {
        FnHandlerWrapper::new(self)
    }
}
