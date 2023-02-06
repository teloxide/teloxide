//! Possible error types.

use std::{io, time::Duration};

use serde::Deserialize;
use thiserror::Error;

use crate::types::ResponseParameters;

/// An error caused by sending a request to Telegram.
#[derive(Debug, Error)]
pub enum RequestError {
    /// A Telegram API error.
    #[error("A Telegram's error: {0}")]
    Api(#[from] ApiError),

    /// The group has been migrated to a supergroup with the specified
    /// identifier.
    #[error("The group has been migrated to a supergroup with ID #{0}")]
    // FIXME: change to `ChatId` :|
    MigrateToChatId(i64),

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    #[error("Retry after {0:?}")]
    RetryAfter(Duration),

    /// Network error while sending a request to Telegram.
    #[error("A network error: {0}")]
    // NOTE: this variant must not be created by anything except the explicit From impl
    Network(#[source] reqwest::Error),

    /// Error while parsing a response from Telegram.
    ///
    /// If you've received this error, please, [open an issue] with the
    /// description of the error.
    ///
    /// [open an issue]: https://github.com/teloxide/teloxide/issues/new
    #[error("An error while parsing JSON: {source} (raw: {raw:?})")]
    InvalidJson {
        #[source]
        source: serde_json::Error,
        /// The raw string JSON that couldn't been parsed
        raw: Box<str>,
    },

    /// Occurs when trying to send a file to Telegram.
    #[error("An I/O error: {0}")]
    Io(#[from] io::Error),
}

/// An error caused by downloading a file.
#[derive(Debug, Error)]
pub enum DownloadError {
    /// A network error while downloading a file from Telegram.
    #[error("A network error: {0}")]
    // NOTE: this variant must not be created by anything except the explicit From impl
    Network(#[source] reqwest::Error),

    /// An I/O error while writing a file to destination.
    #[error("An I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait AsResponseParameters {
    fn response_parameters(&self) -> Option<ResponseParameters>;

    fn retry_after(&self) -> Option<Duration> {
        self.response_parameters().and_then(|rp| match rp {
            ResponseParameters::RetryAfter(n) => Some(n),
            _ => None,
        })
    }

    fn migrate_to_chat_id(&self) -> Option<i64> {
        self.response_parameters().and_then(|rp| match rp {
            ResponseParameters::MigrateToChatId(id) => Some(id),
            _ => None,
        })
    }
}

impl AsResponseParameters for crate::RequestError {
    fn response_parameters(&self) -> Option<ResponseParameters> {
        match *self {
            Self::RetryAfter(n) => Some(ResponseParameters::RetryAfter(n)),
            Self::MigrateToChatId(id) => Some(ResponseParameters::MigrateToChatId(id)),
            _ => None,
        }
    }
}

/// A kind of an API error.
#[derive(Debug, Error, Deserialize, PartialEq, Hash, Eq, Clone)]
#[serde(field_identifier)]
#[non_exhaustive]
pub enum ApiError {
    /// Occurs when the bot tries to send message to user who blocked the bot.
    #[serde(rename = "Forbidden: bot was blocked by the user")]
    #[error("Forbidden: bot was blocked by the user")]
    BotBlocked,

    /// Occurs when the bot token is incorrect.
    // FIXME: rename this to something akin "InvalidToken"
    #[serde(rename = "Unauthorized")]
    #[error("Unauthorized")]
    NotFound,

    /// Occurs when bot tries to modify a message without modification content.
    ///
    /// May happen in methods:
    /// 1. [`EditMessageText`]
    ///
    /// [`EditMessageText`]: crate::payloads::EditMessageText
    #[serde(rename = "Bad Request: message is not modified: specified new message content and \
                      reply markup are exactly the same as a current content and reply markup \
                      of the message")]
    #[error(
        "Bad Request: message is not modified: specified new message content and reply markup are \
         exactly the same as a current content and reply markup of the message"
    )]
    MessageNotModified,

    /// Occurs when bot tries to forward or delete a message which was deleted.
    ///
    /// May happen in methods:
    /// 1. [`ForwardMessage`]
    /// 2. [`DeleteMessage`]
    ///
    /// [`ForwardMessage`]: crate::payloads::ForwardMessage
    /// [`DeleteMessage`]: crate::payloads::DeleteMessage
    #[serde(rename = "Bad Request: MESSAGE_ID_INVALID")]
    #[error("Bad Request: MESSAGE_ID_INVALID")]
    MessageIdInvalid,

    /// Occurs when bot tries to forward a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`ForwardMessage`]
    ///
    /// [`ForwardMessage`]: crate::payloads::ForwardMessage
    #[serde(rename = "Bad Request: message to forward not found")]
    #[error("Bad Request: message to forward not found")]
    MessageToForwardNotFound,

    /// Occurs when bot tries to delete a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`DeleteMessage`]
    ///
    /// [`DeleteMessage`]: crate::payloads::DeleteMessage
    #[serde(rename = "Bad Request: message to delete not found")]
    #[error("Bad Request: message to delete not found")]
    MessageToDeleteNotFound,

    /// Occurs when bot tries to send a text message without text.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: message text is empty")]
    #[error("Bad Request: message text is empty")]
    MessageTextIsEmpty,

    /// Occurs when bot tries to edit a message after long time.
    ///
    /// May happen in methods:
    /// 1. [`EditMessageText`]
    ///
    /// [`EditMessageText`]: crate::payloads::EditMessageText
    #[serde(rename = "Bad Request: message can't be edited")]
    #[error("Bad Request: message can't be edited")]
    MessageCantBeEdited,

    /// Occurs when bot tries to delete a someone else's message in group where
    /// it does not have enough rights.
    ///
    /// May happen in methods:
    /// 1. [`DeleteMessage`]
    ///
    /// [`DeleteMessage`]: crate::payloads::DeleteMessage
    #[serde(rename = "Bad Request: message can't be deleted")]
    #[error("Bad Request: message can't be deleted")]
    MessageCantBeDeleted,

    /// Occurs when bot tries to edit a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`EditMessageText`]
    ///
    /// [`EditMessageText`]: crate::payloads::EditMessageText
    #[serde(rename = "Bad Request: message to edit not found")]
    #[error("Bad Request: message to edit not found")]
    MessageToEditNotFound,

    /// Occurs when bot tries to reply to a message which does not exists.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: reply message not found")]
    #[error("Bad Request: reply message not found")]
    MessageToReplyNotFound,

    /// Occurs when bot tries to
    #[serde(rename = "Bad Request: message identifier is not specified")]
    #[error("Bad Request: message identifier is not specified")]
    MessageIdentifierNotSpecified,

    /// Occurs when bot tries to send a message with text size greater then
    /// 4096 symbols.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: message is too long")]
    #[error("Bad Request: message is too long")]
    MessageIsTooLong,

    /// Occurs when bot tries to edit a message with text size greater then
    /// 4096 symbols.
    ///
    /// May happen in methods:
    /// 1. [`EditMessageText`]
    /// 2. [`EditMessageTextInline`]
    /// 3. [`EditMessageCaption`]
    /// 4. [`EditMessageCaptionInline`]
    ///
    /// [`EditMessageText`]: crate::payloads::EditMessageText
    /// [`EditMessageTextInline`]: crate::payloads::EditMessageTextInline
    /// [`EditMessageCaption`]: crate::payloads::EditMessageCaption
    /// [`EditMessageCaptionInline`]: crate::payloads::EditMessageCaptionInline
    #[serde(rename = "Bad Request: MESSAGE_TOO_LONG")]
    #[error("Bad Request: MESSAGE_TOO_LONG")]
    EditedMessageIsTooLong,

    /// Occurs when bot tries to send media group with more than 10 items.
    ///
    /// May happen in methods:
    /// 1. [`SendMediaGroup`]
    ///
    /// [`SendMediaGroup`]: crate::payloads::SendMediaGroup
    #[serde(rename = "Bad Request: Too much messages to send as an album")]
    #[error("Bad Request: Too much messages to send as an album")]
    ToMuchMessages,

    /// Occurs when bot tries to answer an inline query with more than 50
    /// results.
    ///
    /// Consider using offsets to paginate results.
    ///
    /// May happen in methods:
    /// 1. [`AnswerInlineQuery`]
    ///
    /// [`AnswerInlineQuery`]: crate::payloads::AnswerInlineQuery
    #[serde(rename = "Bad Request: RESULTS_TOO_MUCH")]
    #[error("Bad Request: RESULTS_TOO_MUCH")]
    TooMuchInlineQueryResults,

    /// Occurs when bot tries to stop poll that has already been stopped.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::payloads::SendPoll
    #[serde(rename = "Bad Request: poll has already been closed")]
    #[error("Bad Request: poll has already been closed")]
    PollHasAlreadyClosed,

    /// Occurs when bot tries to send poll with less than 2 options.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::payloads::SendPoll
    #[serde(rename = "Bad Request: poll must have at least 2 option")]
    #[error("Bad Request: poll must have at least 2 option")]
    PollMustHaveMoreOptions,

    /// Occurs when bot tries to send poll with more than 10 options.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::payloads::SendPoll
    #[serde(rename = "Bad Request: poll can't have more than 10 options")]
    #[error("Bad Request: poll can't have more than 10 options")]
    PollCantHaveMoreOptions,

    /// Occurs when bot tries to send poll with empty option (without text).
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::payloads::SendPoll
    #[serde(rename = "Bad Request: poll options must be non-empty")]
    #[error("Bad Request: poll options must be non-empty")]
    PollOptionsMustBeNonEmpty,

    /// Occurs when bot tries to send poll with empty question (without text).
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::payloads::SendPoll
    #[serde(rename = "Bad Request: poll question must be non-empty")]
    #[error("Bad Request: poll question must be non-empty")]
    PollQuestionMustBeNonEmpty,

    /// Occurs when bot tries to send poll with total size of options more than
    /// 100 symbols.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::payloads::SendPoll
    #[serde(rename = "Bad Request: poll options length must not exceed 100")]
    #[error("Bad Request: poll options length must not exceed 100")]
    PollOptionsLengthTooLong,

    /// Occurs when bot tries to send poll with question size more than 255
    /// symbols.
    ///
    /// May happen in methods:
    /// 1. [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::payloads::SendPoll
    #[serde(rename = "Bad Request: poll question length must not exceed 255")]
    #[error("Bad Request: poll question length must not exceed 255")]
    PollQuestionLengthTooLong,

    /// Occurs when bot tries to stop poll with message without poll.
    ///
    /// May happen in methods:
    /// 1. [`StopPoll`]
    ///
    /// [`StopPoll`]: crate::payloads::StopPoll
    #[serde(rename = "Bad Request: message with poll to stop not found")]
    #[error("Bad Request: message with poll to stop not found")]
    MessageWithPollNotFound,

    /// Occurs when bot tries to stop poll with message without poll.
    ///
    /// May happen in methods:
    /// 1. [`StopPoll`]
    ///
    /// [`StopPoll`]: crate::payloads::StopPoll
    #[serde(rename = "Bad Request: message is not a poll")]
    #[error("Bad Request: message is not a poll")]
    MessageIsNotAPoll,

    /// Occurs when bot tries to send a message to chat in which it is not a
    /// member.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: chat not found")]
    #[error("Bad Request: chat not found")]
    ChatNotFound,

    /// Occurs when bot tries to send method with unknown user_id.
    ///
    /// May happen in methods:
    /// 1. [`getUserProfilePhotos`]
    ///
    /// [`getUserProfilePhotos`]:
    /// crate::payloads::GetUserProfilePhotos
    #[serde(rename = "Bad Request: user not found")]
    #[error("Bad Request: user not found")]
    UserNotFound,

    /// Occurs when bot tries to send [`SetChatDescription`] with same text as
    /// in the current description.
    ///
    /// May happen in methods:
    /// 1. [`SetChatDescription`]
    ///
    /// [`SetChatDescription`]: crate::payloads::SetChatDescription
    #[serde(rename = "Bad Request: chat description is not modified")]
    #[error("Bad Request: chat description is not modified")]
    ChatDescriptionIsNotModified,

    /// Occurs when bot tries to answer to query after timeout expire.
    ///
    /// May happen in methods:
    /// 1. [`AnswerCallbackQuery`]
    ///
    /// [`AnswerCallbackQuery`]: crate::payloads::AnswerCallbackQuery
    #[serde(rename = "Bad Request: query is too old and response timeout expired or query id is \
                      invalid")]
    #[error("Bad Request: query is too old and response timeout expired or query id is invalid")]
    InvalidQueryId,

    /// Occurs when bot tries to send InlineKeyboardMarkup with invalid button
    /// url.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: BUTTON_URL_INVALID")]
    #[error("Bad Request: BUTTON_URL_INVALID")]
    ButtonUrlInvalid,

    /// Occurs when bot tries to send button with data size more than 64 bytes.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: BUTTON_DATA_INVALID")]
    #[error("Bad Request: BUTTON_DATA_INVALID")]
    ButtonDataInvalid,

    /// Occurs when bot tries to send button with data size == 0.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: can't parse inline keyboard button: Text buttons are \
                      unallowed in the inline keyboard")]
    #[error(
        "Bad Request: can't parse inline keyboard button: Text buttons are unallowed in the \
         inline keyboard"
    )]
    TextButtonsAreUnallowed,

