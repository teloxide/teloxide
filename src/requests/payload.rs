/// Payload of a request.
///
/// Simply speaking structs implementing this trait represent arguments of
/// a telegram bot API method.
///
/// This trait provides some additional information needed for sending request
/// to the telegram.
#[cfg_attr(all(docsrs, feature = "nightly"), doc(spotlight))]
pub trait Payload {
    /// Return type of the telegram method.
    ///
    /// Note: that should not include result wrappers (e.g. it should be simply
    /// `Message`, `True` or something else)
    type Output;

    /// Name of the telegram method. Case insensitive, though must not include
    /// underscores. (e.g.: `GetMe`, `GETME`, `getme`, `getMe` are ok, but
    /// `get_me` is not ok)
    const NAME: &'static str;
}
