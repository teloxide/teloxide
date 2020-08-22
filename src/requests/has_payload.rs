use crate::requests::Payload;

/// Represent types that have payload inside it. E.g.: the payload itself or a
/// `Request`.
///
/// This trait is something between [`DerefMut`] and [`BorrowMut`] — it allows
/// only one implementation per type (the [output] is associated type, not a
/// generic), have implementations for all types `P` such `P: `[`Payload`], but
/// have no magic compiler support like [`DerefMut`] does nor does it require
/// any laws about `Eq`, `Ord` and `Hash` as [`BorrowMut`] does.
///
/// Also [output] type is bounded by [`Payload`] trait.
///
/// This trait is mostly used to implement payload setters (on both payloads &
/// requests), so you probably won't find yourself using it directly.
///
/// [`DerefMut`]: std::ops::DerefMut
/// [`BorrowMut`]: std::borrow::BorrowMut
/// [`Payload`]: crate::requests::Payload
/// [output]: HasPayload::Payload
pub trait HasPayload {
    /// Type of the payload contained.
    type Payload: Payload;

    /// Gain mutable access to the underlying payload.
    fn payload_mut(&mut self) -> &mut Self::Payload;

    /// Gain immutable access to the underlying payload.
    fn payload_ref(&self) -> &Self::Payload;
}

impl<P> HasPayload for P
where
    P: Payload,
{
    type Payload = Self;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self
    }

    fn payload_ref(&self) -> &Self::Payload {
        self
    }
}
