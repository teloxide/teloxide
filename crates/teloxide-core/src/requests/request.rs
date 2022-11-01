use std::future::{Future, IntoFuture};

// use either::Either;
// use futures::future;

use crate::requests::{HasPayload, Output};

/// A ready-to-send Telegram request.
// FIXME(waffle): Write better doc for the trait
///
/// ## Implementation notes
///
/// It is not recommended to do any kind of _work_ in `send` or `send_ref`.
/// Instead it's recommended to do all the (possible) stuff in the returned
/// future. In other words — keep it lazy.
///
/// This is crucial for request wrappers which may want to cancel and/or never
/// send the underlying request. E.g.: [`Throttle<B>`]'s `send_ref` calls
/// `B::send_ref` while _not_ meaning to really send the request at the moment.
///
/// [`Throttle<B>`]: crate::adaptors::Throttle
#[cfg_attr(all(any(docsrs, dep_docsrs), feature = "nightly"), doc(notable_trait))]
pub trait Request
where
    Self: HasPayload,
    Self: IntoFuture<Output = Result<Output<Self>, Self::Err>, IntoFuture = Self::Send>,
{
    /// The type of an error that may happen while sending a request to
    /// Telegram.
    type Err: std::error::Error + Send;

    /// The type of the future returned by the [`send`](Request::send) method.
    type Send: Future<Output = Result<Output<Self>, Self::Err>> + Send;

    /// A type of the future returned by the [`send_ref`](Request::send_ref)
    /// method.
    // Note: it intentionally forbids borrowing from `self` though we couldn't allow
    // borrowing without GATs anyway.
    type SendRef: Future<Output = Result<Output<Self>, Self::Err>> + Send;

    /// Send this request.
    ///
    /// ## Examples
    ///
    /// ```
    /// # async {
    /// use teloxide_core::{
    ///     payloads::GetMe,
    ///     requests::{JsonRequest, Request},
    ///     types::Me,
    ///     Bot,
    /// };
    ///
    /// let bot = Bot::new("TOKEN");
    ///
    /// // Note: it's recommended to `Requester` instead of creating requests directly
    /// let method = GetMe::new();
    /// let request = JsonRequest::new(bot, method);
    /// let request_clone = request.clone();
    /// let _: Me = request.send().await.unwrap();
    ///
    /// // You can also just await requests, without calling `send`:
    /// let _: Me = request_clone.await.unwrap();
    /// # };
    /// ```
    #[must_use = "Futures are lazy and do nothing unless polled or awaited"]
    fn send(self) -> Self::Send;

    /// Send this request by reference.
    ///
    /// This method is analogous to [`send`](Request::send), but it doesn't take
    /// the ownership of `self`. This allows to send the same (or slightly
    /// different) requests over and over.
    ///
    /// Also, it is expected that calling this method is better than just
    /// cloning requests. (Because instead of copying all the data
    /// and then serializing it, this method should just serialize the data.)
    ///
    /// ## Examples
    ///
    /// ```
    /// # async {
    /// use teloxide_core::{prelude::*, requests::Request, types::ChatId, Bot};
    ///
    /// let bot = Bot::new("TOKEN");
    /// # let chat_ids = vec![1i64, 2, 3, 4].into_iter().map(ChatId).map(Into::into).collect::<Vec<_>>();
    ///
    /// let mut req = bot.send_message(ChatId(0xAAAAAAAA), "Hi there!");
    /// for chat_id in chat_ids {
    ///     req.chat_id = chat_id;
    ///     req.send_ref().await.unwrap();
    /// }
    /// # };
    /// ```
    #[must_use = "Futures are lazy and do nothing unless polled or awaited"]
    fn send_ref(&self) -> Self::SendRef;

    #[cfg(feature = "erased")]
    fn erase<'a>(self) -> crate::adaptors::erased::ErasedRequest<'a, Self::Payload, Self::Err>
    where
        Self: Sized + 'a,
    {
        crate::adaptors::erased::ErasedRequest::erase(self)
    }
}

// FIXME: re-introduce `Either` impls once `Either: IntoFuture` (or make out own
// `Either`) (same for `Requester`)

// impl<L, R> Request for Either<L, R>
// where
//     L: Request,
//     R: Request<Payload = L::Payload, Err = L::Err>,
// {
//     type Err = L::Err;

//     type Send = future::Either<L::Send, R::Send>;

//     type SendRef = future::Either<L::SendRef, R::SendRef>;

//     fn send(self) -> Self::Send {
//         self.map_left(<_>::send)
//             .map_right(<_>::send)
//             .either(future::Either::Left, future::Either::Right)
//     }

//     fn send_ref(&self) -> Self::SendRef {
//         self.as_ref()
//             .map_left(<_>::send_ref)
//             .map_right(<_>::send_ref)
//             .either(future::Either::Left, future::Either::Right)
//     }
// }
