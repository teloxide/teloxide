use crate::types::{
    InputFile, InputFileLike, InputMediaAnimation, InputMediaAudio, InputMediaDocument,
    InputMediaPhoto, InputMediaVideo, Location, MessageEntity, ParseMode, RichBlockCaption,
    RichBlockTableCell, RichMessageButton, RichText,
};
use serde::Serialize;

/// Describes a rich message to be sent.
///
/// The source fields are private, so every constructor selects exactly one of
/// HTML, Markdown, or typed blocks. Method-specific static constraints can be
/// checked with [`ValidateWith<RichMessageContext>`] before dispatch. This
/// validation does not prove that Telegram will accept permissions or other
/// server-side capabilities.
///
/// [`ValidateWith<RichMessageContext>`]: crate::requests::ValidateWith
/// [`RichMessageContext`]: crate::requests::RichMessageContext
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessage {
    blocks: Option<Vec<InputRichBlock>>,
    html: Option<String>,
    markdown: Option<String>,
    /// Media referenced by `tg://photo`, `tg://video`, `tg://audio`, or
    /// `tg://document` links.
    pub media: Option<Vec<InputRichMessageMedia>>,
    /// Show the rich message right-to-left.
    pub is_rtl: Option<bool>,
    /// Skip automatic entity detection.
    pub skip_entity_detection: Option<bool>,
}

impl InputRichMessage {
    /// Creates a rich message from HTML.
    pub fn html(content: impl Into<String>) -> Self {
        Self {
            blocks: None,
            html: Some(content.into()),
            markdown: None,
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Creates a rich message from Markdown.
    pub fn markdown(content: impl Into<String>) -> Self {
        Self {
            blocks: None,
            html: None,
            markdown: Some(content.into()),
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Creates a rich message from typed blocks.
    ///
    /// Files nested in media blocks are collected for multipart uploads.
    pub fn blocks(blocks: impl IntoIterator<Item = InputRichBlock>) -> Self {
        Self {
            blocks: Some(blocks.into_iter().collect()),
            html: None,
            markdown: None,
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Sets media referenced by rich-message links.
    pub fn media(mut self, media: impl IntoIterator<Item = InputRichMessageMedia>) -> Self {
        self.media = Some(media.into_iter().collect());
        self
    }

    /// Sets right-to-left display.
    pub const fn rtl(mut self, is_rtl: bool) -> Self {
        self.is_rtl = Some(is_rtl);
        self
    }

    /// Controls automatic entity detection.
    pub const fn skip_entity_detection(mut self, skip: bool) -> Self {
        self.skip_entity_detection = Some(skip);
        self
    }

    fn files(&self) -> impl Iterator<Item = &InputFile> {
        self.media.iter().flatten().flat_map(InputRichMessageMedia::files)
    }

    fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        self.media.iter_mut().flatten().flat_map(InputRichMessageMedia::files_mut)
    }

    /// Returns the block source, if this message was constructed from blocks.
    pub fn blocks_ref(&self) -> Option<&[InputRichBlock]> {
        self.blocks.as_deref()
    }

    /// Returns the HTML source, if this message was constructed from HTML.
    pub fn html_ref(&self) -> Option<&str> {
        self.html.as_deref()
    }

    /// Returns the Markdown source, if this message was constructed from
    /// Markdown.
    pub fn markdown_ref(&self) -> Option<&str> {
        self.markdown.as_deref()
    }
}

impl InputFileLike for InputRichMessage {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.files().for_each(|file| file.copy_into(into));
        self.blocks.copy_into(into);
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.files_mut().for_each(|file| file.move_into(into));
        self.blocks.move_into(into);
    }
}

/// A media element embedded in an outgoing rich message.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessageMedia {
    /// Identifier used by a `tg://photo`, `tg://video`, `tg://audio`, or
    /// `tg://document` link.
    pub id: String,
    /// The referenced media.
    pub media: InputRichMessageMediaContent,
}

impl InputRichMessageMedia {
    pub fn new(id: impl Into<String>, media: InputRichMessageMediaContent) -> Self {
        Self { id: id.into(), media }
    }

    fn files(&self) -> impl Iterator<Item = &InputFile> {
        self.media.files()
    }

    fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        self.media.files_mut()
    }
}

/// Media kinds supported inside rich messages.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichMessageMediaContent {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
    VoiceNote(InputMediaVoiceNote),
}

impl InputRichMessageMediaContent {
    fn files(&self) -> impl Iterator<Item = &InputFile> {
        let mut files = Vec::new();
        match self {
            Self::Photo(media) => files.push(&media.media),
            Self::Audio(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
            }
            Self::Document(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
            }
            Self::Animation(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
            }
            Self::Video(media) => {
                files.push(&media.media);
                files.extend(media.thumbnail.iter());
                files.extend(media.cover.iter());
            }
            Self::VoiceNote(media) => files.push(&media.media),
        }
        files.into_iter()
    }

    fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        let mut files = Vec::new();
        match self {
            Self::Photo(media) => files.push(&mut media.media),
            Self::Audio(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
            }
            Self::Document(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
            }
            Self::Animation(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
            }
            Self::Video(media) => {
                files.push(&mut media.media);
                files.extend(media.thumbnail.iter_mut());
                files.extend(media.cover.iter_mut());
            }
            Self::VoiceNote(media) => files.push(&mut media.media),
        }
        files.into_iter()
    }
}

/// A voice-note media element used in a rich message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaVoiceNote {
    pub media: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<u16>,
}

impl InputMediaVoiceNote {
    pub const fn new(media: InputFile) -> Self {
        Self { media, caption: None, parse_mode: None, caption_entities: None, duration: None }
    }

    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub const fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn caption_entities(mut self, entities: impl IntoIterator<Item = MessageEntity>) -> Self {
        self.caption_entities = Some(entities.into_iter().collect());
        self
    }

    pub const fn duration(mut self, duration: u16) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl InputFileLike for InputMediaVoiceNote {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.copy_into(into);
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.move_into(into);
    }
}

/// A block in an outgoing rich message.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichBlock {
    Paragraph(InputRichBlockParagraph),
    Heading(InputRichBlockSectionHeading),
    Pre(InputRichBlockPreformatted),
    Footer(InputRichBlockFooter),
    Divider(InputRichBlockDivider),
    MathematicalExpression(InputRichBlockMathematicalExpression),
    Anchor(InputRichBlockAnchor),
    List(InputRichBlockList),
    Blockquote(InputRichBlockBlockQuotation),
    ExpandableBlockquote(InputRichBlockExpandableBlockQuotation),
    Pullquote(InputRichBlockPullQuotation),
    Collage(InputRichBlockCollage),
    Slideshow(InputRichBlockSlideshow),
    Table(InputRichBlockTable),
    Buttons(InputRichBlockButtons),
    Document(InputRichBlockDocument),
    Details(InputRichBlockDetails),
    Map(InputRichBlockMap),
    Animation(InputRichBlockAnimation),
    Audio(InputRichBlockAudio),
    Photo(InputRichBlockPhoto),
    Video(InputRichBlockVideo),
    VoiceNote(InputRichBlockVoiceNote),
    /// Only valid when sending a rich message draft.
    Thinking(InputRichBlockThinking),
}

