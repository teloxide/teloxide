use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::types::{
    Animation, Audio, CopyTextButton, DisabledButton, Document, Location, LoginUrl, PhotoSize,
    SwitchInlineQueryChosenChat, User, Video, Voice, WebAppInfo,
};

/// A rich-formatted message returned by Telegram.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessage {
    pub blocks: Vec<RichBlock>,
    pub is_rtl: Option<bool>,
}

/// A button embedded in a rich message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessageButton {
    pub text: RichText,
    pub style: Option<String>,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub login_url: Option<LoginUrl>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub copy_text: Option<CopyTextButton>,
    pub disabled: Option<DisabledButton>,
}

impl RichMessageButton {
    /// Creates a rich-message button without an action.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            style: None,
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            disabled: None,
        }
    }

    /// Creates a rich-message callback button.
    pub fn callback(text: impl Into<RichText>, callback_data: impl Into<String>) -> Self {
        let mut button = Self::new(text);
        button.callback_data = Some(callback_data.into());
        button
    }

    /// Creates a rich-message URL button.
    pub fn url(text: impl Into<RichText>, url: impl Into<String>) -> Self {
        let mut button = Self::new(text);
        button.url = Some(url.into());
        button
    }

    /// Creates a disabled rich-message button.
    pub fn disabled(text: impl Into<RichText>) -> Self {
        let mut button = Self::new(text);
        button.disabled = Some(DisabledButton {});
        button
    }

    /// Sets the native button style introduced by Bot API 10.3.
    #[must_use]
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
}

/// Rich-formatted inline text.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RichText {
    Text(String),
    List(Vec<Self>),
    Object(RichTextObject),
}

impl<'de> Deserialize<'de> for RichText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(text) => Ok(Self::Text(text)),
            Value::Array(_) => {
                serde_json::from_value(value).map(Self::List).map_err(de::Error::custom)
            }
            Value::Object(object) => {
                let type_name = object
                    .get("type")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| de::Error::missing_field("type"))?;
                let value = Value::Object(object);
                if is_known_rich_text_type(&type_name) {
                    serde_json::from_value(value).map(Self::Object).map_err(de::Error::custom)
                } else {
                    Ok(Self::Object(RichTextObject::Unknown(value)))
                }
            }
            _ => Err(de::Error::custom("rich text must be a string, array, or object")),
        }
    }
}

impl From<String> for RichText {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<Vec<Self>> for RichText {
    fn from(value: Vec<Self>) -> Self {
        Self::List(value)
    }
}

/// A typed rich-text object.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum RichTextObject {
    Bold(RichTextBold),
    Italic(RichTextItalic),
    Underline(RichTextUnderline),
    Strikethrough(RichTextStrikethrough),
    Spoiler(RichTextSpoiler),
    DateTime(RichTextDateTime),
    TextMention(RichTextTextMention),
    Subscript(RichTextSubscript),
    Superscript(RichTextSuperscript),
    Marked(RichTextMarked),
    Code(RichTextCode),
    CustomEmoji(RichTextCustomEmoji),
    MathematicalExpression(RichTextMathematicalExpression),
    Url(RichTextUrl),
    EmailAddress(RichTextEmailAddress),
    PhoneNumber(RichTextPhoneNumber),
    BankCardNumber(RichTextBankCardNumber),
    Mention(RichTextMention),
    Hashtag(RichTextHashtag),
    Cashtag(RichTextCashtag),
    BotCommand(RichTextBotCommand),
    Button(RichTextButton),
    Anchor(RichTextAnchor),
    AnchorLink(RichTextAnchorLink),
    Reference(RichTextReference),
    ReferenceLink(RichTextReferenceLink),
    /// A future rich-text object preserved as its original JSON value.
    Unknown(Value),
}

fn is_known_rich_text_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "bold"
            | "italic"
            | "underline"
            | "strikethrough"
            | "spoiler"
            | "date_time"
            | "text_mention"
            | "subscript"
            | "superscript"
            | "marked"
            | "code"
            | "custom_emoji"
            | "mathematical_expression"
            | "url"
            | "email_address"
            | "phone_number"
            | "bank_card_number"
            | "mention"
            | "hashtag"
            | "cashtag"
            | "bot_command"
            | "button"
            | "anchor"
            | "anchor_link"
            | "reference"
            | "reference_link"
    )
}

