use futures::{future::BoxFuture, FutureExt};
use std::{future::Future, marker::PhantomData};
use crate::dispatching::core::Context;

pub trait AsyncBorrowSendFn<'a, T>
where
    T: ?Sized + 'static,
    Self: Fn(&'a T) -> <Self as AsyncBorrowSendFn<'a, T>>::Fut,
{
    type Out;
    type Fut: Future<Output = Self::Out> + Send + 'a;
}

impl<'a, T, F, Fut> AsyncBorrowSendFn<'a, T> for F
where
    T: ?Sized + 'static,
    F: Fn(&'a T) -> Fut,
    Fut: Future + Send + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

pub trait Guard<Upd: ?Sized> {
    fn check<'a>(&self, update: &'a Upd) -> BoxFuture<'a, bool>;
}

pub struct GuardFnWrapper<F, Infer>(F, PhantomData<tokio::sync::Mutex<Infer>>);

impl<F, Infer> GuardFnWrapper<F, Infer> {
    pub fn new(func: F) -> Self {
        GuardFnWrapper(func, PhantomData)
    }
}

pub struct GiveSomeReturnFuture;
pub struct GiveSomeReturnBool;
pub struct GiveCtxToUpdReturnFuture;
pub struct GiveCtxToUpdReturnBool;

impl<F, Upd> Guard<Upd> for GuardFnWrapper<F, GiveSomeReturnFuture>
where
    Upd: ?Sized + 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn check<'a>(&self, cx: &'a Upd) -> BoxFuture<'a, bool> {
        Box::pin((self.0)(&cx))
    }
}

impl<F, Upd> Guard<Upd> for GuardFnWrapper<F, GiveSomeReturnBool>
where
    Upd: ?Sized,
    F: Fn(&Upd) -> bool,
{
    fn check<'a>(&self, cx: &'a Upd) -> BoxFuture<'a, bool> {
        Box::pin(futures::future::ready((self.0)(&cx))) as _
    }
}

impl<Ctx, F, Upd> Guard<Ctx> for GuardFnWrapper<F, GiveCtxToUpdReturnFuture>
where
    Ctx: Context<Upd = Upd>,
    Upd: 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn check<'a>(&self, cx: &'a Ctx) -> BoxFuture<'a, bool> {
        Box::pin((self.0)(cx.get_upd()))
    }
}

impl<Ctx, F, Upd> Guard<Ctx> for GuardFnWrapper<F, GiveCtxToUpdReturnBool>
where
    Ctx: Context<Upd = Upd>,
    F: Fn(&Upd) -> bool,
{
    fn check<'a>(&self, cx: &'a Ctx) -> BoxFuture<'a, bool> {
        Box::pin(futures::future::ready((self.0)(cx.get_upd()))) as _
    }
}

pub trait IntoGuard<Upd: ?Sized, T: Guard<Upd>> {
    fn into_guard(self) -> T;
}

impl<F, T, Upd> IntoGuard<Upd, GuardFnWrapper<F, T>> for GuardFnWrapper<F, T>
where
    Upd: ?Sized,
    GuardFnWrapper<F, T>: Guard<Upd>,
{
    fn into_guard(self) -> GuardFnWrapper<F, T> {
        self
    }
}

impl<F, Upd> IntoGuard<Upd, GuardFnWrapper<F, GiveSomeReturnFuture>> for F
where
    Upd: ?Sized + 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn into_guard(self) -> GuardFnWrapper<F, GiveSomeReturnFuture> {
        GuardFnWrapper::new(self)
    }
}

impl<F, Upd> IntoGuard<Upd, GuardFnWrapper<F, GiveSomeReturnBool>> for F
where
    Upd: ?Sized,
    F: Fn(&Upd) -> bool,
{
    fn into_guard(self) -> GuardFnWrapper<F, GiveSomeReturnBool> {
        GuardFnWrapper::new(self)
    }
}

impl<Ctx, F, Upd> IntoGuard<Ctx, GuardFnWrapper<F, GiveCtxToUpdReturnFuture>> for F
where
    Ctx: Context<Upd = Upd>,
    Upd: 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn into_guard(self) -> GuardFnWrapper<F, GiveCtxToUpdReturnFuture> {
        GuardFnWrapper::new(self)
    }
}

impl<Ctx, F, Upd> IntoGuard<Ctx, GuardFnWrapper<F, GiveCtxToUpdReturnBool>> for F
where
    Ctx: Context<Upd = Upd>,
    F: Fn(&Upd) -> bool,
{
    fn into_guard(self) -> GuardFnWrapper<F, GiveCtxToUpdReturnBool> {
        GuardFnWrapper::new(self)
    }
}

impl<Upd> Guard<Upd> for Box<dyn Guard<Upd> + Send + Sync> {
    fn check<'a>(&self, update: &'a Upd) -> BoxFuture<'a, bool> {
        (**self).check(update)
    }
}

pub struct Guards<Upd> {
    guards: Vec<Box<dyn Guard<Upd> + Send + Sync>>,
}

impl<Upd> Guards<Upd> {
    pub fn new() -> Self {
        Guards { guards: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn add_guard<T>(&mut self, data: T)
    where
        T: Guard<Upd> + Send + Sync + 'static,
    {
        self.add_boxed_guard(Box::new(data));
    }

    pub fn add_boxed_guard(&mut self, data: Box<dyn Guard<Upd> + Send + Sync>) {
        self.guards.push(data);
    }

    pub async fn check(&self, update: &Upd) -> bool {
        Guard::check(self, update).await
    }

    pub fn is_empty(&self) -> bool {
        self.guards.is_empty()
    }
}

impl<Upd> Guard<Upd> for Guards<Upd> {
    fn check<'a>(&self, update: &'a Upd) -> BoxFuture<'a, bool> {
        Box::pin(
            futures::future::join_all(self.guards.iter().map(|guard| guard.check(update)))
                .map(|results| results.iter().all(|&x| x)),
        ) as _
    }
}

pub struct OrGuard<Left, Right>(Left, Right);

impl<Left, Right> OrGuard<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        OrGuard(left, right)
    }
}

impl<Left, Right, Upd> Guard<Upd> for OrGuard<Left, Right>
where
    Left: Guard<Upd>,
    Right: Guard<Upd>,
{
    fn check<'a>(&self, update: &'a Upd) -> BoxFuture<'a, bool> {
        Box::pin(
            futures::future::join(self.0.check(update), self.1.check(update)).map(|(x, y)| x || y),
        ) as _
    }
}