    /// Occurs when bot tries to get file by wrong file id.
    ///
    /// May happen in methods:
    /// 1. [`GetFile`]
    ///
    /// [`GetFile`]: crate::payloads::GetFile
    #[serde(rename = "Bad Request: wrong file id")]
    #[error("Bad Request: wrong file id")]
    WrongFileId,

    /// Occurs when bot tries to send files with wrong file identifier or HTTP
    /// url
    #[serde(rename = "Bad Request: wrong file identifier/HTTP URL specified")]
    #[error("Bad Request: wrong file identifier/HTTP URL specified")]
    WrongFileIdOrUrl,

    /// Occurs when When sending files with an url to a site that doesn't
    /// respond.
    #[serde(rename = "Bad Request: failed to get HTTP URL content")]
    #[error("Bad Request: failed to get HTTP URL content")]
    FailedToGetUrlContent,

    /// Occurs when bot tries to do some with group which was deactivated.
    #[serde(rename = "Bad Request: group is deactivated")]
    #[error("Bad Request: group is deactivated")]
    GroupDeactivated,

    /// Occurs when image processing fails on telegram's side.
    ///
    /// This is likely caused by an incorrectly encoded image, make sure that
    /// the image is correctly encoded in a format telegram accepts.
    #[serde(rename = "Bad Request: IMAGE_PROCESS_FAILED")]
    #[error("Bad Request: IMAGE_PROCESS_FAILED")]
    ImageProcessFailed,