impl<'de> Deserialize<'de> for RichTextObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let type_name = value
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| de::Error::missing_field("type"))?;
        if is_known_rich_text_type(type_name) {
            let known =
                serde_json::from_value::<KnownRichTextObject>(value).map_err(de::Error::custom)?;
            Ok(known.into())
        } else {
            Ok(Self::Unknown(value))
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum KnownRichTextObject {
    Bold(RichTextBold),
    Italic(RichTextItalic),
    Underline(RichTextUnderline),
    Strikethrough(RichTextStrikethrough),
    Spoiler(RichTextSpoiler),
    DateTime(RichTextDateTime),
    TextMention(RichTextTextMention),
    Subscript(RichTextSubscript),
    Superscript(RichTextSuperscript),
    Marked(RichTextMarked),
    Code(RichTextCode),
    CustomEmoji(RichTextCustomEmoji),
    MathematicalExpression(RichTextMathematicalExpression),
    Url(RichTextUrl),
    EmailAddress(RichTextEmailAddress),
    PhoneNumber(RichTextPhoneNumber),
    BankCardNumber(RichTextBankCardNumber),
    Mention(RichTextMention),
    Hashtag(RichTextHashtag),
    Cashtag(RichTextCashtag),
    BotCommand(RichTextBotCommand),
    Button(RichTextButton),
    Anchor(RichTextAnchor),
    AnchorLink(RichTextAnchorLink),
    Reference(RichTextReference),
    ReferenceLink(RichTextReferenceLink),
}

impl From<KnownRichTextObject> for RichTextObject {
    fn from(value: KnownRichTextObject) -> Self {
        match value {
            KnownRichTextObject::Bold(value) => Self::Bold(value),
            KnownRichTextObject::Italic(value) => Self::Italic(value),
            KnownRichTextObject::Underline(value) => Self::Underline(value),
            KnownRichTextObject::Strikethrough(value) => Self::Strikethrough(value),
            KnownRichTextObject::Spoiler(value) => Self::Spoiler(value),
            KnownRichTextObject::DateTime(value) => Self::DateTime(value),
            KnownRichTextObject::TextMention(value) => Self::TextMention(value),
            KnownRichTextObject::Subscript(value) => Self::Subscript(value),
            KnownRichTextObject::Superscript(value) => Self::Superscript(value),
            KnownRichTextObject::Marked(value) => Self::Marked(value),
            KnownRichTextObject::Code(value) => Self::Code(value),
            KnownRichTextObject::CustomEmoji(value) => Self::CustomEmoji(value),
            KnownRichTextObject::MathematicalExpression(value) => {
                Self::MathematicalExpression(value)
            }
            KnownRichTextObject::Url(value) => Self::Url(value),
            KnownRichTextObject::EmailAddress(value) => Self::EmailAddress(value),
            KnownRichTextObject::PhoneNumber(value) => Self::PhoneNumber(value),
            KnownRichTextObject::BankCardNumber(value) => Self::BankCardNumber(value),
            KnownRichTextObject::Mention(value) => Self::Mention(value),
            KnownRichTextObject::Hashtag(value) => Self::Hashtag(value),
            KnownRichTextObject::Cashtag(value) => Self::Cashtag(value),
            KnownRichTextObject::BotCommand(value) => Self::BotCommand(value),
            KnownRichTextObject::Button(value) => Self::Button(value),
            KnownRichTextObject::Anchor(value) => Self::Anchor(value),
            KnownRichTextObject::AnchorLink(value) => Self::AnchorLink(value),
            KnownRichTextObject::Reference(value) => Self::Reference(value),
            KnownRichTextObject::ReferenceLink(value) => Self::ReferenceLink(value),
        }
    }
}

fn serialize_rich_text_object<S, T>(
    serializer: S,
    type_name: &'static str,
    value: &T,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut object = serde_json::to_value(value).map_err(serde::ser::Error::custom)?;
    let map = object
        .as_object_mut()
        .ok_or_else(|| serde::ser::Error::custom("rich-text object must serialize to an object"))?;
    map.insert("type".to_owned(), Value::String(type_name.to_owned()));
    object.serialize(serializer)
}

impl Serialize for RichTextObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bold(value) => serialize_rich_text_object(serializer, "bold", value),
            Self::Italic(value) => serialize_rich_text_object(serializer, "italic", value),
            Self::Underline(value) => serialize_rich_text_object(serializer, "underline", value),
            Self::Strikethrough(value) => {
                serialize_rich_text_object(serializer, "strikethrough", value)
            }
            Self::Spoiler(value) => serialize_rich_text_object(serializer, "spoiler", value),
            Self::DateTime(value) => serialize_rich_text_object(serializer, "date_time", value),
            Self::TextMention(value) => {
                serialize_rich_text_object(serializer, "text_mention", value)
            }
            Self::Subscript(value) => serialize_rich_text_object(serializer, "subscript", value),
            Self::Superscript(value) => {
                serialize_rich_text_object(serializer, "superscript", value)
            }
            Self::Marked(value) => serialize_rich_text_object(serializer, "marked", value),
            Self::Code(value) => serialize_rich_text_object(serializer, "code", value),
            Self::CustomEmoji(value) => {
                serialize_rich_text_object(serializer, "custom_emoji", value)
            }
            Self::MathematicalExpression(value) => {
                serialize_rich_text_object(serializer, "mathematical_expression", value)
            }
            Self::Url(value) => serialize_rich_text_object(serializer, "url", value),
            Self::EmailAddress(value) => {
                serialize_rich_text_object(serializer, "email_address", value)
            }
            Self::PhoneNumber(value) => {
                serialize_rich_text_object(serializer, "phone_number", value)
            }
            Self::BankCardNumber(value) => {
                serialize_rich_text_object(serializer, "bank_card_number", value)
            }
            Self::Mention(value) => serialize_rich_text_object(serializer, "mention", value),
            Self::Hashtag(value) => serialize_rich_text_object(serializer, "hashtag", value),
            Self::Cashtag(value) => serialize_rich_text_object(serializer, "cashtag", value),
            Self::BotCommand(value) => serialize_rich_text_object(serializer, "bot_command", value),
            Self::Button(value) => serialize_rich_text_object(serializer, "button", value),
            Self::Anchor(value) => serialize_rich_text_object(serializer, "anchor", value),
            Self::AnchorLink(value) => serialize_rich_text_object(serializer, "anchor_link", value),
            Self::Reference(value) => serialize_rich_text_object(serializer, "reference", value),
            Self::ReferenceLink(value) => {
                serialize_rich_text_object(serializer, "reference_link", value)
            }
            Self::Unknown(value) => value.serialize(serializer),
        }
    }
}

