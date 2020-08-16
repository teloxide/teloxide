use serde::Serialize;

use crate::{
    net,
    requests::{GetUpdates, Request, ResponseResult},
    types::{AllowedUpdate, NonStrictVec, Update},
    Bot,
};

/// This is non strict version of [`GetUpdates`], this means that if it will
/// fail to deserialize some updates, it won't fail entirely, but will just
/// return some errors.
///
/// Note: this is not a 'real' telegram method, this is simply [`GetUpdates`]
/// with changed return type.
///
/// [`GetUpdates`]: crate::requests::GetUpdates
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct GetUpdatesNonStrict(pub GetUpdates);

#[async_trait::async_trait]
impl Request for GetUpdatesNonStrict {
    type Output = NonStrictVec<Update>;

    async fn send(&self) -> ResponseResult<Self::Output> {
        net::request_json(self.0.bot.client(), self.0.bot.token(), "getUpdates", &self).await
    }
}

impl GetUpdatesNonStrict {
    pub(crate) fn new(bot: Bot) -> Self {
        Self(GetUpdates::new(bot))
    }

    /// Identifier of the first update to be returned.
    ///
    /// Must be greater by one than the highest among the identifiers of
    /// previously received updates. By default, updates starting with the
    /// earliest unconfirmed update are returned. An update is considered
    /// confirmed as soon as [`GetUpdates`] is called with an [`offset`]
    /// higher than its [`id`]. The negative offset can be specified to
    /// retrieve updates starting from `-offset` update from the end of the
    /// updates queue. All previous updates will forgotten.
    ///
    /// [`GetUpdates`]: self::GetUpdates
    /// [`offset`]: self::GetUpdates::offset
    /// [`id`]: crate::types::Update::id
    pub fn offset(mut self, value: i32) -> Self {
        self.0.offset = Some(value);
        self
    }

    /// Limits the number of updates to be retrieved.
    ///
    /// Values between `1`—`100` are accepted. Defaults to `100`.
    pub fn limit(mut self, value: u8) -> Self {
        self.0.limit = Some(value);
        self
    }

    /// Timeout in seconds for long polling.
    ///
    /// Defaults to `0`, i.e. usual short polling. Should be positive, short
    /// polling should be used for testing purposes only.
    pub fn timeout(mut self, value: u32) -> Self {
        self.0.timeout = Some(value);
        self
    }

    /// List the types of updates you want your bot to receive.
    ///
    /// For example, specify [[`Message`], [`EditedChannelPost`],
    /// [`CallbackQuery`]] to only receive updates of these types.
    /// See [`AllowedUpdate`] for a complete list of available update types.
    ///
    /// Specify an empty list to receive all updates regardless of type
    /// (default). If not specified, the previous setting will be used.
    ///
    /// **Note:**
    /// This parameter doesn't affect updates created before the call to the
    /// [`Bot::get_updates`], so unwanted updates may be received for a short
    /// period of time.
    ///
    /// [`Message`]: self::AllowedUpdate::Message
    /// [`EditedChannelPost`]: self::AllowedUpdate::EditedChannelPost
    /// [`CallbackQuery`]: self::AllowedUpdate::CallbackQuery
    /// [`AllowedUpdate`]: self::AllowedUpdate
    /// [`Bot::get_updates`]: crate::Bot::get_updates
    pub fn allowed_updates<T>(mut self, value: T) -> Self
    where
        T: Into<Vec<AllowedUpdate>>,
    {
        self.0.allowed_updates = Some(value.into());
        self
    }
}
