#![allow(non_snake_case)]
#![allow(unused_variables)]

mod parser_handler;

pub use parser_handler::{MapParser, Parser, ParserHandler, ParserOut, RecombineFrom};

use crate::dispatching::{
    core::{FromContext, FromContextOwn, HandleResult},
    dispatcher_context::DispatcherContext,
};
use futures::{future::BoxFuture, FutureExt};
use std::{future::Future, marker::PhantomData};

pub type HandleFuture<Err, Data> = BoxFuture<'static, Result<HandleResult<Err>, Data>>;

pub trait Handler<Data, Err> {
    fn handle(&self, data: Data) -> HandleFuture<Err, Data>;
}

pub trait IntoHandler<T> {
    fn into_handler(self) -> T;
}

pub struct FnHandlerWrapper<F, Upd, P, Fut> {
    f: F,
    phantom: PhantomData<(P, Upd, Fut)>,
}

impl<F, Upd, P, Fut> FnHandlerWrapper<F, Upd, P, Fut> {
    pub fn new(f: F) -> Self {
        FnHandlerWrapper { f, phantom: PhantomData }
    }
}

macro_rules! impl_handler_and_into {
    ($(($($gen:ident),*),)*) => {$(
        impl<$($gen,)* Upd, Err, F, Fut> Handler<DispatcherContext<Upd>, Err> for FnHandlerWrapper<F, Upd, ($($gen,)*), (Fut, )>
        where
            Upd: Send + 'static,
            Err: Send + 'static,
            $($gen: FromContext<Upd>,)*
            F: Fn($($gen),*) -> Fut,
            Fut: Future + Send + 'static,
            Fut::Output: Into<HandleResult<Err>> + Send,
        {
            fn handle(&self, cx: DispatcherContext<Upd>) -> HandleFuture<Err, DispatcherContext<Upd>> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                let fut = (self.f)($($gen),*);
                Box::pin(fut.map(Into::into).map(Ok)) as _
            }
        }
        impl<$($gen,)* F, Upd, Fut: Future> IntoHandler<FnHandlerWrapper<F, Upd, ($($gen,)*), (Fut, )>> for F
        where
            $($gen: FromContext<Upd>,)*
            F: Fn($($gen),*) -> Fut,
        {
            fn into_handler(self) -> FnHandlerWrapper<F, Upd, ($($gen,)*), (Fut, )> {
                FnHandlerWrapper::new(self)
            }
        }

        impl<$($gen,)* Upd, Err, F> Handler<DispatcherContext<Upd>, Err> for FnHandlerWrapper<F, Upd, ($($gen,)*), private::Sealed>
        where
            Upd: Send + 'static,
            Err: Send + 'static,
            $($gen: FromContext<Upd>,)*
            F: Fn($($gen),*),
        {
            fn handle(&self, cx: DispatcherContext<Upd>) -> HandleFuture<Err, DispatcherContext<Upd>> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                (self.f)($($gen),*);
                Box::pin(futures::future::ready(Ok(HandleResult::Ok))) as _
            }
        }

        impl<$($gen,)* F, Upd> IntoHandler<FnHandlerWrapper<F, Upd, ($($gen,)*), private::Sealed>> for F
        where
            $($gen: FromContext<Upd>,)*
            F: Fn($($gen),*),
        {
            fn into_handler(self) -> FnHandlerWrapper<F, Upd, ($($gen,)*), private::Sealed> {
                FnHandlerWrapper::new(self)
            }
        }

        //------------------ 2

        impl<OwnTy, $($gen,)* Upd, Err, F, Fut> Handler<DispatcherContext<Upd>, Err> for FnHandlerWrapper<F, Upd, ($($gen,)*), (Fut, OwnTy)>
        where
            Upd: Send + 'static,
            Err: Send + 'static,
            $($gen: FromContext<Upd>,)*
            OwnTy: FromContextOwn<Upd>,
            F: Fn(OwnTy, $($gen),*) -> Fut,
            Fut: Future + Send + 'static,
            Fut::Output: Into<HandleResult<Err>> + Send,
        {
            fn handle(&self, cx: DispatcherContext<Upd>) -> HandleFuture<Err, DispatcherContext<Upd>> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                let fut = (self.f)(OwnTy::from_context(cx), $($gen),*);
                Box::pin(fut.map(Into::into).map(Ok)) as _
            }
        }
        impl<OwnTy, Upd, $($gen,)* F, Fut: Future> IntoHandler<FnHandlerWrapper<F, Upd, ($($gen,)*), (Fut, OwnTy)>> for F
        where
            OwnTy: FromContextOwn<Upd>,
            $($gen: FromContext<Upd>,)*
            F: Fn(OwnTy, $($gen),*) -> Fut,
        {
            fn into_handler(self) -> FnHandlerWrapper<F, Upd, ($($gen,)*), (Fut, OwnTy)> {
                FnHandlerWrapper::new(self)
            }
        }

        impl<$($gen,)* OwnTy, Upd, Err, F> Handler<DispatcherContext<Upd>, Err> for FnHandlerWrapper<F, Upd, ($($gen,)*), (private::Sealed, OwnTy)>
        where
            Upd: Send + 'static,
            Err: Send + 'static,
            OwnTy: FromContextOwn<Upd>,
            $($gen: FromContext<Upd>,)*
            F: Fn(OwnTy, $($gen),*),
        {
            fn handle(&self, cx: DispatcherContext<Upd>) -> HandleFuture<Err, DispatcherContext<Upd>> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                (self.f)(OwnTy::from_context(cx), $($gen),*);
                Box::pin(futures::future::ready(Ok(HandleResult::Ok))) as _
            }
        }

        impl<OwnTy, Upd, $($gen,)* F> IntoHandler<FnHandlerWrapper<F, Upd, ($($gen,)*), (private::Sealed, OwnTy)>> for F
        where
            $($gen: FromContext<Upd>,)*
            OwnTy: FromContextOwn<Upd>,
            F: Fn(OwnTy, $($gen),*),
        {
            fn into_handler(self) -> FnHandlerWrapper<F, Upd, ($($gen,)*), (private::Sealed, OwnTy)> {
                FnHandlerWrapper::new(self)
            }
        }
    )*};
}

impl_handler_and_into! {
    (),
    (A),
    (A, B),
}

mod private {
    pub struct Sealed;
}
