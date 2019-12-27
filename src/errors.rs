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

    /// Occurs when bot trying to edit the message after long time
    #[serde(rename = "Bad Request: message can't be edited")]
    MessageCantBeEdited,

    #[serde(rename = "Bad Request: message can't be deleted")]
    MessageCantBeDeleted, // TODO: docs

    /// Occurs when bot trying to edit a message which does not exists
    #[serde(rename = "Bad Request: message to edit not found")]
    MessageToEditNotFound,

    /// Occurs when bot trying to reply to a message which does not exists
    #[serde(rename = "Bad Request: reply message not found")]
    MessageToReplyNotFound,

    #[serde(rename = "Bad Request: message identifier is not specified")]
    MessageIdentifierNotSpecified, // TODO: docs

    /// Occurs when bot trying to send a message with text size greater then 4096 symbols
    #[serde(rename = "Bad Request: message is too long")]
    MessageIsTooLong,

    /// Occurs when bot trying to send media group with more than 10 items
    #[serde(rename = "Bad Request: Too much messages to send as an album")]
    ToMuchMessages,

    #[serde(rename = "Bad Request: poll can't be stopped")]
    PollCantBeStopped, // TODO: docs

    /// Occurs when bot trying to stop poll that has already been stopped
    #[serde(rename = "Bad Request: poll has already been closed")]
    PollHasAlreadyClosed,

    /// Occurs when bot trying to send poll with less than 2 options
    #[serde(rename = "Bad Request: poll must have at least 2 option")]
    PollMustHaveMoreOptions,

    /// Occurs when bot trying to send poll with more than 10 options
    #[serde(rename = "Bad Request: poll can't have more than 10 options")]
    PollCantHaveMoreOptions,

    /// Occurs when bot trying to send poll with empty option (without text)
    #[serde(rename = "Bad Request: poll options must be non-empty")]
    PollOptionsMustBeNonEmpty,

    /// Occurs when bot trying to send poll with empty question (without text)
    #[serde(rename = "Bad Request: poll question must be non-empty")]
    PollQuestionMustBeNonEmpty,

    /// Occurs when bot trying to send poll with total size of options more than 100 symbols
    #[serde(rename = "Bad Request: poll options length must not exceed 100")]
    PollOptionsLengthTooLong,

    /// Occurs when bot trying to send poll with question size more than 255 symbols
    #[serde(rename = "Bad Request: poll question length must not exceed 255")]
    PollQuestionLengthTooLong,

    /// Occurs when bot trying to stop poll with message without poll
    #[serde(rename = "Bad Request: message with poll to stop not found")]
    MessageWithPollNotFound,

    /// Occurs when bot trying to stop poll with message without poll
    #[serde(rename = "Bad Request: message is not a poll")]
    MessageIsNotAPoll,

    /// Occurs when bot trying to send a message to chat in which it is not a member
    #[serde(rename = "Bad Request: chat not found")]
    ChatNotFound,

    #[serde(rename = "Bad Request: user_id_invalid")]
    InvalidUserId, // TODO: docs
}