    /// Occurs when bot tries to set chat photo from file ID
    ///
    /// May happen in methods:
    /// 1. [`SetChatPhoto`]
    ///
    /// [`SetChatPhoto`]: crate::payloads::SetChatPhoto
    #[serde(rename = "Bad Request: Photo should be uploaded as an InputFile")]
    #[error("Bad Request: Photo should be uploaded as an InputFile")]
    PhotoAsInputFileRequired,

    /// Occurs when bot tries to add sticker to stickerset by invalid name.
    ///
    /// May happen in methods:
    /// 1. [`AddStickerToSet`]
    ///
    /// [`AddStickerToSet`]: crate::payloads::AddStickerToSet
    #[serde(rename = "Bad Request: STICKERSET_INVALID")]
    #[error("Bad Request: STICKERSET_INVALID")]
    InvalidStickersSet,

    /// Occurs when bot tries to create a sticker set with a name that is
    /// already used by another sticker set.
    ///
    /// May happen in methods:
    /// 1. [`CreateNewStickerSet`]
    ///
    /// [`CreateNewStickerSet`]: crate::payloads::CreateNewStickerSet
    #[serde(rename = "Bad Request: sticker set name is already occupied")]
    #[error("Bad Request: sticker set name is already occupied")]
    StickerSetNameOccupied,

