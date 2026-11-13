//! Native HTML widgets and their query protocol.
use serde::{Deserialize, Serialize};

use crate::types::{Chat, MessageEntity, MessageId, ParseMode, UniqueGift, User};

/// An immutable HTML document with mutable message and shared state.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Widget {
    pub html: String,
    pub id: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub public_data: Option<String>,
    pub chat_data: Option<String>,
}

/// A query from a widget in an original message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct WidgetQuery {
    pub id: String,
    pub from: User,
    pub chat: Option<Chat>,
    #[serde(default, with = "crate::types::option_msg_id_as_int")]
    #[cfg_attr(test, schemars(with = "Option<i32>"))]
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub chat_type: Option<WidgetChatType>,
    #[serde(rename = "type")]
    pub kind: WidgetQueryType,
    pub data: String,
    pub widget_id: Option<String>,
}

/// A query from a forwarded shared widget. Its destination chat is not exposed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SharedWidgetQuery {
    pub id: String,
    pub from: User,
    #[serde(rename = "type")]
    pub kind: WidgetQueryType,
    pub data: String,
    pub widget_id: String,
}

/// Chat type reported for a widget sent through inline mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum WidgetChatType {
    Sender,
    Private,
    Group,
    Supergroup,
    Channel,
}

/// Data, wallet signature, or transaction notification carried by a query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WidgetQueryType {
    Data { wallet_card: Option<TonWalletCard> },
    SignData { wallet: WidgetQueryWallet, sign_data: String },
    SentTransaction { wallet: WidgetQueryWallet, msg_hash: String },
}

/// Wallet identity carried by a wallet query. Amount is in smallest units.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct WidgetQueryWallet {
    pub address: String,
    pub amount: String,
    pub public_key: Option<String>,
    pub state_init: Option<String>,
}

/// The public TON card shown on a user's profile.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct TonWalletCard {
    pub address: String,
    pub amount: String,
    pub stickers: Vec<TonWalletCardSticker>,
    pub background_nft_address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct TonWalletCardSticker {
    pub position: AreaPosition,
    pub nft: TonNft,
}

/// Position and dimensions as percentages of the media area.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct AreaPosition {
    pub x_percentage: f64,
    pub y_percentage: f64,
    pub width_percentage: f64,
    pub height_percentage: f64,
    pub rotation_angle: f64,
    pub corner_radius_percentage: f64,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct TonNft {
    pub address: String,
    pub collection_address: Option<String>,
    pub index: String,
    pub name: String,
    pub description: String,
    pub attributes: Option<Vec<TonNftAttribute>>,
    pub gift: Option<UniqueGift>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct TonNftAttribute {
    pub trait_type: String,
    pub value: String,
}

/// A new widget offered as inline message content. Each shared send creates a
/// separate shared object; use `InputSharedWidgetMessageContent` to reuse one.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputWidgetMessageContent {
    pub html: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub public_data: Option<String>,
    pub chat_data: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub allow_queries: Option<bool>,
    pub is_shared: Option<bool>,
}

impl InputWidgetMessageContent {
    pub fn new(html: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            width: None,
            height: None,
            public_data: None,
            chat_data: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            allow_queries: None,
            is_shared: None,
        }
    }
}

/// Inline message content referring to an existing shared widget.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputSharedWidgetMessageContent {
    pub widget_id: String,
    pub chat_data: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

impl InputSharedWidgetMessageContent {
    pub fn new(widget_id: impl Into<String>) -> Self {
        Self {
            widget_id: widget_id.into(),
            chat_data: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
        }
    }
}
