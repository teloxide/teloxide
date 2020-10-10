use crate::contrib::views::view::View;
use std::marker::PhantomData;

/// ViewFactory is used to construct an `View`s, that will be sent to telegram. It is need in
/// first order to add possibility of generate code using `proc-macro`.
pub trait ViewFactory {
    /// What you need to generate a view. It may be, for example, name of city to show the weather.
    /// For static buttons it may be `()`.
    type Ctx;
    /// The view that will produced by `construct` method, and which can produce an update.
    type View: View;
    fn construct(&self, ctx: Self::Ctx) -> Self::View;
}

/// Wrapper for `Fn(Ctx) -> View`. Needed for bypass `Rust` compiler restrictions on implements traits.
pub struct ViewFactoryWrapper<F, Ctx, V>
    where
        V: View,
        F: Fn(Ctx) -> V 
{
    f: F,
    phantom1: PhantomData<Ctx>,
    phantom2: PhantomData<V>,
}
impl<F, Ctx, V> From<F> for ViewFactoryWrapper<F, Ctx, V>
    where
        V: View,
        F: Fn(Ctx) -> V
{
    fn from(f: F) -> Self {
        Self { 
            f,
            phantom1: PhantomData,
            phantom2: PhantomData
        }
    }
}

impl<Ctx, V, F> ViewFactory for ViewFactoryWrapper<F, Ctx, V>
    where
        V: View,
        F: Fn(Ctx) -> V,
{
    type Ctx = Ctx;
    type View = V;

    fn construct(&self, ctx: Self::Ctx) -> Self::View {
        (self.f)(ctx)
    }
}
