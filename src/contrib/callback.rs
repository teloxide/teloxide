use core::marker::PhantomData;

use crate::{
    contrib::{handler::Handler, parser::Parser},
    dispatching::UpdateWithCx,
};

use teloc::{
    container::Container, get_dependencies::GetDependencies, reexport::frunk, Resolver, Scope,
    ServiceProvider,
};
#[cfg(feature = "macros")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
pub use teloxide_macros::Callback;

#[async_trait::async_trait]
pub trait Callback {
    type Update;
    type Err;
    async fn try_handle(
        &self,
        input: UpdateWithCx<Self::Update>,
    ) -> Result<Result<(), Self::Err>, UpdateWithCx<Self::Update>>;
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

    async fn try_handle(
        &self,
        input: UpdateWithCx<<Self as Parser>::Update>,
    ) -> Result<Result<(), Self::Err>, UpdateWithCx<<Self as Parser>::Update>> {
        match self.parse(input) {
            Ok(d) => Ok(self.handle(d).await),
            Err(e) => Err(e),
        }
    }
}

pub struct Alternative<C1, C2> {
    left: C1,
    right: C2,
}

impl<C1, C2> Alternative<C1, C2> {
    pub fn new(left: C1, right: C2) -> Self {
        Alternative { left, right }
    }
}

pub struct AlternativeContainer<C1, C2>(PhantomData<(C1, C2)>);
impl<C1, C2> Container<Alternative<C1, C2>> for AlternativeContainer<C1, C2> {}
impl<'b, C1, C2, DepsElems, Indexes, D, S, SI>
    Resolver<
        'b,
        AlternativeContainer<C1, C2>,
        Alternative<C1, C2>,
        Self,
        (DepsElems, Indexes, D, S, SI),
    > for ServiceProvider<D, S, SI>
where
    C1: 'b,
    C2: 'b,
    Self: GetDependencies<'b, frunk::Hlist![C1, C2], DepsElems, Indexes>,
{
    fn resolve(&'b self) -> Alternative<C1, C2> {
        let (c1, c2) = self.get_deps().into_tuple2();
        Alternative::new(c1, c2)
    }
}

impl<'b, C1, C2, DepsElems, Indexes, SP, S, SI>
    Resolver<
        'b,
        AlternativeContainer<C1, C2>,
        Alternative<C1, C2>,
        Self,
        (SP, S, SI, DepsElems, Indexes),
    > for Scope<'_, SP, S, SI>
where
    C1: 'b,
    C2: 'b,
    Self: GetDependencies<'b, frunk::Hlist![C1, C2], DepsElems, Indexes>,
{
    fn resolve(&'b self) -> Alternative<C1, C2> {
        let (c1, c2) = self.get_deps().into_tuple2();
        Alternative::new(c1, c2)
    }
}

#[async_trait::async_trait]
impl<C1, C2> Callback for Alternative<C1, C2>
where
    C1: Callback + Send + Sync,
    C2: Callback<Update = C1::Update, Err = C1::Err> + Send + Sync,
    <C1 as Callback>::Update: Send,
    <C1 as Callback>::Err: Send,
{
    type Update = C1::Update;
    type Err = C1::Err;

    async fn try_handle(
        &self,
        input: UpdateWithCx<Self::Update>,
    ) -> Result<Result<(), Self::Err>, UpdateWithCx<Self::Update>> {
        match self.left.try_handle(input).await {
            Ok(res) => Ok(res),
            Err(input) => self.right.try_handle(input).await,
        }
    }
}

#[macro_export]
macro_rules! Cascade {
    ($left:ty, $($rest:ty),+) => {
        teloxide::contrib::callback::Alternative<$left, $crate::Cascade![$($rest),+]>
    };
    ($left:ty) => {
        $left
    };
}
