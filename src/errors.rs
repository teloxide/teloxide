use derive_more::From;
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

//<editor-fold desc="download">
/// An error occurred after downloading a file.
#[derive(Debug, Error, From)]
pub enum DownloadError {
    #[error("A network error: {0}")]
    NetworkError(#[source] reqwest::Error),

    #[error("An I/O error: {0}")]
    Io(#[source] std::io::Error),
}

//</editor-fold>

//<editor-fold desc="request">
/// An error occurred after making a request to Telegram.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("A Telegram's error #{status_code}: {kind:?}")]
    ApiError { status_code: StatusCode, kind: ApiErrorKind },

    /// The group has been migrated to a supergroup with the specified
    /// identifier.
    #[error("The group has been migrated to a supergroup with ID #{0}")]
    MigrateToChatId(i64),

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    #[error("Retry after {0} seconds")]
    RetryAfter(i32),

    #[error("A network error: {0}")]
    NetworkError(#[source] reqwest::Error),

    #[error("An error while parsing JSON: {0}")]
    InvalidJson(#[source] serde_json::Error),
}

//</editor-fold>

/// A kind of an API error returned from Telegram.
#[derive(Debug, Deserialize, PartialEq, Copy, Hash, Eq, Clone)]
pub enum ApiErrorKind {
    /// Occurs when the bot tries to send message to user who blocked the bot.
    #[serde(rename = "Forbidden: bot was blocked by the user")]
    BotBlocked,

    /// Occurs when bot tries to modify a message without modification content.
    ///
    /// May happen in methods:
    /// 1. [`EditMessageText`]
    ///
    /// [`EditMessageText`]: crate::requests::EditMessageText
    #[serde(rename = "Bad Request: message is not modified: specified new \
                      message content and reply markup are exactly the same \
                      as a current content and reply markup of the message")]
    MessageNotModified,

    /// Occurs when bot tries to forward or delete a message which was deleted.
    ///
    /// May happen in methods:
    /// 1. [`ForwardMessage`]
    /// 2. [`DeleteMessage`]
    ///
    /// [`ForwardMessage`]: crate::requests::ForwardMessage
    /// [`DeleteMessage`]: crate::requests::DeleteMessage
    #[serde(rename = "Bad Request: MESSAGE_ID_INVALID")]
    MessageIdInvalid,

    /// Occurs when bot tries to forward a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`ForwardMessage`]
    ///
    /// [`ForwardMessage`]: crate::requests::ForwardMessage
    #[serde(rename = "Bad Request: message to forward not found")]
    MessageToForwardNotFound,

    /// Occurs when bot tries to delete a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`DeleteMessage`]
    ///
    /// [`DeleteMessage`]: crate::requests::DeleteMessage
    #[serde(rename = "Bad Request: message to delete not found")]
    MessageToDeleteNotFound,

    /// Occurs when bot tries to send a text message without text.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: message text is empty")]
    MessageTextIsEmpty,

    /// Occurs when bot tries to edit a message after long time.
    ///
    /// May happen in methods:
    /// 1. [`EditMessageText`]
    ///
    /// [`EditMessageText`]: crate::requests::EditMessageText
    #[serde(rename = "Bad Request: message can't be edited")]
    MessageCantBeEdited,

    /// Occurs when bot tries to delete a someone else's message in group where
    /// it does not have enough rights.
    ///
    /// May happen in methods:
    /// 1. [`DeleteMessage`]
    ///
    /// [`DeleteMessage`]: crate::requests::DeleteMessage
    #[serde(rename = "Bad Request: message can't be deleted")]
    MessageCantBeDeleted,

    /// Occurs when bot tries to edit a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`EditMessageText`]
    ///
    /// [`EditMessageText`]: crate::requests::EditMessageText
    #[serde(rename = "Bad Request: message to edit not found")]
    MessageToEditNotFound,

    /// Occurs when bot tries to reply to a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: reply message not found")]
    MessageToReplyNotFound,

    /// Occurs when bot tries to
    #[serde(rename = "Bad Request: message identifier is not specified")]
    MessageIdentifierNotSpecified,

    /// Occurs when bot tries to send a message with text size greater then
    /// 4096 symbols.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: message is too long")]
    MessageIsTooLong,

    /// Occurs when bot tries to send media group with more than 10 items.
    ///
    /// May happen in methods:
    /// 1. [`SendMediaGroup`]
    ///
    /// [`SendMediaGroup`]: crate::requests::SendMediaGroup
    #[serde(rename = "Bad Request: Too much messages to send as an album")]
    ToMuchMessages,

    /// Occurs when bot tries to stop poll that has already been stopped.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::SendPoll
    #[serde(rename = "Bad Request: poll has already been closed")]
    PollHasAlreadyClosed,

    /// Occurs when bot tries to send poll with less than 2 options.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::SendPoll
    #[serde(rename = "Bad Request: poll must have at least 2 option")]
    PollMustHaveMoreOptions,

    /// Occurs when bot tries to send poll with more than 10 options.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::SendPoll
    #[serde(rename = "Bad Request: poll can't have more than 10 options")]
    PollCantHaveMoreOptions,

    /// Occurs when bot tries to send poll with empty option (without text).
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::SendPoll
    #[serde(rename = "Bad Request: poll options must be non-empty")]
    PollOptionsMustBeNonEmpty,

    /// Occurs when bot tries to send poll with empty question (without text).
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::SendPoll
    #[serde(rename = "Bad Request: poll question must be non-empty")]
    PollQuestionMustBeNonEmpty,

    /// Occurs when bot tries to send poll with total size of options more than
    /// 100 symbols.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::SendPoll
    #[serde(rename = "Bad Request: poll options length must not exceed 100")]
    PollOptionsLengthTooLong,

    /// Occurs when bot tries to send poll with question size more than 255
    /// symbols.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::SendPoll
    #[serde(rename = "Bad Request: poll question length must not exceed 255")]
    PollQuestionLengthTooLong,

    /// Occurs when bot tries to stop poll with message without poll.
    ///
    /// May happen in methods:
    /// 1. [`StopPoll`]
    ///
    /// [`StopPoll`]: crate::requests::StopPoll
    #[serde(rename = "Bad Request: message with poll to stop not found")]
    MessageWithPollNotFound,

    /// Occurs when bot tries to stop poll with message without poll.
    ///
    /// May happen in methods:
    /// 1. [`StopPoll`]
    ///
    /// [`StopPoll`]: crate::requests::StopPoll
    #[serde(rename = "Bad Request: message is not a poll")]
    MessageIsNotAPoll,

    /// Occurs when bot tries to send a message to chat in which it is not a
    /// member.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: chat not found")]
    ChatNotFound,

    /// Occurs when bot tries to send method with unknown user_id.
    ///
    /// May happen in methods:
    /// 1. [`getUserProfilePhotos`]
    ///
    /// [`getUserProfilePhotos`]:
    /// crate::requests::GetUserProfilePhotos
    #[serde(rename = "Bad Request: user not found")]
    UserNotFound,

    /// Occurs when bot tries to send [`SetChatDescription`] with same text as
    /// in the current description.
    ///
    /// May happen in methods:
    /// 1. [`SetChatDescription`]
    ///
    /// [`SetChatDescription`]: crate::requests::SetChatDescription
    #[serde(rename = "Bad Request: chat description is not modified")]
    ChatDescriptionIsNotModified,

    /// Occurs when bot tries to answer to query after timeout expire.
    ///
    /// May happen in methods:
    /// 1. [`AnswerCallbackQuery`]
    ///
    /// [`AnswerCallbackQuery`]: crate::requests::AnswerCallbackQuery
    #[serde(rename = "Bad Request: query is too old and response timeout \
                      expired or query id is invalid")]
    InvalidQueryID,

    /// Occurs when bot tries to send InlineKeyboardMarkup with invalid button
    /// url.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: BUTTON_URL_INVALID")]
    ButtonURLInvalid,

    /// Occurs when bot tries to send button with data size more than 64 bytes.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: BUTTON_DATA_INVALID")]
    ButtonDataInvalid,

    /// Occurs when bot tries to send button with data size == 0.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: can't parse inline keyboard button: Text \
                      buttons are unallowed in the inline keyboard")]
    TextButtonsAreUnallowed,

    /// Occurs when bot tries to get file by wrong file id.
    ///
    /// May happen in methods:
    /// 1. [`GetFile`]
    ///
    /// [`GetFile`]: crate::requests::GetFile
    #[serde(rename = "Bad Request: wrong file id")]
    WrongFileID,

    /// Occurs when bot tries to do some with group which was deactivated.
    #[serde(rename = "Bad Request: group is deactivated")]
    GroupDeactivated,

    /// Occurs when bot tries to set chat photo from file ID
    ///
    /// May happen in methods:
    /// 1. [`SetChatPhoto`]
    ///
    /// [`SetChatPhoto`]: crate::requests::SetChatPhoto
    #[serde(rename = "Bad Request: Photo should be uploaded as an InputFile")]
    PhotoAsInputFileRequired,

    /// Occurs when bot tries to add sticker to stickerset by invalid name.
    ///
    /// May happen in methods:
    /// 1. [`AddStickerToSet`]
    ///
    /// [`AddStickerToSet`]: crate::requests::AddStickerToSet
    #[serde(rename = "Bad Request: STICKERSET_INVALID")]
    InvalidStickersSet,

    /// Occurs when bot tries to pin a message without rights to pin in this
    /// chat.
    ///
    /// May happen in methods:
    /// 1. [`PinChatMessage`]
    ///
    /// [`PinChatMessage`]: crate::requests::PinChatMessage
    #[serde(rename = "Bad Request: not enough rights to pin a message")]
    NotEnoughRightsToPinMessage,

    /// Occurs when bot tries to use method in group which is allowed only in a
    /// supergroup or channel.
    #[serde(rename = "Bad Request: method is available only for supergroups \
                      and channel")]
    MethodNotAvailableInPrivateChats,

    /// Occurs when bot tries to demote chat creator.
    ///
    /// May happen in methods:
    /// 1. [`PromoteChatMember`]
    ///
    /// [`PromoteChatMember`]: crate::requests::PromoteChatMember
    #[serde(rename = "Bad Request: can't demote chat creator")]
    CantDemoteChatCreator,

    /// Occurs when bot tries to restrict self in group chats.
    ///
    /// May happen in methods:
    /// 1. [`RestrictChatMember`]
    ///
    /// [`RestrictChatMember`]: crate::requests::RestrictChatMember
    #[serde(rename = "Bad Request: can't restrict self")]
    CantRestrictSelf,

    /// Occurs when bot tries to restrict chat member without rights to
    /// restrict in this chat.
    ///
    /// May happen in methods:
    /// 1. [`RestrictChatMember`]
    ///
    /// [`RestrictChatMember`]: crate::requests::RestrictChatMember
    #[serde(rename = "Bad Request: not enough rights to restrict/unrestrict \
                      chat member")]
    NotEnoughRightsToRestrict,

    /// Occurs when bot tries set webhook to protocol other than HTTPS.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::requests::SetWebhook
    #[serde(rename = "Bad Request: bad webhook: HTTPS url must be provided \
                      for webhook")]
    WebhookRequireHTTPS,

    /// Occurs when bot tries to set webhook to port other than 80, 88, 443 or
    /// 8443.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::requests::SetWebhook
    #[serde(rename = "Bad Request: bad webhook: Webhook can be set up only \
                      on ports 80, 88, 443 or 8443")]
    BadWebhookPort,

    /// Occurs when bot tries to set webhook to unknown host.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::requests::SetWebhook
    #[serde(rename = "Bad Request: bad webhook: Failed to resolve host: \
                      Name or service not known")]
    UnknownHost,

    /// Occurs when bot tries to set webhook to invalid URL.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::requests::SetWebhook
    #[serde(rename = "Bad Request: can't parse URL")]
    CantParseUrl,

    /// Occurs when bot tries to send message with unfinished entities.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: can't parse entities")]
    CantParseEntities,

    /// Occurs when bot tries to use getUpdates while webhook is active.
    ///
    /// May happen in methods:
    /// 1. [`GetUpdates`]
    ///
    /// [`GetUpdates`]: crate::requests::GetUpdates
    #[serde(rename = "can't use getUpdates method while webhook is active")]
    CantGetUpdates,

    /// Occurs when bot tries to do some in group where bot was kicked.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Unauthorized: bot was kicked from a chat")]
    BotKicked,

    /// Occurs when bot tries to send message to deactivated user.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Unauthorized: user is deactivated")]
    UserDeactivated,

    /// Occurs when you tries to initiate conversation with a user.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(
        rename = "Unauthorized: bot can't initiate conversation with a user"
    )]
    CantInitiateConversation,

    /// Occurs when you tries to send message to bot.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Unauthorized: bot can't send messages to bots")]
    CantTalkWithBots,

    /// Occurs when bot tries to send button with invalid http url.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::SendMessage
    #[serde(rename = "Bad Request: wrong HTTP URL")]
    WrongHTTPurl,

    /// Occurs when bot tries GetUpdate before the timeout. Make sure that only
    /// one Updater is running.
    ///
    /// May happen in methods:
    /// 1. [`GetUpdates`]
    ///
    /// [`GetUpdates`]: crate::requests::GetUpdates
    #[serde(rename = "Conflict: terminated by other getUpdates request; \
                      make sure that only one bot instance is running")]
    TerminatedByOtherGetUpdates,

    /// Occurs when bot tries to get file by invalid file id.
    ///
    /// May happen in methods:
    /// 1. [`GetFile`]
    ///
    /// [`GetFile`]: crate::requests::GetFile
    #[serde(rename = "Bad Request: invalid file id")]
    FileIdInvalid,

    #[serde(other)]
    Other,
}
