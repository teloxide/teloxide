use crate::{adaptors::DefaultParseMode, requests::Requester, types::ParseMode};

#[cfg(feature = "cache_me")]
use crate::adaptors::CacheMe;

#[cfg(feature = "erased")]
use crate::adaptors::ErasedRequester;

#[cfg(feature = "trace_adaptor")]
use crate::adaptors::trace::{Settings, Trace};

#[cfg(feature = "throttle")]
use crate::adaptors::throttle::{Limits, Throttle};

/// Extensions methods for [`Requester`].
pub trait RequesterExt: Requester {
    /// Add `get_me` caching ability, see [`CacheMe`] for more.
    #[cfg(feature = "cache_me")]
    #[must_use]
    fn cache_me(self) -> CacheMe<Self>
    where
        Self: Sized,
    {
        CacheMe::new(self)
    }

    /// Erase requester type.
    #[cfg(feature = "erased")]
    #[must_use]
    fn erase<'a>(self) -> ErasedRequester<'a, Self::Err>
    where
        Self: 'a,
        Self: Sized,
    {
        ErasedRequester::new(self)
    }

    /// Trace requests, see [`Trace`] for more.
    #[cfg(feature = "trace_adaptor")]
    #[must_use]
    fn trace(self, settings: Settings) -> Trace<Self>
    where
        Self: Sized,
    {
        Trace::new(self, settings)
    }

    /// Add throttling ability, see [`Throttle`] for more.
    ///
    /// Note: this spawns the worker, just as [`Throttle::new_spawn`].
    #[cfg(feature = "throttle")]
    #[must_use]
    fn throttle(self, limits: Limits) -> Throttle<Self>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        Self::Err: crate::errors::AsResponseParameters,
        Self::GetChat: Send,
    {
        Throttle::new_spawn(self, limits)
    }

    /// Specifies default [`ParseMode`], which will be used during all calls to:
    ///
    ///  - [`send_message`]
    ///  - [`send_photo`]
    ///  - [`send_video`]
    ///  - [`send_audio`]
    ///  - [`send_document`]
    ///  - [`send_animation`]
    ///  - [`send_voice`]
    ///  - [`send_poll`]
    ///  - [`edit_message_text`] (and [`edit_message_text_inline`])
    ///  - [`edit_message_caption`] (and [`edit_message_caption_inline`])
    ///
    /// [`send_message`]: crate::requests::Requester::send_message
    /// [`send_photo`]: crate::requests::Requester::send_photo
    /// [`send_video`]: crate::requests::Requester::send_video
    /// [`send_audio`]: crate::requests::Requester::send_audio
    /// [`send_document`]: crate::requests::Requester::send_document
    /// [`send_animation`]: crate::requests::Requester::send_animation
    /// [`send_voice`]: crate::requests::Requester::send_voice
    /// [`send_poll`]: crate::requests::Requester::send_poll
    /// [`edit_message_text`]: crate::requests::Requester::edit_message_text
    /// [`edit_message_text_inline`]:
    /// crate::requests::Requester::edit_message_text_inline
    /// [`edit_message_caption`]:
    /// crate::requests::Requester::edit_message_caption
    /// [`edit_message_caption_inline`]:
    /// crate::requests::Requester::edit_message_caption_inline
    #[must_use]
    fn parse_mode(self, parse_mode: ParseMode) -> DefaultParseMode<Self>
    where
        Self: Sized,
    {
        DefaultParseMode::new(self, parse_mode)
    }
}

impl<T> RequesterExt for T
where
    T: Requester,
{
    /* use default impls */
}