macro_rules! rich_text_from {
    ($type:ident, $variant:ident) => {
        impl From<$type> for RichText {
            fn from(value: $type) -> Self {
                Self::Object(RichTextObject::$variant(value))
            }
        }
    };
}

macro_rules! rich_text_wrapper {
    ($($type:ident => $variant:ident),+ $(,)?) => {
        $(
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[cfg_attr(test, derive(schemars::JsonSchema))]
            pub struct $type {
                pub text: Box<RichText>,
            }

            impl $type {
                pub fn new(text: impl Into<RichText>) -> Self {
                    Self { text: Box::new(text.into()) }
                }
            }

            rich_text_from!($type, $variant);
        )+
    };
}

rich_text_wrapper! {
    RichTextBold => Bold,
    RichTextItalic => Italic,
    RichTextUnderline => Underline,
    RichTextStrikethrough => Strikethrough,
    RichTextSpoiler => Spoiler,
    RichTextSubscript => Subscript,
    RichTextSuperscript => Superscript,
    RichTextMarked => Marked,
    RichTextCode => Code,
}

macro_rules! rich_text_link {
    ($type:ident, $variant:ident, $field:ident) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(test, derive(schemars::JsonSchema))]
        pub struct $type {
            pub text: Box<RichText>,
            pub $field: String,
        }

        rich_text_from!($type, $variant);
    };
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextDateTime {
    pub text: Box<RichText>,
    pub unix_time: i64,
    pub date_time_format: String,
}
rich_text_from!(RichTextDateTime, DateTime);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextTextMention {
    pub text: Box<RichText>,
    pub user: User,
}
rich_text_from!(RichTextTextMention, TextMention);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCustomEmoji {
    pub custom_emoji_id: String,
    pub alternative_text: String,
}
rich_text_from!(RichTextCustomEmoji, CustomEmoji);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMathematicalExpression {
    pub expression: String,
}
rich_text_from!(RichTextMathematicalExpression, MathematicalExpression);

