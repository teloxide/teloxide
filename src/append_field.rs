#[cfg(feature = "frunk")]
use frunk::{from_generic, generic::Generic, hlist::HAppender, into_generic};

#[cfg(feature = "frunk")]
pub fn append_field<T1, T2, T1Repr, F>(src: T1, field: F) -> T2
where
    T1: Generic<Repr = T1Repr>,
    T2: Generic<Repr = <T1Repr as HAppender<F>>::Output>,
    T1Repr: HAppender<F>,
{
    from_generic(into_generic(src).append(field))
}
