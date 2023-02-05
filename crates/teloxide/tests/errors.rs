use serde::Deserialize;

#[test]
fn custom_result() {
    use teloxide_core::ApiError;

    let cases = &[
        ("{\"data\": \"Forbidden: bot was blocked by the user\"}", ApiError::BotBlocked),
        ("{\"data\": \"Unauthorized\"}", ApiError::NotFound),
        (
            "{\"data\": \"Bad Request: message is not modified: specified new message content and \
             reply markup are exactly the same as a current content and reply markup of the \
             message\"}",
            ApiError::MessageNotModified,
        ),
        ("{\"data\": \"Bad Request: MESSAGE_ID_INVALID\"}", ApiError::MessageIdInvalid),
        (
            "{\"data\": \"Bad Request: message to forward not found\"}",
            ApiError::MessageToForwardNotFound,
        ),
        (
            "{\"data\": \"Bad Request: message to delete not found\"}",
            ApiError::MessageToDeleteNotFound,
        ),
        ("{\"data\": \"Bad Request: message text is empty\"}", ApiError::MessageTextIsEmpty),
        ("{\"data\": \"Bad Request: message can't be edited\"}", ApiError::MessageCantBeEdited),
        ("{\"data\": \"Bad Request: message can't be deleted\"}", ApiError::MessageCantBeDeleted),
        ("{\"data\": \"Bad Request: message to edit not found\"}", ApiError::MessageToEditNotFound),
        ("{\"data\": \"Bad Request: reply message not found\"}", ApiError::MessageToReplyNotFound),
        (
            "{\"data\": \"Bad Request: message identifier is not specified\"}",
            ApiError::MessageIdentifierNotSpecified,
        ),
        ("{\"data\": \"Bad Request: message is too long\"}", ApiError::MessageIsTooLong),
        ("{\"data\": \"Bad Request: MESSAGE_TOO_LONG\"}", ApiError::EditedMessageIsTooLong),
        (
            "{\"data\": \"Bad Request: Too much messages to send as an album\"}",
            ApiError::ToMuchMessages,
        ),
        ("{\"data\": \"Bad Request: RESULTS_TOO_MUCH\"}", ApiError::TooMuchInlineQueryResults),
        (
            "{\"data\": \"Bad Request: poll has already been closed\"}",
            ApiError::PollHasAlreadyClosed,
        ),
        (
            "{\"data\": \"Bad Request: poll must have at least 2 option\"}",
            ApiError::PollMustHaveMoreOptions,
        ),
        (
            "{\"data\": \"Bad Request: poll can't have more than 10 options\"}",
            ApiError::PollCantHaveMoreOptions,
        ),
        (
            "{\"data\": \"Bad Request: poll options must be non-empty\"}",
            ApiError::PollOptionsMustBeNonEmpty,
        ),
        (
            "{\"data\": \"Bad Request: poll question must be non-empty\"}",
            ApiError::PollQuestionMustBeNonEmpty,
        ),
        (
            "{\"data\": \"Bad Request: poll options length must not exceed 100\"}",
            ApiError::PollOptionsLengthTooLong,
        ),
        (
            "{\"data\": \"Bad Request: poll question length must not exceed 255\"}",
            ApiError::PollQuestionLengthTooLong,
        ),
        (
            "{\"data\": \"Bad Request: message with poll to stop not found\"}",
            ApiError::MessageWithPollNotFound,
        ),
        ("{\"data\": \"Bad Request: message is not a poll\"}", ApiError::MessageIsNotAPoll),
        ("{\"data\": \"Bad Request: chat not found\"}", ApiError::ChatNotFound),
        ("{\"data\": \"Bad Request: user not found\"}", ApiError::UserNotFound),
        (
            "{\"data\": \"Bad Request: chat description is not modified\"}",
            ApiError::ChatDescriptionIsNotModified,
        ),
        (
            "{\"data\": \"Bad Request: query is too old and response timeout expired or query id \
             is invalid\"}",
            ApiError::InvalidQueryId,
        ),
        ("{\"data\": \"Bad Request: BUTTON_URL_INVALID\"}", ApiError::ButtonUrlInvalid),
        ("{\"data\": \"Bad Request: BUTTON_DATA_INVALID\"}", ApiError::ButtonDataInvalid),
        (
            "{\"data\": \"Bad Request: can't parse inline keyboard button: Text buttons are \
             unallowed in the inline keyboard\"}",
            ApiError::TextButtonsAreUnallowed,
        ),
        ("{\"data\": \"Bad Request: wrong file id\"}", ApiError::WrongFileId),
        (
            "{\"data\": \"Bad Request: wrong file identifier/HTTP URL specified\"}",
            ApiError::WrongFileIdOrUrl,
        ),
        (
            "{\"data\": \"Bad Request: failed to get HTTP URL content\"}",
            ApiError::FailedToGetUrlContent,
        ),
        ("{\"data\": \"Bad Request: group is deactivated\"}", ApiError::GroupDeactivated),
        ("{\"data\": \"Bad Request: IMAGE_PROCESS_FAILED\"}", ApiError::ImageProcessFailed),
        (
            "{\"data\": \"Bad Request: Photo should be uploaded as an InputFile\"}",
            ApiError::PhotoAsInputFileRequired,
        ),
        ("{\"data\": \"Bad Request: STICKERSET_INVALID\"}", ApiError::InvalidStickersSet),
        (
            "{\"data\": \"Bad Request: sticker set name is already occupied\"}",
            ApiError::StickerSetNameOccupied,
        ),
        ("{\"data\": \"Bad Request: USER_IS_BOT\"}", ApiError::StickerSetOwnerIsBot),
        (
            "{\"data\": \"Bad Request: invalid sticker set name is specified\"}",
            ApiError::InvalidStickerName,
        ),
        (
            "{\"data\": \"Bad Request: not enough rights to pin a message\"}",
            ApiError::NotEnoughRightsToPinMessage,
        ),
        (
            "{\"data\": \"Bad Request: not enough rights to manage pinned messages in the chat\"}",
            ApiError::NotEnoughRightsToManagePins,
        ),
        (
            "{\"data\": \"Bad Request: not enough rights to change chat permissions\"}",
            ApiError::NotEnoughRightsToChangeChatPermissions,
        ),
        (
            "{\"data\": \"Bad Request: method is available only for supergroups and channel\"}",
            ApiError::MethodNotAvailableInPrivateChats,
        ),
        ("{\"data\": \"Bad Request: can't demote chat creator\"}", ApiError::CantDemoteChatCreator),
        ("{\"data\": \"Bad Request: can't restrict self\"}", ApiError::CantRestrictSelf),
        (
            "{\"data\": \"Bad Request: not enough rights to restrict/unrestrict chat member\"}",
            ApiError::NotEnoughRightsToRestrict,
        ),
        (
            "{\"data\": \"Bad Request: need administrator rights in the channel chat\"}",
            ApiError::NotEnoughRightsToPostMessages,
        ),
        (
            "{\"data\": \"Bad Request: bad webhook: HTTPS url must be provided for webhook\"}",
            ApiError::WebhookRequireHttps,
        ),
        (
            "{\"data\": \"Bad Request: bad webhook: Webhook can be set up only on ports 80, 88, \
             443 or 8443\"}",
            ApiError::BadWebhookPort,
        ),
        (
            "{\"data\": \"Bad Request: bad webhook: Failed to resolve host: Name or service not \
             known\"}",
            ApiError::UnknownHost,
        ),
        ("{\"data\": \"Bad Request: can't parse URL\"}", ApiError::CantParseUrl),
        (
            "{\"data\": \"Bad Request: can't parse entities\"}",
            ApiError::CantParseEntities(
                "Bad Request: can't parse entities: SomeRandomString".to_owned(),
            ),
        ),
        (
            "{\"data\": \"can't use getUpdates method while webhook is active\"}",
            ApiError::CantGetUpdates,
        ),
        ("{\"data\": \"Unauthorized: bot was kicked from a chat\"}", ApiError::BotKicked),
        (
            "{\"data\": \"Forbidden: bot was kicked from the supergroup chat\"}",
            ApiError::BotKickedFromSupergroup,
        ),
        ("{\"data\": \"Forbidden: user is deactivated\"}", ApiError::UserDeactivated),
        (
            "{\"data\": \"Unauthorized: bot can't initiate conversation with a user\"}",
            ApiError::CantInitiateConversation,
        ),
        (
            "{\"data\": \"Unauthorized: bot can't send messages to bots\"}",
            ApiError::CantTalkWithBots,
        ),
        ("{\"data\": \"Bad Request: wrong HTTP URL\"}", ApiError::WrongHttpUrl),
        (
            "{\"data\": \"Conflict: terminated by other getUpdates request; make sure that only \
             one bot instance is running\"}",
            ApiError::TerminatedByOtherGetUpdates,
        ),
        ("{\"data\": \"Bad Request: invalid file id\"}", ApiError::FileIdInvalid),
        ("{\"data\": \"Request Entity Too Large\"}", ApiError::RequestEntityTooLarge),
        ("{\"data\": \"RandomError\"}", ApiError::Unknown("RandomError".to_string())),
    ];

    #[derive(Deserialize, Debug)]
    struct Res<T> {
        data: T,
    }

    for (data, expected) in cases {
        let raw = serde_json::from_str::<Res<String>>(data).unwrap().data;
        let parsed = serde_json::from_str::<Res<ApiError>>(data).unwrap().data;
        assert_eq!(&parsed, expected);

        let expected_error_message = match parsed {
            ApiError::Unknown(_) => {
                format!("Unknown error: \"{raw}\"")
            }
            _ => raw,
        };
        assert_eq!(parsed.to_string(), expected_error_message);
    }
}

