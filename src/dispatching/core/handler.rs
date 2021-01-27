#![allow(non_snake_case)]
#![allow(unused_variables)]

mod parser_handler;

pub use parser_handler::{Parser, ParserHandler, ParserOut, RecombineFrom};

use crate::dispatching::core::{FromContext, FromContextOwn, HandleResult};
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
    phantom: PhantomData<tokio::sync::Mutex<(P, Upd, Fut)>>,
}

impl<F, Upd, P, Fut> FnHandlerWrapper<F, Upd, P, Fut> {
    pub fn new(f: F) -> Self {
        FnHandlerWrapper { f, phantom: PhantomData }
    }
}

macro_rules! impl_handler_and_into {
    ($(($($gen:ident),*),)*) => {$(
        impl<$($gen,)* Ctx, Err, F, Fut> Handler<Ctx, Err> for FnHandlerWrapper<F, Ctx, ($($gen,)*), (Fut, )>
        where
            Ctx: Send + 'static,
            Err: Send + 'static,
            $($gen: FromContext<Ctx>,)*
            F: Fn($($gen),*) -> Fut,
            Fut: Future + Send + 'static,
            Fut::Output: Into<HandleResult<Err>> + Send,
        {
            fn handle(&self, cx: Ctx) -> HandleFuture<Err, Ctx> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                let fut = (self.f)($($gen),*);
                Box::pin(fut.map(Into::into).map(Ok)) as _
            }
        }
        impl<$($gen,)* F, Ctx, Fut: Future> IntoHandler<FnHandlerWrapper<F, Ctx, ($($gen,)*), (Fut, )>> for F
        where
            $($gen: FromContext<Ctx>,)*
            F: Fn($($gen),*) -> Fut,
        {
            #[inline]
            fn into_handler(self) -> FnHandlerWrapper<F, Ctx, ($($gen,)*), (Fut, )> {
                FnHandlerWrapper::new(self)
            }
        }

        impl<$($gen,)* Ctx, Err, F> Handler<Ctx, Err> for FnHandlerWrapper<F, Ctx, ($($gen,)*), private::Sealed>
        where
            Ctx: Send + 'static,
            Err: Send + 'static,
            $($gen: FromContext<Ctx>,)*
            F: Fn($($gen),*),
        {
            fn handle(&self, cx: Ctx) -> HandleFuture<Err, Ctx> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                (self.f)($($gen),*);
                Box::pin(futures::future::ready(Ok(HandleResult::Ok))) as _
            }
        }

        impl<$($gen,)* F, Ctx> IntoHandler<FnHandlerWrapper<F, Ctx, ($($gen,)*), private::Sealed>> for F
        where
            $($gen: FromContext<Ctx>,)*
            F: Fn($($gen),*),
        {
            #[inline]
            fn into_handler(self) -> FnHandlerWrapper<F, Ctx, ($($gen,)*), private::Sealed> {
                FnHandlerWrapper::new(self)
            }
        }

        //------------------ 2

        impl<OwnTy, $($gen,)* Ctx, Err, F, Fut> Handler<Ctx, Err> for FnHandlerWrapper<F, Ctx, ($($gen,)*), (Fut, OwnTy)>
        where
            Ctx: Send + 'static,
            Err: Send + 'static,
            $($gen: FromContext<Ctx>,)*
            OwnTy: FromContextOwn<Ctx>,
            F: Fn(OwnTy, $($gen),*) -> Fut,
            Fut: Future + Send + 'static,
            Fut::Output: Into<HandleResult<Err>> + Send,
        {
            fn handle(&self, cx: Ctx) -> HandleFuture<Err, Ctx> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                let fut = (self.f)(OwnTy::from_context(cx), $($gen),*);
                Box::pin(fut.map(Into::into).map(Ok)) as _
            }
        }
        impl<OwnTy, Ctx, $($gen,)* F, Fut: Future> IntoHandler<FnHandlerWrapper<F, Ctx, ($($gen,)*), (Fut, OwnTy)>> for F
        where
            OwnTy: FromContextOwn<Ctx>,
            $($gen: FromContext<Ctx>,)*
            F: Fn(OwnTy, $($gen),*) -> Fut,
        {
            #[inline]
            fn into_handler(self) -> FnHandlerWrapper<F, Ctx, ($($gen,)*), (Fut, OwnTy)> {
                FnHandlerWrapper::new(self)
            }
        }

        impl<$($gen,)* OwnTy, Ctx, Err, F> Handler<Ctx, Err> for FnHandlerWrapper<F, Ctx, ($($gen,)*), (private::Sealed, OwnTy)>
        where
            Ctx: Send + 'static,
            Err: Send + 'static,
            OwnTy: FromContextOwn<Ctx>,
            $($gen: FromContext<Ctx>,)*
            F: Fn(OwnTy, $($gen),*),
        {
            fn handle(&self, cx: Ctx) -> HandleFuture<Err, Ctx> {
                $(let $gen = match <$gen>::from_context(&cx) {
                    Some(t) => t,
                    None => return Box::pin(futures::future::ready(Err(cx))) as _
                };)*
                (self.f)(OwnTy::from_context(cx), $($gen),*);
                Box::pin(futures::future::ready(Ok(HandleResult::Ok))) as _
            }
        }

        impl<OwnTy, Ctx, $($gen,)* F> IntoHandler<FnHandlerWrapper<F, Ctx, ($($gen,)*), (private::Sealed, OwnTy)>> for F
        where
            $($gen: FromContext<Ctx>,)*
            OwnTy: FromContextOwn<Ctx>,
            F: Fn(OwnTy, $($gen),*),
        {
            #[inline]
            fn into_handler(self) -> FnHandlerWrapper<F, Ctx, ($($gen,)*), (private::Sealed, OwnTy)> {
                FnHandlerWrapper::new(self)
            }
        }
    )*};
}

impl_handler_and_into! {
    (),
    (A),
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F1),
    (A, B, C, D, E, F1, G),
    (A, B, C, D, E, F1, G, H),
}

mod private {
    pub struct Sealed;
}
