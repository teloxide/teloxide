use crate::dispatching::core::Context;
use futures::{future::BoxFuture, FutureExt};
use std::{future::Future, marker::PhantomData};

/// The trait is used to simulate GAT for the functions that returns [`Future`]. It used for the
/// [`Guard`] impls.
///
/// When GAT will stabilized we remove this trait and add `type Fut<'a>: Future + 'a` to the [`Guard`].
///
/// [`Future`]: std::future::Future
/// [`Guard`]: TODO
pub trait AsyncBorrowSendFn<'a, T>
where
    T: ?Sized + 'static,
    Self: Fn(&'a T) -> <Self as AsyncBorrowSendFn<'a, T>>::Fut,
{
    /// Future output.
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

/// The trait is used to recognize when we want to handle the update. It returns the `BoxFuture` so
/// you can call async functions in it. For example you can ask your database or something similar.
///
/// Guards **must not** mutate the foreign state. In that case behaviour is unspecified.
pub trait Guard<Upd: ?Sized> {
    fn check<'a>(&self, update: &'a Upd) -> BoxFuture<'a, bool>;
}

/// The struct is used to wrap the functions that is used as an guard.
///
/// In the `Infer` generic you must point the unique type that will be inferred for the unique function.
pub struct GuardFnWrapper<F, Infer>(F, PhantomData<tokio::sync::Mutex<Infer>>);

impl<F, Infer> GuardFnWrapper<F, Infer> {
    pub fn new(func: F) -> Self {
        GuardFnWrapper(func, PhantomData)
    }
}

// These structs are public only to participate in type inference.
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

/// The trait is used in the `with_*` functions to convert the different types of functions into
/// `GuardFnWrapper` to simulate specialization.
///
/// `IntoGuard` **must** be unique for concrete type otherwise we get type inference error in `with_*` functions.
///
/// If you create your own [`Guard`], you must implement `IntoGuard<Upd, Self> for YourGuard`.
///
/// [`Guard`]: TODO
pub trait IntoGuard<Upd: ?Sized, T: Guard<Upd>> {
    fn into_guard(self) -> T;
}

impl<F, T, Upd> IntoGuard<Upd, GuardFnWrapper<F, T>> for GuardFnWrapper<F, T>
where
    Upd: ?Sized,
    GuardFnWrapper<F, T>: Guard<Upd>,
{
    #[inline]
    fn into_guard(self) -> GuardFnWrapper<F, T> {
        self
    }
}

impl<F, Upd> IntoGuard<Upd, GuardFnWrapper<F, GiveSomeReturnFuture>> for F
where
    Upd: ?Sized + 'static,
    F: for<'a> AsyncBorrowSendFn<'a, Upd, Out = bool>,
{
    #[inline]
    fn into_guard(self) -> GuardFnWrapper<F, GiveSomeReturnFuture> {
        GuardFnWrapper::new(self)
    }
}

impl<F, Upd> IntoGuard<Upd, GuardFnWrapper<F, GiveSomeReturnBool>> for F
where
    Upd: ?Sized,
    F: Fn(&Upd) -> bool,
{
    #[inline]
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
    #[inline]
    fn into_guard(self) -> GuardFnWrapper<F, GiveCtxToUpdReturnFuture> {
        GuardFnWrapper::new(self)
    }
}

impl<Ctx, F, Upd> IntoGuard<Ctx, GuardFnWrapper<F, GiveCtxToUpdReturnBool>> for F
where
    Ctx: Context<Upd = Upd>,
    F: Fn(&Upd) -> bool,
{
    #[inline]
    fn into_guard(self) -> GuardFnWrapper<F, GiveCtxToUpdReturnBool> {
        GuardFnWrapper::new(self)
    }
}

impl<Upd> Guard<Upd> for Box<dyn Guard<Upd> + Send + Sync> {
    fn check<'a>(&self, update: &'a Upd) -> BoxFuture<'a, bool> {
        (**self).check(update)
    }
}

/// The struct contains list of the guards and return true as guard only when containing guards return true.
///
/// The execution queue is not determined.
pub struct Guards<Upd> {
    guards: Vec<Box<dyn Guard<Upd> + Send + Sync>>,
}

impl<Upd> Guards<Upd> {
    pub fn new() -> Self {
        Guards { guards: Vec::new() }
    }

    /// Add the guard to the list of guards.
    pub fn add_guard<T>(&mut self, data: T)
    where
        T: Guard<Upd> + Send + Sync + 'static,
    {
        self.add_boxed_guard(Box::new(data));
    }

    /// Add the `Box<dyn Guard<Upd> + Send + Sync>` to the list of the guards.
    pub fn add_boxed_guard(&mut self, data: Box<dyn Guard<Upd> + Send + Sync>) {
        self.guards.push(data);
    }

    /// It will return `true` only when all the containing guards return `true`.
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

/// The struct return `true` as guard when at least one guard return `true`.
///
/// The execution queue is not determined.
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
