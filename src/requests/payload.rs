use std::time::Duration;

/// Payload of a request.
///
/// Simply speaking, structures implementing this trait represent arguments of
/// a Telegram bot API method.
///
/// Also, this trait provides some additional information needed to send a
/// request to Telegram.
#[cfg_attr(all(any(docsrs, dep_docsrs), feature = "nightly"), doc(notable_trait))]
pub trait Payload {
    /// The return type of a Telegram method.
    ///
    /// Note: it should not include `Result` wrappers (e.g. it should be simply
    /// [`Message`], [`True`] or something else).
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    type Output;

    /// Name of a Telegram method.
    ///
    /// It is case insensitive, though must not include underscores. (e.g.
    /// `GetMe`, `GETME`, `getme`, `getMe` are ok, but `get_me` is not ok).
    const NAME: &'static str;

    /// If this payload may take long time to execute (e.g. [`GetUpdates`] with
    /// big `timeout`), the **minimum** timeout that should be used.
    ///
    /// [`GetUpdates`]: crate::payloads::GetUpdates
    fn timeout_hint(&self) -> Option<Duration> {
        None
    }
}
