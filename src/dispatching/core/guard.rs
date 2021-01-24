use crate::dispatching::dispatcher_context::DispatcherContext;
use futures::{future::BoxFuture, FutureExt};
use std::{future::Future, marker::PhantomData};

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

pub struct GuardFnWrapper<F, Infer>(F, PhantomData<Infer>);

impl<F, Infer> GuardFnWrapper<F, Infer> {
    pub fn new(func: F) -> Self {
        GuardFnWrapper(func, PhantomData)
    }
}

impl<F, Upd> Guard<Upd> for GuardFnWrapper<F, ()>
where
    Upd: ?Sized + 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn check<'a>(&self, cx: &'a Upd) -> BoxFuture<'a, bool> {
        Box::pin((self.0)(&cx))
    }
}

impl<F, Upd> Guard<Upd> for GuardFnWrapper<F, bool>
where
    Upd: ?Sized,
    F: Fn(&Upd) -> bool,
{
    fn check<'a>(&self, cx: &'a Upd) -> BoxFuture<'a, bool> {
        Box::pin(futures::future::ready((self.0)(&cx))) as _
    }
}

impl<F, Upd> Guard<DispatcherContext<Upd>> for GuardFnWrapper<F, ((), ())>
where
    Upd: 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn check<'a>(&self, cx: &'a DispatcherContext<Upd>) -> BoxFuture<'a, bool> {
        Box::pin((self.0)(&cx.upd))
    }
}

impl<F, Upd> Guard<DispatcherContext<Upd>> for GuardFnWrapper<F, (bool, (), ())>
where
    F: Fn(&Upd) -> bool,
{
    fn check<'a>(&self, cx: &'a DispatcherContext<Upd>) -> BoxFuture<'a, bool> {
        Box::pin(futures::future::ready((self.0)(&cx.upd))) as _
    }
}

impl<F, Upd> Guard<DispatcherContext<Upd>> for GuardFnWrapper<F, ((),)>
where
    Upd: 'static,
    F: for<'a> AsyncBorrowSendFn<'a, DispatcherContext<Upd>, Out = bool>,
{
    fn check<'a>(&self, cx: &'a DispatcherContext<Upd>) -> BoxFuture<'a, bool> {
        Box::pin((self.0)(&cx))
    }
}

impl<F, Upd> Guard<DispatcherContext<Upd>> for GuardFnWrapper<F, (bool, ())>
where
    F: Fn(&DispatcherContext<Upd>) -> bool,
{
    fn check<'a>(&self, cx: &'a DispatcherContext<Upd>) -> BoxFuture<'a, bool> {
        Box::pin(futures::future::ready((self.0)(&cx))) as _
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

impl<F, Upd> IntoGuard<Upd, GuardFnWrapper<F, ()>> for F
where
    Upd: ?Sized + 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn into_guard(self) -> GuardFnWrapper<F, ()> {
        GuardFnWrapper::new(self)
    }
}

impl<F, Upd> IntoGuard<Upd, GuardFnWrapper<F, bool>> for F
where
    Upd: ?Sized,
    F: Fn(&Upd) -> bool,
{
    fn into_guard(self) -> GuardFnWrapper<F, bool> {
        GuardFnWrapper::new(self)
    }
}

impl<F, Upd> IntoGuard<DispatcherContext<Upd>, GuardFnWrapper<F, ((), ())>> for F
where
    Upd: 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    fn into_guard(self) -> GuardFnWrapper<F, ((), ())> {
        GuardFnWrapper::new(self)
    }
}

impl<F, Upd> IntoGuard<DispatcherContext<Upd>, GuardFnWrapper<F, (bool, (), ())>> for F
where
    F: Fn(&Upd) -> bool,
{
    fn into_guard(self) -> GuardFnWrapper<F, (bool, (), ())> {
        GuardFnWrapper::new(self)
    }
}

impl<F, Upd> IntoGuard<DispatcherContext<Upd>, GuardFnWrapper<F, ((),)>> for F
where
    Upd: 'static,
    F: for<'a> AsyncBorrowSendFn<'a, DispatcherContext<Upd>, Out = bool>,
{
    fn into_guard(self) -> GuardFnWrapper<F, ((),)> {
        GuardFnWrapper::new(self)
    }
}

impl<F, Upd> IntoGuard<DispatcherContext<Upd>, GuardFnWrapper<F, (bool, ())>> for F
where
    F: Fn(&DispatcherContext<Upd>) -> bool,
{
    fn into_guard(self) -> GuardFnWrapper<F, (bool, ())> {
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

    pub fn add<T>(mut self, data: T) -> Self
    where
        T: Guard<Upd> + Send + Sync + 'static,
    {
        self.guards.push(Box::new(data));
        self
    }

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

    pub fn with(mut self, other: Self) -> Self {
        self.guards.extend(other.guards.into_iter());
        self
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
