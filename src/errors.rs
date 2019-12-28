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

    /// Occurs when bot tries to modify a message without modification content
    #[serde(rename = "Bad Request: message is not modified: specified new message content and reply markup are exactly the same as a current content and reply markup of the message")]
    MessageNotModified,

    /// Occurs when bot tries to forward or delete a message which was deleted
    #[serde(rename = "Bad Request: MESSAGE_ID_INVALID")]
    MessageIdInvalid,

    /// Occurs when bot tries to forward a message which does not exists
    #[serde(rename = "Bad Request: message to forward not found")]
    MessageToForwardNotFound,

    /// Occurs when bot tries to delete a message which does not exists
    #[serde(rename = "Bad Request: message to delete not found")]
    MessageToDeleteNotFound,

    /// Occurs when bot tries to send a text message without text
    #[serde(rename = "Bad Request: message text is empty")]
    MessageTextIsEmpty,

    /// Occurs when bot tries to edit the message after long time
    #[serde(rename = "Bad Request: message can't be edited")]
    MessageCantBeEdited,

    #[serde(rename = "Bad Request: message can't be deleted")]
    MessageCantBeDeleted, // TODO: docs

    /// Occurs when bot tries to edit a message which does not exists
    #[serde(rename = "Bad Request: message to edit not found")]
    MessageToEditNotFound,

    /// Occurs when bot tries to reply to a message which does not exists
    #[serde(rename = "Bad Request: reply message not found")]
    MessageToReplyNotFound,

    #[serde(rename = "Bad Request: message identifier is not specified")]
    MessageIdentifierNotSpecified, // TODO: docs

    /// Occurs when bot tries to send a message with text size greater then 4096 symbols
    #[serde(rename = "Bad Request: message is too long")]
    MessageIsTooLong,

    /// Occurs when bot tries to send media group with more than 10 items
    #[serde(rename = "Bad Request: Too much messages to send as an album")]
    ToMuchMessages,

    #[serde(rename = "Bad Request: poll can't be stopped")]
    PollCantBeStopped, // TODO: docs

    /// Occurs when bot tries to stop poll that has already been stopped
    #[serde(rename = "Bad Request: poll has already been closed")]
    PollHasAlreadyClosed,

    /// Occurs when bot tries to send poll with less than 2 options
    #[serde(rename = "Bad Request: poll must have at least 2 option")]
    PollMustHaveMoreOptions,

    /// Occurs when bot tries to send poll with more than 10 options
    #[serde(rename = "Bad Request: poll can't have more than 10 options")]
    PollCantHaveMoreOptions,

    /// Occurs when bot tries to send poll with empty option (without text)
    #[serde(rename = "Bad Request: poll options must be non-empty")]
    PollOptionsMustBeNonEmpty,

    /// Occurs when bot tries to send poll with empty question (without text)
    #[serde(rename = "Bad Request: poll question must be non-empty")]
    PollQuestionMustBeNonEmpty,

    /// Occurs when bot tries to send poll with total size of options more than 100 symbols
    #[serde(rename = "Bad Request: poll options length must not exceed 100")]
    PollOptionsLengthTooLong,

    /// Occurs when bot tries to send poll with question size more than 255 symbols
    #[serde(rename = "Bad Request: poll question length must not exceed 255")]
    PollQuestionLengthTooLong,

    /// Occurs when bot tries to stop poll with message without poll
    #[serde(rename = "Bad Request: message with poll to stop not found")]
    MessageWithPollNotFound,

    /// Occurs when bot tries to stop poll with message without poll
    #[serde(rename = "Bad Request: message is not a poll")]
    MessageIsNotAPoll,

    /// Occurs when bot tries to send a message to chat in which it is not a member
    #[serde(rename = "Bad Request: chat not found")]
    ChatNotFound,

    #[serde(rename = "Bad Request: user_id_invalid")]
    InvalidUserId, // TODO: docs

    /// Occurs when bot tries to send SetChatDescription with same text as in the current description
    #[serde(rename = "Bad Request: chat description is not modified")]
    ChatDescriptionIsNotModified,

    /// Occurs when bot tries to answer to query after timeout expire (AnswerCallbackQuery)
    #[serde(rename = "Bad Request: query is too old and response timeout expired or query id is invalid")]
    InvalidQueryID,

    #[serde(rename = "Bad Request: PEER_ID_INVALID")]
    InvalidPeerID, // TODO: docs

    #[serde(rename = "Bad Request: Failed to get HTTP URL content")]
    InvalidHTTPUrlContent, // TODO: docs

    #[serde(rename = "Bad Request: Button URL invalid")]
    ButtonURLInvalid, // TODO: docs

    #[serde(rename = "Bad Request: URL host is empty")]
    URLHostIsEmpty, // TODO: docs

    #[serde(rename = "Bad Request: START_PARAM_INVALID")]
    StartParamInvalid, // TODO: docs

    #[serde(rename = "Bad Request: BUTTON_DATA_INVALID")]
    ButtonDataInvalid, // TODO: docs

    #[serde(rename = "Bad Request: wrong file identifier/HTTP URL specified")]
    WrongFileIdentifier, // TODO: docs

    #[serde(rename = "Bad Request: group is deactivated")]
    GroupDeactivated, // TODO: docs

    /// Occurs when bot tries to set chat photo from file ID (SetChatPhoto)
    #[serde(rename = "Bad Request: Photo should be uploaded as an InputFile")]
    PhotoAsInputFileRequired,

    #[serde(rename = "Bad Request: STICKERSET_INVALID")]
    InvalidStickersSet, // TODO: docs

    #[serde(rename = "Bad Request: there is no sticker in the request")]
    NoStickerInRequest, // TODO: docs

    #[serde(rename = "Bad Request: Admin permissions is required!")]
    ChatAdminRequired, // TODO: docs

    #[serde(rename = "Bad Request: need administrator rights in the channel chat")]
    NeedAdministratorRightsInTheChannel, // TODO: docs

    #[serde(rename = "Bad Request: not enough rights to pin a message")]
    NotEnoughRightsToPinMessage, // TODO: docs

    #[serde(rename = "Bad Request: method is available only for supergroups and channel")]
    MethodNotAvailableInPrivateChats, // TODO: docs

    #[serde(rename = "Bad Request: can't demote chat creator")]
    CantDemoteChatCreator, // TODO: docs

    #[serde(rename = "Bad Request: can't restrict self")]
    CantRestrictSelf, // TODO: docs

    #[serde(rename = "Bad Request: not enough rights to restrict/unrestrict chat member")]
    NotEnoughRightsToRestrict, // TODO: docs

    #[serde(rename = "Bad Request: PHOTO_INVALID_DIMENSIONS")]
    PhotoDimensions, // TODO: docs

    #[serde(rename = "Bad Request: supergroup members are unavailable")]
    UnavailableMembers, // TODO: docs

    #[serde(rename = "Bad Request: type of file mismatch")]
    TypeOfFileMismatch, // TODO: docs

    #[serde(rename = "Bad Request: wrong remote file id specified")]
    WrongRemoteFileIdSpecified, // TODO: docs

    #[serde(rename = "Bad Request: PAYMENT_PROVIDER_INVALID")]
    PaymentProviderInvalid, // TODO: docs

    #[serde(rename = "Bad Request: currency_total_amount_invalid")]
    CurrencyTotalAmountInvalid, // TODO: docs

    #[serde(rename = "Bad Request: HTTPS url must be provided for webhook")]
    WebhookRequireHTTPS, // TODO: docs

    #[serde(rename = "Bad Request: Webhook can be set up only on ports 80, 88, 443 or 8443")]
    BadWebhookPort, // TODO: docs

    #[serde(rename = "Bad Request: getaddrinfo: Temporary failure in name resolution")]
    BadWebhookAddrInfo, // TODO: docs

    #[serde(rename = "Bad Request: failed to resolve host: no address associated with hostname")]
    BadWebhookNoAddressAssociatedWithHostname, // TODO: docs

    #[serde(rename = "Bad Request: can't parse URL")]
    CantParseUrl, // TODO: docs

    #[serde(rename = "Bad Request: unsupported URL protocol")]
    UnsupportedUrlProtocol, // TODO: docs

    #[serde(rename = "Bad Request: can't parse entities")]
    CantParseEntities, // TODO: docs

    #[serde(rename = "Bad Request: result_id_duplicate")]
    ResultIdDuplicate, // TODO: docs

    #[serde(rename = "Bad Request: bot_domain_invalid")]
    BotDomainInvalid, // TODO: docs

    #[serde(rename = "method not found")]
    MethodNotKnown, // TODO: docs

    /// Occurs when bot tries GetUpdate before the timeout.
    /// Make sure that only one Updater is running.
    #[serde(rename = "terminated by other getUpdates request")]
    TerminatedByOtherGetUpdates, // TODO: docs

    #[serde(rename = "can't use getUpdates method while webhook is active")]
    CantGetUpdates, // TODO: docs

    #[serde(rename = "bot was kicked from a chat")]
    BotKicked, // TODO: docs

    #[serde(rename = "user is deactivated")]
    UserDeactivated, // TODO: docs

    #[serde(rename = "bot can't initiate conversation with a user")]
    CantInitiateConversation, // TODO: docs

    #[serde(rename = "bot can't send messages to bots")]
    CantTalkWithBots, // TODO: docs

    #[serde(other)]
    OtherKind(String)
}