impl InputFileLike for InputRichBlock {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::List(value) => value.items.copy_into(into),
            Self::Blockquote(value) => value.blocks.copy_into(into),
            Self::ExpandableBlockquote(_) => {}
            Self::Collage(value) => value.blocks.copy_into(into),
            Self::Slideshow(value) => value.blocks.copy_into(into),
            Self::Details(value) => value.blocks.copy_into(into),
            Self::Buttons(_) => {}
            Self::Document(value) => value.document.copy_into(into),
            Self::Animation(value) => value.animation.copy_into(into),
            Self::Audio(value) => value.audio.copy_into(into),
            Self::Photo(value) => value.photo.copy_into(into),
            Self::Video(value) => value.video.copy_into(into),
            Self::VoiceNote(value) => value.voice_note.copy_into(into),
            Self::Paragraph(_)
            | Self::Heading(_)
            | Self::Pre(_)
            | Self::Footer(_)
            | Self::Divider(_)
            | Self::MathematicalExpression(_)
            | Self::Anchor(_)
            | Self::Pullquote(_)
            | Self::Table(_)
            | Self::Map(_)
            | Self::Thinking(_) => {}
        }
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::List(value) => value.items.move_into(into),
            Self::Blockquote(value) => value.blocks.move_into(into),
            Self::ExpandableBlockquote(_) => {}
            Self::Collage(value) => value.blocks.move_into(into),
            Self::Slideshow(value) => value.blocks.move_into(into),
            Self::Details(value) => value.blocks.move_into(into),
            Self::Buttons(_) => {}
            Self::Document(value) => value.document.move_into(into),
            Self::Animation(value) => value.animation.move_into(into),
            Self::Audio(value) => value.audio.move_into(into),
            Self::Photo(value) => value.photo.move_into(into),
            Self::Video(value) => value.video.move_into(into),
            Self::VoiceNote(value) => value.voice_note.move_into(into),
            Self::Paragraph(_)
            | Self::Heading(_)
            | Self::Pre(_)
            | Self::Footer(_)
            | Self::Divider(_)
            | Self::MathematicalExpression(_)
            | Self::Anchor(_)
            | Self::Pullquote(_)
            | Self::Table(_)
            | Self::Map(_)
            | Self::Thinking(_) => {}
        }
    }
}

macro_rules! input_rich_text_block {
    ($($type:ident),+ $(,)?) => {
        $(
            #[derive(Clone, Debug, PartialEq, Serialize)]
            #[cfg_attr(test, derive(schemars::JsonSchema))]
            pub struct $type {
                pub text: RichText,
            }
        )+
    };
}