rich_text_link!(RichTextUrl, Url, url);
rich_text_link!(RichTextEmailAddress, EmailAddress, email_address);
rich_text_link!(RichTextPhoneNumber, PhoneNumber, phone_number);
rich_text_link!(RichTextBankCardNumber, BankCardNumber, bank_card_number);
rich_text_link!(RichTextMention, Mention, username);
rich_text_link!(RichTextHashtag, Hashtag, hashtag);
rich_text_link!(RichTextCashtag, Cashtag, cashtag);
rich_text_link!(RichTextBotCommand, BotCommand, bot_command);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextButton {
    pub button: Box<RichMessageButton>,
}
rich_text_from!(RichTextButton, Button);

rich_text_link!(RichTextAnchorLink, AnchorLink, anchor_name);
rich_text_link!(RichTextReference, Reference, name);
rich_text_link!(RichTextReferenceLink, ReferenceLink, reference_name);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextAnchor {
    pub name: String,
}
rich_text_from!(RichTextAnchor, Anchor);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCaption {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTableCell {
    pub text: Option<RichText>,
    pub is_header: Option<bool>,
    pub colspan: Option<u32>,
    pub rowspan: Option<u32>,
    pub align: String,
    pub valign: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockListItem {
    pub label: String,
    pub blocks: Vec<RichBlock>,
    pub has_checkbox: Option<bool>,
    pub is_checked: Option<bool>,
    pub value: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

/// A rich block returned by Telegram.
///
/// Unknown block variants are preserved instead of failing deserialization of
/// the containing update.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum RichBlock {
    /// A block whose type is supported by this version of teloxide.
    Known(Box<RichBlockKind>),
    /// A future Telegram block type preserved as its original JSON value.
    Unknown(Value),
}

impl Serialize for RichBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Known(value) => value.serialize(serializer),
            Self::Unknown(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for RichBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let type_name = value
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| de::Error::missing_field("type"))?;

        match type_name {
            "paragraph"
            | "heading"
            | "pre"
            | "footer"
            | "divider"
            | "mathematical_expression"
            | "anchor"
            | "list"
            | "blockquote"
            | "expandable_blockquote"
            | "pullquote"
            | "collage"
            | "slideshow"
            | "table"
            | "buttons"
            | "document"
            | "details"
            | "map"
            | "animation"
            | "audio"
            | "photo"
            | "video"
            | "voice_note"
            | "thinking" => serde_json::from_value::<RichBlockKind>(value)
                .map(|value| Self::Known(Box::new(value)))
                .map_err(de::Error::custom),
            _ => Ok(Self::Unknown(value)),
        }
    }
}

impl From<RichBlockKind> for RichBlock {
    fn from(value: RichBlockKind) -> Self {
        Self::Known(Box::new(value))
    }
}

/// The currently known rich-block variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichBlockKind {
    Paragraph(RichBlockParagraph),
    Heading(RichBlockSectionHeading),
    Pre(RichBlockPreformatted),
    Footer(RichBlockFooter),
    Divider(RichBlockDivider),
    MathematicalExpression(RichBlockMathematicalExpression),
    Anchor(RichBlockAnchor),
    List(RichBlockList),
    Blockquote(RichBlockBlockQuotation),
    ExpandableBlockquote(RichBlockExpandableBlockQuotation),
    Pullquote(RichBlockPullQuotation),
    Collage(RichBlockCollage),
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Buttons(RichBlockButtons),
    Document(RichBlockDocument),
    Details(RichBlockDetails),
    Map(RichBlockMap),
    Animation(RichBlockAnimation),
    Audio(RichBlockAudio),
    Photo(RichBlockPhoto),
    Video(RichBlockVideo),
    VoiceNote(RichBlockVoiceNote),
    Thinking(RichBlockThinking),
}