    /// Occurs when bot tries to create a sticker set with user id of a bot.
    ///
    /// May happen in methods:
    /// 1. [`CreateNewStickerSet`]
    ///
    /// [`CreateNewStickerSet`]: crate::payloads::CreateNewStickerSet
    #[serde(rename = "Bad Request: USER_IS_BOT")]
    #[error("Bad Request: USER_IS_BOT")]
    StickerSetOwnerIsBot,

    /// Occurs when bot tries to create a sticker set with invalid name.
    ///
    /// From documentation of [`CreateNewStickerSet`]:
    /// > Short name of sticker set, to be used in `t.me/addstickers/` URLs
    /// (e.g., _animals_). Can contain only english letters, digits and
    /// underscores. Must begin with a letter, can't contain consecutive
    /// underscores and must end in “\_by\_<bot\_username>”. <bot\_username>
    /// is case insensitive. 1-64 characters.
    ///
    /// May happen in methods:
    /// 1. [`CreateNewStickerSet`]
    ///
    /// [`CreateNewStickerSet`]: crate::payloads::CreateNewStickerSet
    #[serde(rename = "Bad Request: invalid sticker set name is specified")]
    #[error("Bad Request: invalid sticker set name is specified")]
    InvalidStickerName,

    /// Occurs when bot tries to pin a message without rights to pin in this
    /// chat.
    ///
    /// May happen in methods:
    /// 1. [`PinChatMessage`]
    ///
    /// [`PinChatMessage`]: crate::payloads::PinChatMessage
    #[serde(rename = "Bad Request: not enough rights to pin a message")]
    #[error("Bad Request: not enough rights to pin a message")]
    NotEnoughRightsToPinMessage,

