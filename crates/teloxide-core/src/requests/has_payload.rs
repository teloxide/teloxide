use either::Either;

use crate::requests::Payload;

/// Represents types having payload inside.
///
/// This trait is something between [`DerefMut`] and [`BorrowMut`] — it allows
/// only one implementation per type (the [output type] is associated, not
/// generic), has implementations for all types `P` such `P: `[`Payload`], but
/// has no magic compiler support like [`DerefMut`] does nor does it require
/// any laws about `Eq`, `Ord` and `Hash` as [`BorrowMut`] does.
///
/// Also the [output type] is bounded by the [`Payload`] trait.
///
/// This trait is mostly used to implement payload setters (on both payloads &
/// requests), so you probably won't find yourself using it directly.
///
/// [`DerefMut`]: std::ops::DerefMut
/// [`BorrowMut`]: std::borrow::BorrowMut
/// [`Payload`]: crate::requests::Payload
/// [output type]: HasPayload::Payload
pub trait HasPayload {
    /// The type of the payload contained.
    type Payload: Payload;

    /// Gain mutable access to the underlying payload.
    fn payload_mut(&mut self) -> &mut Self::Payload;

    /// Gain immutable access to the underlying payload.
    fn payload_ref(&self) -> &Self::Payload;

    /// Update payload with a function
    fn with_payload_mut<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self::Payload),
    {
        f(self.payload_mut());
        self
    }
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

impl<L, R> HasPayload for Either<L, R>
where
    L: HasPayload,
    R: HasPayload<Payload = L::Payload>,
{
    type Payload = L::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.as_mut().either(<_>::payload_mut, <_>::payload_mut)
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.as_ref().either(<_>::payload_ref, <_>::payload_ref)
    }
}
