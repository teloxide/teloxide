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

#[derive(Debug, Deserialize, PartialEq, Copy, Hash, Eq, Clone)]
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

    /// Occurs when bot tries to edit a message after long time
    #[serde(rename = "Bad Request: message can't be edited")]
    MessageCantBeEdited,

    /// Occurs when bot tries to delete a message in group where it does not have enough rights
    #[serde(rename = "Bad Request: message can't be deleted")]
    MessageCantBeDeleted,

    /// Occurs when bot tries to edit a message which does not exists
    #[serde(rename = "Bad Request: message to edit not found")]
    MessageToEditNotFound,

    /// Occurs when bot tries to reply to a message which does not exists
    #[serde(rename = "Bad Request: reply message not found")]
    MessageToReplyNotFound,

    /// Occurs when bot tries to
    #[serde(rename = "Bad Request: message identifier is not specified")]
    MessageIdentifierNotSpecified,

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

    /// Occurs when bot tries to send method with unknown user_id (getUserProfilePhotos)
    #[serde(rename = "Bad Request: user not found")]
    UserNotFound,

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

    /// Occurs when bot tries to send InlineKeyboardMarkup with invalid button url
    #[serde(rename = "Bad Request: BUTTON_URL_INVALID")]
    ButtonURLInvalid,

    #[serde(rename = "Bad Request: URL host is empty")]
    URLHostIsEmpty, // TODO: docs

    #[serde(rename = "Bad Request: START_PARAM_INVALID")]
    StartParamInvalid, // TODO: docs

    /// Occurs when bot tries to send button with data size more than 64 bytes
    #[serde(rename = "Bad Request: BUTTON_DATA_INVALID")]
    ButtonDataInvalid,

    /// Occurs when bot tries to send button with data size == 0
    #[serde(rename = "Bad Request: can't parse inline keyboard button: Text buttons are unallowed in the inline keyboard")]
    TextButtonsAreUnallowed,

    /// Occurs when bot tries to get file by wrong file id
    #[serde(rename = "Bad Request: wrong file id")]
    WrongFileID,

    /// Occurs when bot tries to do some with group which was deactivated
    #[serde(rename = "Bad Request: group is deactivated")]
    GroupDeactivated,

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

    /// Occurs when bot tries to pin a message without rights to pin in this chat
    #[serde(rename = "Bad Request: not enough rights to pin a message")]
    NotEnoughRightsToPinMessage,

    /// Occurs when bot tries to use method in group which is allowed inly in a supergroup or channel
    #[serde(rename = "Bad Request: method is available only for supergroups and channel")]
    MethodNotAvailableInPrivateChats,

    /// Occurs when bot tries to demote chat creator
    #[serde(rename = "Bad Request: can't demote chat creator")]
    CantDemoteChatCreator,

    /// Occurs when bot tries to restrict self in group chats
    #[serde(rename = "Bad Request: can't restrict self")]
    CantRestrictSelf,

    /// Occurs when bot tries to restrict chat member without rights to restrict in this chat
    #[serde(rename = "Bad Request: not enough rights to restrict/unrestrict chat member")]
    NotEnoughRightsToRestrict,

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

    /// Occurs when bot tries set webhook to protocol other than HTTPS
    #[serde(rename = "Bad Request: bad webhook: HTTPS url must be provided for webhook")]
    WebhookRequireHTTPS,

    /// Occurs when bot tries to set webhook to port other than 80, 88, 443 or 8443
    #[serde(rename = "Bad Request: bad webhook: Webhook can be set up only on ports 80, 88, 443 or 8443")]
    BadWebhookPort,

    /// Occurs when bot tries to set webhook to unknown host
    #[serde(rename = "Bad Request: bad webhook: Failed to resolve host: Name or service not known")]
    UnknownHost,

    #[serde(rename = "Bad Request: getaddrinfo: Temporary failure in name resolution")]
    BadWebhookAddrInfo, // TODO: docs

    #[serde(rename = "Bad Request: failed to resolve host: no address associated with hostname")]
    BadWebhookNoAddressAssociatedWithHostname, // TODO: docs

    /// Occurs when bot tries to set webhook to invalid URL
    #[serde(rename = "Bad Request: can't parse URL")]
    CantParseUrl,

    #[serde(rename = "Bad Request: unsupported URL protocol")]
    UnsupportedUrlProtocol, // TODO: docs

    /// Occurs when bot tries to send message with unfinished entities
    #[serde(rename = "Bad Request: can't parse entities")]
    CantParseEntities,

    #[serde(rename = "Bad Request: result_id_duplicate")]
    ResultIdDuplicate, // TODO: docs

    #[serde(rename = "Bad Request: bot_domain_invalid")]
    BotDomainInvalid, // TODO: docs

    /// Occurs when bot tries to use getUpdates while webhook is active
    #[serde(rename = "can't use getUpdates method while webhook is active")]
    CantGetUpdates,

    /// Occurs when bot tries to do some in group where bot was kicked
    #[serde(rename = "Unauthorized: bot was kicked from a chat")]
    BotKicked,

    /// Occurs when bot tries to send message to deactivated user
    #[serde(rename = "Unauthorized: user is deactivated")]
    UserDeactivated,

    /// Occurs when you tries to initiate conversation with a user
    #[serde(rename = "Unauthorized: bot can't initiate conversation with a user")]
    CantInitiateConversation,

    /// Occurs when you tries to send message to bot
    #[serde(rename = "Unauthorized: bot can't send messages to bots")]
    CantTalkWithBots,

    /// Occurs when bot tries to send button with invalid http url
    #[serde(rename = "Bad Request: wrong HTTP URL")]
    WrongHTTPurl,

    /// Occurs when bot tries GetUpdate before the timeout.
    /// Make sure that only one Updater is running.
    #[serde(rename = "Conflict: terminated by other getUpdates request; make sure that only one bot instance is running")]
    TerminatedByOtherGetUpdates,

    /// Occurs when bot tries to get file by invalid file id
    #[serde(rename = "Bad Request: invalid file id")]
    FileIdInvalid,

    #[serde(other)]
    OtherKind
}