macro_rules! rich_text_block {
    ($($type:ident),+ $(,)?) => {
        $(
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[cfg_attr(test, derive(schemars::JsonSchema))]
            pub struct $type {
                pub text: RichText,
            }
        )+
    };
}

rich_text_block!(RichBlockParagraph, RichBlockFooter, RichBlockThinking);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSectionHeading {
    pub text: RichText,
    pub size: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPreformatted {
    pub text: RichText,
    pub language: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDivider {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMathematicalExpression {
    pub expression: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnchor {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockList {
    pub items: Vec<RichBlockListItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockBlockQuotation {
    pub blocks: Vec<RichBlock>,
    pub credit: Option<RichText>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockExpandableBlockQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPullQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCollage {
    pub blocks: Vec<RichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSlideshow {
    pub blocks: Vec<RichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTable {
    pub cells: Vec<Vec<RichBlockTableCell>>,
    pub is_bordered: Option<bool>,
    pub is_striped: Option<bool>,
    pub is_compact: Option<bool>,
    pub caption: Option<RichText>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockButtons {
    pub buttons: Vec<RichMessageButton>,
    pub align: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDocument {
    pub document: Document,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDetails {
    pub summary: RichText,
    pub blocks: Vec<RichBlock>,
    pub is_open: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMap {
    pub location: Location,
    pub zoom: u8,
    pub width: u32,
    pub height: u32,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnimation {
    pub animation: Animation,
    pub has_spoiler: Option<bool>,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAudio {
    pub audio: Audio,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPhoto {
    pub photo: Vec<PhotoSize>,
    pub has_spoiler: Option<bool>,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVideo {
    pub video: Video,
    pub has_spoiler: Option<bool>,
    pub caption: Option<RichBlockCaption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVoiceNote {
    pub voice_note: Voice,
    pub caption: Option<RichBlockCaption>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_block_is_preserved() {
        let value = serde_json::json!({"type": "future_block", "x": 1});
        let block: RichBlock = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(block, RichBlock::Unknown(value));
    }

    #[test]
    fn known_block_roundtrips_wire_shape() {
        let raw = serde_json::json!({"type": "paragraph", "text": "hello"});
        let block: RichBlock = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(block).unwrap(), raw);
    }

    #[test]
    fn unknown_block_roundtrips_wire_shape() {
        let raw = serde_json::json!({"type": "future_block", "x": 1});
        let block: RichBlock = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(block).unwrap(), raw);
    }

    #[test]
    fn known_rich_text_object_roundtrips_wire_shape() {
        let raw = serde_json::json!({"type": "bold", "text": "hello"});
        let text: RichText = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(text).unwrap(), raw);
    }

    #[test]
    fn unknown_rich_text_object_is_preserved() {
        let raw = serde_json::json!({"type": "future_text", "text": "hello"});
        let text: RichText = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(text).unwrap(), raw);
    }

    #[test]
    fn malformed_known_rich_text_object_is_rejected() {
        let result = serde_json::from_value::<RichText>(serde_json::json!({
            "type": "bold"
        }));
        assert!(result.is_err());
    }

    #[test]
    fn malformed_known_block_is_rejected() {
        let result = serde_json::from_value::<RichBlock>(serde_json::json!({
            "type": "paragraph"
        }));
        assert!(result.is_err());
    }

    #[test]
    fn known_block_is_typed() {
        let block: RichBlock = serde_json::from_value(serde_json::json!({
            "type": "paragraph",
            "text": "hello"
        }))
        .unwrap();
        assert!(matches!(block, RichBlock::Known(_)));
    }
}
