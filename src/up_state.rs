#![cfg(feature = "frunk")]

use frunk::{from_generic, generic::Generic, hlist::HAppender, into_generic};

/// Constructs a structure from another structure and a field.
///
/// Let `X` be a structure of `field1, ..., fieldN`, `Y` be `field1, ...,
/// fieldN, fieldN+1`. Both `X` and `Y` implement [`Generic`]. Then `Y::up(x,
/// fieldN+1)` constructs `Y` from all the fields of `x: X` plus `Y`'s
/// `fieldN+1`.
///
/// [`Generic`]: https://docs.rs/frunk/latest/frunk/generic/trait.Generic.html
pub trait UpState: Sized {
    fn up<T1, T1Repr, F>(src: T1, field: F) -> Self
    where
        T1: Generic<Repr = T1Repr>,
        Self: Generic<Repr = <T1Repr as HAppender<F>>::Output>,
        T1Repr: HAppender<F>,
    {
        from_generic(into_generic(src).append(field))
    }
}

impl<T2> UpState for T2 {}