    /// Occurs when bot tries to pin or unpin a message without rights to pin
    /// in this chat.
    ///
    /// May happen in methods:
    /// 1. [`PinChatMessage`]
    /// 2. [`UnpinChatMessage`]
    ///
    /// [`PinChatMessage`]: crate::payloads::PinChatMessage
    /// [`UnpinChatMessage`]: crate::payloads::UnpinChatMessage
    #[serde(rename = "Bad Request: not enough rights to manage pinned messages in the chat")]
    #[error("Bad Request: not enough rights to manage pinned messages in the chat")]
    NotEnoughRightsToManagePins,

    /// Occurs when bot tries change default chat permissions without "Ban
    /// Users" permission in this chat.
    ///
    /// May happen in methods:
    /// 1. [`SetChatPermissions`]
    ///
    /// [`SetChatPermissions`]: crate::payloads::SetChatPermissions
    #[serde(rename = "Bad Request: not enough rights to change chat permissions")]
    #[error("Bad Request: not enough rights to change chat permissions")]
    NotEnoughRightsToChangeChatPermissions,

    /// Occurs when bot tries to use method in group which is allowed only in a
    /// supergroup or channel.
    #[serde(rename = "Bad Request: method is available only for supergroups and channel")]
    #[error("Bad Request: method is available only for supergroups and channel")]
    MethodNotAvailableInPrivateChats,

    /// Occurs when bot tries to demote chat creator.
    ///
    /// May happen in methods:
    /// 1. [`PromoteChatMember`]
    ///
    /// [`PromoteChatMember`]: crate::payloads::PromoteChatMember
    #[serde(rename = "Bad Request: can't demote chat creator")]
    #[error("Bad Request: can't demote chat creator")]
    CantDemoteChatCreator,

    /// Occurs when bot tries to restrict self in group chats.
    ///
    /// May happen in methods:
    /// 1. [`RestrictChatMember`]
    ///
    /// [`RestrictChatMember`]: crate::payloads::RestrictChatMember
    #[serde(rename = "Bad Request: can't restrict self")]
    #[error("Bad Request: can't restrict self")]
    CantRestrictSelf,

