use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::Update,
};
use crate::types::AllowedUpdate;

/// Use this method to receive incoming updates using long polling ([wiki]).
/// An array ([`Vec`]) of [`Update`]s is returned.
///
/// **Notes:**
/// 1. This method will not work if an outgoing webhook is set up.
/// 2. In order to avoid getting duplicate updates,
///    recalculate offset after each server response.
///
/// [wiki]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
/// [Update]: crate::types::Update
/// [Vec]: std::alloc::Vec
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize, Default)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one
    /// than the highest among the identifiers of previously received updates.
    /// By default, updates starting with the earliest unconfirmed update are
    /// returned. An update is considered confirmed as soon as [`GetUpdates`]
    /// is called with an [`offset`] higher than its [`id`]. The negative
    /// offset can be specified to retrieve updates starting from `-offset`
    /// update from the end of the updates queue. All previous updates will
    /// forgotten.
    ///
    /// [`GetUpdates`]: self::GetUpdates
    /// [`offset`]: self::GetUpdates::offset
    /// [`id`]: crate::types::Update::id
    pub offset: Option<i32>,
    /// Limits the number of updates to be retrieved.
    /// Values between `1`—`100` are accepted. Defaults to `100`.
    pub limit: Option<u8>,
    /// Timeout in seconds for long polling. Defaults to `0`,
    /// i.e. usual short polling. Should be positive, short polling should be
    /// used for testing purposes only.
    pub timeout: Option<u32>,
    /// List the types of updates you want your bot to receive.
    /// For example, specify [[`Message`], [`EditedChannelPost`],
    /// [`CallbackQuery`]] to only receive updates of these types.
    /// See [`AllowedUpdate`] for a complete list of available update types.
    ///
    /// Specify an empty list to receive all updates regardless of type
    /// (default). If not specified, the previous setting will be used.
    ///
    /// **Note:**
    /// This parameter doesn't affect updates created before the call to the
    /// [`GetUpdates`], so unwanted updates may be received for a short period
    /// of time.
    ///
    /// [`Message`]: self::AllowedUpdate::Message
    /// [`EditedChannelPost`]: self::AllowedUpdate::EditedChannelPost
    /// [`CallbackQuery`]: self::AllowedUpdate::CallbackQuery
    /// [`AllowedUpdate`]: self::AllowedUpdate
    /// [`GetUpdates`]: self::GetUpdates
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl Method for GetUpdates {
    type Output = Vec<Update>;

    const NAME: &'static str = "getUpdates";
}

impl json::Payload for GetUpdates {}

impl dynamic::Payload for GetUpdates {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetUpdates {
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
}

impl json::Request<'_, GetUpdates> {
    pub fn offset(mut self, value: i32) -> Self {
        self.payload.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: u8) -> Self {
        self.payload.limit = Some(value);
        self
    }

    pub fn timeout(mut self, value: u32) -> Self {
        self.payload.timeout = Some(value);
        self
    }

    pub fn allowed_updates<T>(mut self, value: T) -> Self
    where
        T: Into<Vec<AllowedUpdate>>, // TODO: into or other trait?
    {
        self.payload.allowed_updates = Some(value.into());
        self
    }
}
