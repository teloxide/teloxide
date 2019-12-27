use reqwest::StatusCode;

//<editor-fold desc="download">
#[derive(Debug, Error, From)]
pub enum DownloadError {
    #[error("A network error: {0}")]
    NetworkError(#[source] reqwest::Error),

    #[error("An I/O error: {0}")]
    Io(#[source] std::io::Error),
}

//</editor-fold>

//<editor-fold desc="request">
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("A Telegram's error #{status_code}: {description}")]
    ApiError {
        status_code: StatusCode,
        description: String,
    },

    /// The group has been migrated to a supergroup with the specified
    /// identifier.
    #[error("The group has been migrated to a supergroup with ID #{0}")]
    MigrateToChatId(i64),

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated
    #[error("Retry after {0} seconds")]
    RetryAfter(i32),

    #[error("A network error: {0}")]
    NetworkError(#[source] reqwest::Error),

    #[error("An error while parsing JSON: {0}")]
    InvalidJson(#[source] serde_json::Error),
}

//</editor-fold>

#[derive(Debug, Deserialize, PartialEq, Copy, Hash, Eq)]
enum ApiErrorKind {
    /// Occurs when the bot has been blocked by the user.
    #[serde(rename = "Forbidden: bot was blocked by the user")]
    BotBlocked,

    /// Occurs when bot trying to modify a message without modification content
    #[serde(rename = "Bad Request: message is not modified: specified new message content and reply markup are exactly the same as a current content and reply markup of the message")]
    MessageNotModified,

    /// Occurs when bot trying to forward or delete a message which was deleted
    #[serde(rename = "Bad Request: MESSAGE_ID_INVALID")]
    MessageIdInvalid,

    /// Occurs when bot trying to forward a message which does not exists
    #[serde(rename = "Bad Request: message to forward not found")]
    MessageToForwardNotFound,

    /// Occurs when bot trying to delete a message which does not exists
    #[serde(rename = "Bad Request: message to delete not found")]
    MessageToDeleteNotFound,

    /// Occurs when bot trying to send a text message without text
    #[serde(rename = "Bad Request: message text is empty")]
    MessageTextIsEmpty,

    MessageCantBeEdited, //?

    MessageCantBeDeleted, //?

    /// Occurs when bot trying to edit a message which does not exists
    #[serde(rename = "Bad Request: message to edit not found")]
    MessageToEditNotFound,

    /// Occurs when bot trying to reply to a message which does not exists
    #[serde(rename = "Bad Request: reply message not found")]
    MessageToReplyNotFound,

    ToMuchMessages, //?

    PollCantBeStopped, //?

    /// Occurs when bot trying to stop poll that has already been stopped
    #[serde(rename = "Bad Request: poll has already been closed")]
    PollHasAlreadyClosed,
}
