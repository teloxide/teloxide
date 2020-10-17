use std::future::Future;

use crate::requests::{HasPayload, Output};

/// A ready-to-send telegram request.
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
/// `B::send_ref` while _not_ meaning to really send the request right now.
#[cfg_attr(all(docsrs, feature = "nightly"), doc(spotlight))]
pub trait Request: HasPayload {
    /*
     * Could be mostly `core::future::IntoFuture` though there is no reason to
     * use it before it's integrated in async/await
     */

    /// Type of error that may happen during sending the request to telegram.
    type Err: std::error::Error;

    /// Type of future returned by [`send`](Request::send) method.
    type Send: Future<Output = Result<Output<Self>, Self::Err>>;

    /// Type of future returned by [`send_ref`](Request::send_ref) method.
    ///
    /// NOTE: it intentionally forbids borrowing from self
    // though anyway we couldn't allow borrowing without GATs :sob:
    type SendRef: Future<Output = Result<Output<Self>, Self::Err>>;

    /// Send the request.
    ///
    /// ## Examples
    // FIXME(waffle): ignored until full request redesign lands
    /// ```ignore
    /// # async {
    /// use teloxide_core::{methods::GetMe, requests::{Request, RequestJson}, types::User, bot::Bot};
    ///
    /// let bot = Bot::new("TOKEN");
    /// let method = GetMe::new();
    /// let request = JsonRequest::new(bot, method);
    /// let _: User = request.send().await.unwrap();
    /// # }
    /// ```
    fn send(self) -> Self::Send;

    /// Send the request.
    ///
    /// This method is analogous to [`send`](Request::send), but it doesn't take
    /// the ownership of `self`. This allows to send the same (or slightly
    /// different) requests over and over.
    ///
    /// _Also_ it is expected that calling this method is better than just
    /// `clone`ing the requests. (because instead of copying all the data
    /// and then serializing it, this method should just serialize the data)
    ///
    /// ## Examples
    // FIXME(waffle): ignored until full request redesign lands
    /// ```ignore
    /// # async {
    /// use teloxide_core::prelude::*;
    ///
    /// let bot = Bot::new("TOKEN");
    /// # let chat_ids = vec![1, 2, 3, 4].into_iter().map(Into::into);
    ///
    /// let mut req = bot.send_message(0, "Hi there!");
    /// for chat_id in chat_ids {
    ///     req.chat_id = chat_id;
    ///     req.send_ref().await.unwrap();
    /// }
    /// # }
    /// ```
    fn send_ref(&self) -> Self::SendRef;
}