input_rich_text_block!(InputRichBlockParagraph, InputRichBlockFooter);

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockSectionHeading {
    pub text: RichText,
    pub size: u8,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPreformatted {
    pub text: RichText,
    pub language: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDivider {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMathematicalExpression {
    pub expression: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAnchor {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockList {
    pub items: Vec<InputRichBlockListItem>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockListItem {
    pub blocks: Vec<InputRichBlock>,
    pub has_checkbox: Option<bool>,
    pub is_checked: Option<bool>,
    pub value: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

impl InputFileLike for InputRichBlockListItem {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.copy_into(into);
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.blocks.move_into(into);
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockBlockQuotation {
    pub blocks: Vec<InputRichBlock>,
    pub credit: Option<RichText>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockExpandableBlockQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPullQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockCollage {
    pub blocks: Vec<InputRichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockSlideshow {
    pub blocks: Vec<InputRichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockTable {
    pub cells: Vec<Vec<RichBlockTableCell>>,
    pub is_bordered: Option<bool>,
    pub is_striped: Option<bool>,
    pub is_compact: Option<bool>,
    pub caption: Option<RichText>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockButtons {
    pub buttons: Vec<RichMessageButton>,
    pub align: Option<String>,
}

impl InputRichBlockButtons {
    /// Creates a rich-message buttons block.
    pub fn new(buttons: impl IntoIterator<Item = RichMessageButton>) -> Self {
        Self { buttons: buttons.into_iter().collect(), align: None }
    }

    /// Sets the native alignment for the buttons block.
    #[must_use]
    pub fn align(mut self, align: impl Into<String>) -> Self {
        self.align = Some(align.into());
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDocument {
    pub document: InputMediaDocument,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDetails {
    pub summary: RichText,
    pub blocks: Vec<InputRichBlock>,
    pub is_open: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMap {
    pub location: Location,
    pub zoom: Option<u8>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAnimation {
    pub animation: InputMediaAnimation,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAudio {
    pub audio: InputMediaAudio,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPhoto {
    pub photo: InputMediaPhoto,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockVideo {
    pub video: InputMediaVideo,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockVoiceNote {
    pub voice_note: InputMediaVoiceNote,
    pub caption: Option<RichBlockCaption>,
}

/// Only valid in `sendRichMessageDraft`.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockThinking {
    pub text: RichText,
}

/// Rich message content used by inline, guest, and Web App query results.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessageContent {
    /// Rich message payload.
    pub rich_message: InputRichMessage,
}

impl InputRichMessageContent {
    /// Wraps a rich message as inline-query message content.
    pub const fn new(rich_message: InputRichMessage) -> Self {
        Self { rich_message }
    }
}

impl InputFileLike for InputRichMessageContent {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.rich_message.copy_into(into);
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.rich_message.move_into(into);
    }
}

/// Result of a chat join request query.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ChatJoinRequestQueryResult {
    Approve,
    Decline,
    Queue,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FileId;

    #[test]
    fn rich_html_serializes_without_unused_variants() {
        let value = serde_json::to_value(
            InputRichMessage::html("<b>hello</b>").rtl(true).skip_entity_detection(true),
        )
        .unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "html": "<b>hello</b>",
                "is_rtl": true,
                "skip_entity_detection": true
            })
        );
    }

    #[test]
    fn optional_input_rich_block_fields_are_omitted() {
        let value = serde_json::to_value(InputRichMessage::blocks([
            InputRichBlock::Details(InputRichBlockDetails {
                summary: RichText::from("summary"),
                blocks: Vec::new(),
                is_open: None,
            }),
            InputRichBlock::Map(InputRichBlockMap {
                location: Location {
                    longitude: 0.0,
                    latitude: 0.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None,
                },
                zoom: None,
                width: None,
                height: None,
                caption: None,
            }),
        ]))
        .unwrap();

        assert_eq!(
            value["blocks"][0],
            serde_json::json!({
                "type": "details",
                "summary": "summary",
                "blocks": []
            })
        );
        assert_eq!(
            value["blocks"][1],
            serde_json::json!({
                "type": "map",
                "location": {"longitude": 0.0, "latitude": 0.0}
            })
        );
    }

    #[test]
    fn nested_media_files_are_collected() {
        let message = InputRichMessage::blocks([InputRichBlock::Details(InputRichBlockDetails {
            summary: RichText::from("files"),
            blocks: vec![InputRichBlock::Video(InputRichBlockVideo {
                video: InputMediaVideo::new(InputFile::memory("video"))
                    .thumbnail(InputFile::memory("thumbnail"))
                    .cover(InputFile::memory("cover")),
                caption: None,
            })],
            is_open: None,
        })]);

        let value = serde_json::to_value(&message).unwrap();
        assert!(value["blocks"][0]["blocks"][0]["video"]["media"]
            .as_str()
            .unwrap()
            .starts_with("attach://"));

        let mut files = Vec::new();
        message.copy_into(&mut |file| files.push(file));
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn voice_note_media_uses_official_type_tag() {
        let media = InputRichMessageMedia::new(
            "voice",
            InputRichMessageMediaContent::VoiceNote(InputMediaVoiceNote::new(InputFile::file_id(
                FileId("voice-file".to_owned()),
            ))),
        );
        let value = serde_json::to_value(media).unwrap();
        assert_eq!(value["id"], "voice");
        assert_eq!(value["media"]["type"], "voice_note");
        assert_eq!(value["media"]["media"], "voice-file");
    }

    #[test]
    fn document_media_collects_media_and_thumbnail_files() {
        let media = InputRichMessageMedia::new(
            "document",
            InputRichMessageMediaContent::Document(
                InputMediaDocument::new(InputFile::memory("document"))
                    .thumbnail(InputFile::memory("thumbnail")),
            ),
        );
        let message =
            InputRichMessage::html(r#"<img src="tg://document?id=document">"#).media([media]);
        let mut files = Vec::new();
        message.copy_into(&mut |file| files.push(file));
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn message_generation_stopped_accepts_numeric_and_string_draft_ids() {
        let chat = serde_json::json!({"id": 7, "type": "private", "first_name": "User"});
        for draft_id in [serde_json::json!(42), serde_json::json!("42")] {
            let value = serde_json::from_value::<crate::types::MessageGenerationStopped>(
                serde_json::json!({"chat": chat.clone(), "draft_id": draft_id}),
            )
            .unwrap();
            assert_eq!(value.draft_id, 42);
        }
    }
}
