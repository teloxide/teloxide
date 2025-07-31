use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(with = "String"))]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    BusinessConnection,
    BusinessMessage,
    EditedBusinessMessage,
    DeletedBusinessMessages,
    MessageReaction,
    MessageReactionCount,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    PurchasedPaidMedia,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
    ChatBoost,
    RemovedChatBoost,
}