    /// Occurs when bot tries to restrict chat member without rights to
    /// restrict in this chat.
    ///
    /// May happen in methods:
    /// 1. [`RestrictChatMember`]
    ///
    /// [`RestrictChatMember`]: crate::payloads::RestrictChatMember
    #[serde(rename = "Bad Request: not enough rights to restrict/unrestrict chat member")]
    #[error("Bad Request: not enough rights to restrict/unrestrict chat member")]
    NotEnoughRightsToRestrict,

    /// Occurs when bot tries to post a message in a channel without "Post
    /// Messages" admin right.
    #[serde(rename = "Bad Request: need administrator rights in the channel chat")]
    #[error("Bad Request: need administrator rights in the channel chat")]
    NotEnoughRightsToPostMessages,

    /// Occurs when bot tries set webhook to protocol other than HTTPS.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::payloads::SetWebhook
    #[serde(rename = "Bad Request: bad webhook: HTTPS url must be provided for webhook")]
    #[error("Bad Request: bad webhook: HTTPS url must be provided for webhook")]
    WebhookRequireHttps,

    /// Occurs when bot tries to set webhook to port other than 80, 88, 443 or
    /// 8443.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::payloads::SetWebhook
    #[serde(rename = "Bad Request: bad webhook: Webhook can be set up only on ports 80, 88, 443 \
                      or 8443")]
    #[error("Bad Request: bad webhook: Webhook can be set up only on ports 80, 88, 443 or 8443")]
    BadWebhookPort,

    /// Occurs when bot tries to set webhook to unknown host.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::payloads::SetWebhook
    #[serde(rename = "Bad Request: bad webhook: Failed to resolve host: Name or service not known")]
    #[error("Bad Request: bad webhook: Failed to resolve host: Name or service not known")]
    UnknownHost,

    /// Occurs when bot tries to set webhook to invalid URL.
    ///
    /// May happen in methods:
    /// 1. [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::payloads::SetWebhook
    #[serde(rename = "Bad Request: can't parse URL")]
    #[error("Bad Request: can't parse URL")]
    CantParseUrl,

    /// Occurs when bot tries to send message with unfinished entities.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: can't parse entities")]
    #[error("Bad Request: can't parse entities")]
    CantParseEntities,

    /// Occurs when bot tries to use getUpdates while webhook is active.
    ///
    /// May happen in methods:
    /// 1. [`GetUpdates`]
    ///
    /// [`GetUpdates`]: crate::payloads::GetUpdates
    #[serde(rename = "can't use getUpdates method while webhook is active")]
    #[error("can't use getUpdates method while webhook is active")]
    CantGetUpdates,

    /// Occurs when bot tries to do some in group where bot was kicked.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Unauthorized: bot was kicked from a chat")]
    #[error("Unauthorized: bot was kicked from a chat")]
    BotKicked,

    /// Occurs when bot tries to do something in a supergroup the bot was
    /// kicked from.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Forbidden: bot was kicked from the supergroup chat")]
    #[error("Forbidden: bot was kicked from the supergroup chat")]
    BotKickedFromSupergroup,

    /// Occurs when bot tries to send a message to a deactivated user (i.e. a
    /// user that was banned by telegram).
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Forbidden: user is deactivated")]
    #[error("Forbidden: user is deactivated")]
    UserDeactivated,

    /// Occurs when you tries to initiate conversation with a user.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Unauthorized: bot can't initiate conversation with a user")]
    #[error("Unauthorized: bot can't initiate conversation with a user")]
    CantInitiateConversation,

    /// Occurs when you tries to send message to bot.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Unauthorized: bot can't send messages to bots")]
    #[error("Unauthorized: bot can't send messages to bots")]
    CantTalkWithBots,

    /// Occurs when bot tries to send button with invalid http url.
    ///
    /// May happen in methods:
    /// 1. [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::payloads::SendMessage
    #[serde(rename = "Bad Request: wrong HTTP URL")]
    #[error("Bad Request: wrong HTTP URL")]
    WrongHttpUrl,

    /// Occurs when bot tries GetUpdate before the timeout. Make sure that only
    /// one Updater is running.
    ///
    /// May happen in methods:
    /// 1. [`GetUpdates`]
    ///
    /// [`GetUpdates`]: crate::payloads::GetUpdates
    #[serde(rename = "Conflict: terminated by other getUpdates request; make sure that only one \
                      bot instance is running")]
    #[error(
        "Conflict: terminated by other getUpdates request; make sure that only one bot instance \
         is running"
    )]
    TerminatedByOtherGetUpdates,

    /// Occurs when bot tries to get file by invalid file id.
    ///
    /// May happen in methods:
    /// 1. [`GetFile`]
    ///
    /// [`GetFile`]: crate::payloads::GetFile
    #[serde(rename = "Bad Request: invalid file id")]
    #[error("Bad Request: invalid file id")]
    FileIdInvalid,

    /// Occurs when bot tries to upload a file which is larger than 50 MB using
    /// multipart/form-data.
    ///
    /// May happen in methods:
    /// 1. [`SendVideo`]
    /// 2. [`SendDocument`]
    ///
    /// [`SendVideo`]: crate::payloads::SendVideo
    /// [`SendDocument`]: crate::payloads::SendDocument
    #[serde(rename = "Request Entity Too Large")]
    #[error("Request Entity Too Large")]
    RequestEntityTooLarge,

    /// Error which is not known to `teloxide`.
    ///
    /// If you've received this error, please [open an issue] with the
    /// description of the error.
    ///
    /// [open an issue]: https://github.com/teloxide/teloxide/issues/new
    #[error("Unknown error: {0:?}")]
    Unknown(String),
}

/// This impl allows to use `?` to propagate [`DownloadError`]s in function
/// returning [`RequestError`]s. For example:
///
/// ```rust
/// # use teloxide_core::errors::{DownloadError, RequestError};
///
/// async fn handler() -> Result<(), RequestError> {
///     download_file().await?; // `?` just works
///
///     Ok(())
/// }
///
/// async fn download_file() -> Result<(), DownloadError> {
///     /* download file here */
///     Ok(())
/// }
/// ```
impl From<DownloadError> for RequestError {
    fn from(download_err: DownloadError) -> Self {
        match download_err {
            DownloadError::Network(err) => RequestError::Network(err),
            DownloadError::Io(err) => RequestError::Io(err),
        }
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(error: reqwest::Error) -> Self {
        DownloadError::Network(hide_token(error))
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        RequestError::Network(hide_token(error))
    }
}

/// Replaces token in the url in the error with `token:redacted` string.
pub(crate) fn hide_token(mut error: reqwest::Error) -> reqwest::Error {
    let url = match error.url_mut() {
        Some(url) => url,
        None => return error,
    };

    if let Some(mut segments) = url.path_segments() {
        // Usually the url looks like "bot<token>/..." or "file/bot<token>/...".
        let (beginning, segment) = match segments.next() {
            Some("file") => ("file/", segments.next()),
            segment => ("", segment),
        };

        if let Some(token) = segment.and_then(|s| s.strip_prefix("bot")) {
            // make sure that what we are about to delete looks like a bot token
            if let Some((id, secret)) = token.split_once(':') {
                // The part before the : in the token is the id of the bot.
                let id_character = |c: char| c.is_ascii_digit();

                // The part after the : in the token is the secret.
                //
                // In all bot tokens we could find the secret is 35 characters long and is
                // 0-9a-zA-Z_- only.
                //
                // It would be nice to research if TBA always has 35 character secrets or if it
                // is just a coincidence.
                const SECRET_LENGTH: usize = 35;
                let secret_character = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';

                if secret.len() >= SECRET_LENGTH
                    && id.chars().all(id_character)
                    && secret.chars().all(secret_character)
                {
                    // found token, hide only the token
                    let without_token =
                        &url.path()[(beginning.len() + "/bot".len() + token.len())..];
                    let redacted = format!("{beginning}token:redacted{without_token}");

                    url.set_path(&redacted);
                    return error;
                }
            }
        }
    }

    // couldn't find token in the url, hide the whole url
    error.without_url()
}
