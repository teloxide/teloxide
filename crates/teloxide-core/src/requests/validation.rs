use std::fmt;

/// A segment in a [`RequestFieldPath`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RequestFieldPathSegment {
    /// A named request field.
    Field(&'static str),
    /// An item in a request collection.
    Index(usize),
}

/// A structured path to a request field.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct RequestFieldPath {
    segments: Vec<RequestFieldPathSegment>,
}

impl RequestFieldPath {
    /// Creates an empty path.
    #[must_use]
    pub const fn new() -> Self {
        Self { segments: Vec::new() }
    }

    /// Creates a path containing one field segment.
    #[must_use]
    pub fn field(name: &'static str) -> Self {
        let mut path = Self::new();
        path.push_field(name);
        path
    }

    /// Appends a field segment.
    pub fn push_field(&mut self, name: &'static str) {
        self.segments.push(RequestFieldPathSegment::Field(name));
    }

    /// Appends an index segment.
    pub fn push_index(&mut self, index: usize) {
        self.segments.push(RequestFieldPathSegment::Index(index));
    }

    /// Removes the last segment.
    pub fn pop(&mut self) {
        self.segments.pop();
    }

    /// Returns the path segments.
    #[must_use]
    pub fn segments(&self) -> &[RequestFieldPathSegment] {
        &self.segments
    }
}

impl fmt::Display for RequestFieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            match segment {
                RequestFieldPathSegment::Field(name) => {
                    if index != 0 {
                        f.write_str(".")?;
                    }
                    f.write_str(name)?;
                }
                RequestFieldPathSegment::Index(value) => write!(f, "[{value}]")?,
            }
        }

        if self.segments.is_empty() {
            f.write_str("request")?;
        }
        Ok(())
    }
}

/// A reason why a value is invalid before it is sent to Telegram.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InvalidValueReason {
    /// The value must not be zero.
    MustBeNonZero,
    /// The value must be within an inclusive range.
    MustBeInRange { min: usize, max: usize },
    /// The value must contain a byte length within an inclusive range.
    MustHaveByteLength { min: usize, max: usize },
    /// Exactly one action field must be present.
    MustHaveExactlyOneAction,
    /// The value must be one of the listed values.
    MustBeOneOf(&'static [&'static str]),
    /// The button must use callback data as its action.
    MustBeCallbackButton,
    /// At least one of the alternative fields must be present.
    MustHaveTextOrRichMessage,
    /// Rich-message button labels may only contain plain text, custom emoji
    /// and date-time objects.
    MustBeRichButtonLabel,
    /// A field is not supported by the selected Bot API object.
    MustBeUnset,
    /// The value must be a 1-64 character ASCII identifier.
    MustBeAsciiIdentifier,
}

impl fmt::Display for InvalidValueReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MustBeNonZero => f.write_str("must be non-zero"),
            Self::MustBeInRange { min, max } => write!(f, "must be in range {min}..={max}"),
            Self::MustHaveByteLength { min, max } => {
                write!(f, "must contain {min}..={max} bytes")
            }
            Self::MustHaveExactlyOneAction => {
                f.write_str("exactly one button action must be specified")
            }
            Self::MustBeOneOf(values) => write!(f, "must be one of {}", values.join(", ")),
            Self::MustBeCallbackButton => f.write_str("must be a callback button"),
            Self::MustHaveTextOrRichMessage => {
                f.write_str("text or rich_message must be specified")
            }
            Self::MustBeRichButtonLabel => {
                f.write_str("must contain only plain text, custom emoji or date-time objects")
            }
            Self::MustBeUnset => f.write_str("must be omitted"),
            Self::MustBeAsciiIdentifier => f.write_str("must be a 1-64 character ASCII identifier"),
        }
    }
}

/// The rich-message sending context used by context-sensitive validation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RichMessageContext {
    /// `sendRichMessage` or another regular message send method.
    Send,
    /// `sendRichMessage` with `ephemeral_message_parameters`.
    EphemeralSend,
    /// `editMessageText` for a regular message.
    Edit,
    /// `editMessageText` for an inline message.
    EditInline,
    /// `editEphemeralMessageText`.
    EphemeralEdit,
    /// `sendRichMessageDraft`.
    Draft,
    /// An inline query result.
    InlineResult,
    /// An `answerGuestQuery` result.
    GuestResult,
    /// An `answerWebAppQuery` or `savePreparedInlineMessage` result.
    PreparedMessage,
}

impl fmt::Display for RichMessageContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Send => "rich message send",
            Self::EphemeralSend => "ephemeral rich message send",
            Self::Edit => "rich message edit",
            Self::EditInline => "inline rich message edit",
            Self::EphemeralEdit => "ephemeral rich message edit",
            Self::Draft => "rich message draft",
            Self::InlineResult => "inline result",
            Self::GuestResult => "guest result",
            Self::PreparedMessage => "prepared message",
        };
        f.write_str(name)
    }
}

/// A statically known reason why a request cannot be sent in its current form.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
pub enum RequestValidationError {
    /// A scalar value violates a method or Bot API constraint.
    #[error("invalid value at {path}: {reason}")]
    InvalidValue {
        /// Path to the invalid value.
        path: RequestFieldPath,
        /// The known violated constraint.
        reason: InvalidValueReason,
    },
    /// A valid object is not allowed by the selected method context.
    #[error("{path} is not supported in {context}")]
    UnsupportedInContext {
        /// Path to the unsupported object.
        path: RequestFieldPath,
        /// The method context that rejected it.
        context: RichMessageContext,
    },
    /// A new file upload is forbidden by the selected method.
    #[error("direct file upload is not allowed at {path}")]
    DirectUploadNotAllowed {
        /// Path to the forbidden file source.
        path: RequestFieldPath,
    },
}

/// Validates a request or another payload before serialization and dispatch.
///
/// Implementations must only check constraints that can be determined from the
/// value itself. A successful result does not guarantee that Telegram accepts
/// the request; permissions and server-side capabilities remain Telegram's
/// responsibility. A validation error means that no HTTP request was sent.
pub trait Validate {
    /// Checks static request constraints without serializing or consuming
    /// `self`.
    fn validate(&self) -> Result<(), RequestValidationError>;
}

impl<T> Validate for T
where
    T: crate::requests::Payload + ?Sized,
{
    fn validate(&self) -> Result<(), RequestValidationError> {
        crate::requests::Payload::validate(self)
    }
}

/// Validates a value whose validity depends on a selected request context.
pub trait ValidateWith<C> {
    /// Checks static constraints for `self` in `context`.
    fn validate_with(&self, context: &C) -> Result<(), RequestValidationError>;
}

/// Validates the non-rich `sendMessageDraft` payload.
pub(crate) fn validate_send_message_draft(
    payload: &crate::payloads::SendMessageDraft,
) -> Result<(), RequestValidationError> {
    if payload.draft_id == 0 {
        return Err(RequestValidationError::InvalidValue {
            path: RequestFieldPath::field("draft_id"),
            reason: InvalidValueReason::MustBeNonZero,
        });
    }
    Ok(())
}

impl RequestFieldPath {
    #[cfg(test)]
    fn with_field_for_test(mut self, name: &'static str) -> Self {
        self.push_field(name);
        self
    }

    #[cfg(test)]
    fn with_index_for_test(mut self, index: usize) -> Self {
        self.push_index(index);
        self
    }
}

fn validate_rich_message_at(
    message: &crate::types::InputRichMessage,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
) -> Result<(), RequestValidationError> {
    if let Some(blocks) = message.blocks_ref() {
        path.push_field("blocks");
        validate_blocks(blocks, context, path)?;
        path.pop();
    }

    if let Some(media) = &message.media {
        path.push_field("media");
        for (index, media) in media.iter().enumerate() {
            path.push_index(index);
            path.push_field("id");
            if media.id.is_empty()
                || media.id.len() > 64
                || !media
                    .id
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
            {
                return Err(RequestValidationError::InvalidValue {
                    path: path.clone(),
                    reason: InvalidValueReason::MustBeAsciiIdentifier,
                });
            }
            path.pop();
            path.push_field("media");
            validate_rich_media_content(&media.media, context, path)?;
            path.pop();
            path.pop();
        }
        path.pop();
    }

    Ok(())
}

fn validate_blocks(
    blocks: &[crate::types::InputRichBlock],
    context: RichMessageContext,
    path: &mut RequestFieldPath,
) -> Result<(), RequestValidationError> {
    for (index, block) in blocks.iter().enumerate() {
        path.push_index(index);
        validate_block(block, context, path)?;
        path.pop();
    }
    Ok(())
}

fn validate_block(
    block: &crate::types::InputRichBlock,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
) -> Result<(), RequestValidationError> {
    use crate::types::InputRichBlock;

    match block {
        InputRichBlock::Paragraph(value) => {
            validate_block_text(&value.text, context, path, "paragraph", "text")?;
        }
        InputRichBlock::Heading(value) => {
            validate_block_text(&value.text, context, path, "heading", "text")?;
        }
        InputRichBlock::Pre(value) => {
            validate_block_text(&value.text, context, path, "pre", "text")?;
        }
        InputRichBlock::Footer(value) => {
            validate_block_text(&value.text, context, path, "footer", "text")?;
        }
        InputRichBlock::List(value) => {
            path.push_field("list");
            path.push_field("items");
            for (index, item) in value.items.iter().enumerate() {
                path.push_index(index);
                path.push_field("blocks");
                validate_blocks(&item.blocks, context, path)?;
                path.pop();
                path.pop();
            }
            path.pop();
            path.pop();
        }
        InputRichBlock::Blockquote(value) => {
            path.push_field("blockquote");
            validate_optional_rich_text(value.credit.as_ref(), context, path, "credit")?;
            path.push_field("blocks");
            validate_blocks(&value.blocks, context, path)?;
            path.pop();
            path.pop();
        }
        InputRichBlock::Document(value) => {
            path.push_field("document");
            path.push_field("media");
            validate_file(&value.document.media, context, path)?;
            path.pop();
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            validate_optional_file(value.document.thumbnail.as_ref(), context, path, "thumbnail")?;
            path.pop();
        }
        InputRichBlock::Collage(value) => {
            path.push_field("collage");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            path.push_field("blocks");
            validate_blocks(&value.blocks, context, path)?;
            path.pop();
            path.pop();
        }
        InputRichBlock::Slideshow(value) => {
            path.push_field("slideshow");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            path.push_field("blocks");
            validate_blocks(&value.blocks, context, path)?;
            path.pop();
            path.pop();
        }
        InputRichBlock::Details(value) => {
            path.push_field("details");
            validate_block_text(&value.summary, context, path, "details", "summary")?;
            path.push_field("blocks");
            validate_blocks(&value.blocks, context, path)?;
            path.pop();
            path.pop();
        }
        InputRichBlock::Animation(value) => {
            path.push_field("animation");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            validate_animation(&value.animation, context, path)?;
            path.pop();
        }
        InputRichBlock::Audio(value) => {
            path.push_field("audio");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            validate_audio(&value.audio, context, path)?;
            path.pop();
        }
        InputRichBlock::Photo(value) => {
            path.push_field("photo");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            validate_photo(&value.photo, context, path)?;
            path.pop();
        }
        InputRichBlock::Video(value) => {
            path.push_field("video");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            validate_video(&value.video, context, path)?;
            path.pop();
        }
        InputRichBlock::VoiceNote(value) => {
            path.push_field("voice_note");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            validate_voice_note(&value.voice_note, context, path)?;
            path.pop();
        }
        InputRichBlock::Buttons(value) => validate_buttons(value, context, path)?,
        InputRichBlock::Thinking(_) if context != RichMessageContext::Draft => {
            return Err(RequestValidationError::UnsupportedInContext {
                path: path.clone(),
                context,
            });
        }
        InputRichBlock::ExpandableBlockquote(value) => {
            path.push_field("expandable_blockquote");
            path.push_field("text");
            validate_rich_text_at(&value.text, context, path)?;
            path.pop();
            validate_optional_rich_text(value.credit.as_ref(), context, path, "credit")?;
            path.pop();
        }
        InputRichBlock::Pullquote(value) => {
            path.push_field("pullquote");
            path.push_field("text");
            validate_rich_text_at(&value.text, context, path)?;
            path.pop();
            validate_optional_rich_text(value.credit.as_ref(), context, path, "credit")?;
            path.pop();
        }
        InputRichBlock::Table(value) => {
            path.push_field("table");
            for (row_index, row) in value.cells.iter().enumerate() {
                path.push_field("cells");
                path.push_index(row_index);
                for (cell_index, cell) in row.iter().enumerate() {
                    path.push_index(cell_index);
                    validate_optional_rich_text(cell.text.as_ref(), context, path, "text")?;
                    path.pop();
                }
                path.pop();
                path.pop();
            }
            validate_optional_rich_text(value.caption.as_ref(), context, path, "caption")?;
            path.pop();
        }
        InputRichBlock::Map(value) => {
            path.push_field("map");
            validate_optional_caption(value.caption.as_ref(), context, path)?;
            path.pop();
        }
        InputRichBlock::Thinking(value) => {
            validate_block_text(&value.text, context, path, "thinking", "text")?;
        }
        InputRichBlock::Divider(_)
        | InputRichBlock::MathematicalExpression(_)
        | InputRichBlock::Anchor(_) => {}
    }

    Ok(())
}

fn validate_block_text(
    text: &crate::types::RichText,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
    block: &'static str,
    field: &'static str,
) -> Result<(), RequestValidationError> {
    path.push_field(block);
    path.push_field(field);
    let result = validate_rich_text_at(text, context, path);
    path.pop();
    path.pop();
    result
}

fn validate_optional_rich_text(
    text: Option<&crate::types::RichText>,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
    field: &'static str,
) -> Result<(), RequestValidationError> {
    let Some(text) = text else { return Ok(()) };
    path.push_field(field);
    let result = validate_rich_text_at(text, context, path);
    path.pop();
    result
}

fn validate_optional_caption(
    caption: Option<&crate::types::RichBlockCaption>,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let Some(caption) = caption else { return Ok(()) };
    path.push_field("caption");
    validate_rich_text_at(&caption.text, context, &mut path.clone())?;
    validate_optional_rich_text(caption.credit.as_ref(), context, path, "credit")?;
    path.pop();
    Ok(())
}

fn validate_rich_text_at(
    text: &crate::types::RichText,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
) -> Result<(), RequestValidationError> {
    use crate::types::RichTextObject;

    match text {
        crate::types::RichText::Text(_) => {}
        crate::types::RichText::List(items) => {
            for (index, item) in items.iter().enumerate() {
                path.push_index(index);
                validate_rich_text_at(item, context, path)?;
                path.pop();
            }
        }
        crate::types::RichText::Object(object) => match object {
            RichTextObject::Bold(value) => {
                validate_nested_rich_text(&value.text, context, path, "bold")?;
            }
            RichTextObject::Italic(value) => {
                validate_nested_rich_text(&value.text, context, path, "italic")?;
            }
            RichTextObject::Underline(value) => {
                validate_nested_rich_text(&value.text, context, path, "underline")?;
            }
            RichTextObject::Strikethrough(value) => {
                validate_nested_rich_text(&value.text, context, path, "strikethrough")?;
            }
            RichTextObject::Spoiler(value) => {
                validate_nested_rich_text(&value.text, context, path, "spoiler")?;
            }
            RichTextObject::DateTime(value) => {
                validate_nested_rich_text(&value.text, context, path, "date_time")?;
            }
            RichTextObject::TextMention(value) => {
                validate_nested_rich_text(&value.text, context, path, "text_mention")?;
            }
            RichTextObject::Subscript(value) => {
                validate_nested_rich_text(&value.text, context, path, "subscript")?;
            }
            RichTextObject::Superscript(value) => {
                validate_nested_rich_text(&value.text, context, path, "superscript")?;
            }
            RichTextObject::Marked(value) => {
                validate_nested_rich_text(&value.text, context, path, "marked")?;
            }
            RichTextObject::Code(value) => {
                validate_nested_rich_text(&value.text, context, path, "code")?;
            }
            RichTextObject::Url(value) => {
                validate_nested_rich_text(&value.text, context, path, "url")?;
            }
            RichTextObject::EmailAddress(value) => {
                validate_nested_rich_text(&value.text, context, path, "email_address")?;
            }
            RichTextObject::PhoneNumber(value) => {
                validate_nested_rich_text(&value.text, context, path, "phone_number")?;
            }
            RichTextObject::BankCardNumber(value) => {
                validate_nested_rich_text(&value.text, context, path, "bank_card_number")?;
            }
            RichTextObject::Mention(value) => {
                validate_nested_rich_text(&value.text, context, path, "mention")?;
            }
            RichTextObject::Hashtag(value) => {
                validate_nested_rich_text(&value.text, context, path, "hashtag")?;
            }
            RichTextObject::Cashtag(value) => {
                validate_nested_rich_text(&value.text, context, path, "cashtag")?;
            }
            RichTextObject::BotCommand(value) => {
                validate_nested_rich_text(&value.text, context, path, "bot_command")?;
            }
            RichTextObject::Button(value) => {
                path.push_field("button");
                let mut button_path = path.clone();
                button_path.push_field("button");
                validate_button(&value.button, context, &button_path)?;
                path.pop();
            }
            RichTextObject::CustomEmoji(_)
            | RichTextObject::MathematicalExpression(_)
            | RichTextObject::Anchor(_)
            | RichTextObject::AnchorLink(_)
            | RichTextObject::Reference(_)
            | RichTextObject::ReferenceLink(_)
            | RichTextObject::Unknown(_) => {}
        },
    }
    Ok(())
}

fn validate_nested_rich_text(
    text: &crate::types::RichText,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
    object: &'static str,
) -> Result<(), RequestValidationError> {
    path.push_field(object);
    path.push_field("text");
    let result = validate_rich_text_at(text, context, path);
    path.pop();
    path.pop();
    result
}

fn validate_rich_media_content(
    media: &crate::types::InputRichMessageMediaContent,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    use crate::types::InputRichMessageMediaContent;

    match media {
        InputRichMessageMediaContent::Animation(value) => validate_animation(value, context, path),
        InputRichMessageMediaContent::Audio(value) => validate_audio(value, context, path),
        InputRichMessageMediaContent::Document(value) => validate_document(value, context, path),
        InputRichMessageMediaContent::Photo(value) => validate_photo(value, context, path),
        InputRichMessageMediaContent::Video(value) => validate_video(value, context, path),
        InputRichMessageMediaContent::VoiceNote(value) => validate_voice_note(value, context, path),
    }
}

fn validate_animation(
    media: &crate::types::InputMediaAnimation,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut path = path.clone();
    path.push_field("media");
    validate_file(&media.media, context, &path)?;
    path.pop();
    validate_optional_file(media.thumbnail.as_ref(), context, &mut path, "thumbnail")
}

fn validate_audio(
    media: &crate::types::InputMediaAudio,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut path = path.clone();
    path.push_field("media");
    validate_file(&media.media, context, &path)?;
    path.pop();
    validate_optional_file(media.thumbnail.as_ref(), context, &mut path, "thumbnail")
}

fn validate_document(
    media: &crate::types::InputMediaDocument,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut path = path.clone();
    path.push_field("media");
    validate_file(&media.media, context, &path)?;
    path.pop();
    validate_optional_file(media.thumbnail.as_ref(), context, &mut path, "thumbnail")
}

fn validate_photo(
    media: &crate::types::InputMediaPhoto,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut path = path.clone();
    path.push_field("media");
    validate_file(&media.media, context, &path)
}

fn validate_video(
    media: &crate::types::InputMediaVideo,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut path = path.clone();
    path.push_field("media");
    validate_file(&media.media, context, &path)?;
    path.pop();
    validate_optional_file(media.thumbnail.as_ref(), context, &mut path, "thumbnail")?;
    validate_optional_file(media.cover.as_ref(), context, &mut path, "cover")
}

fn validate_voice_note(
    media: &crate::types::InputMediaVoiceNote,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut path = path.clone();
    path.push_field("media");
    validate_file(&media.media, context, &path)
}

fn validate_optional_file(
    file: Option<&crate::types::InputFile>,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
    field: &'static str,
) -> Result<(), RequestValidationError> {
    let Some(file) = file else { return Ok(()) };
    path.push_field(field);
    let result = validate_file(file, context, path);
    path.pop();
    result
}

fn validate_file(
    file: &crate::types::InputFile,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let allowed = match context {
        RichMessageContext::Draft
        | RichMessageContext::EditInline
        | RichMessageContext::InlineResult
        | RichMessageContext::GuestResult
        | RichMessageContext::PreparedMessage => {
            matches!(file.source_kind(), crate::types::InputFileSourceKind::FileId)
        }
        _ => true,
    };
    if !allowed {
        return Err(RequestValidationError::DirectUploadNotAllowed { path: path.clone() });
    }
    Ok(())
}

impl ValidateWith<RichMessageContext> for crate::types::InputRichMessage {
    fn validate_with(&self, context: &RichMessageContext) -> Result<(), RequestValidationError> {
        let mut path = RequestFieldPath::field("rich_message");
        validate_rich_message_at(self, *context, &mut path)
    }
}

pub(crate) fn validate_send_rich_message(
    payload: &crate::payloads::SendRichMessage,
) -> Result<(), RequestValidationError> {
    let context = if payload.ephemeral_message_parameters.is_some() {
        RichMessageContext::EphemeralSend
    } else {
        RichMessageContext::Send
    };
    let mut path = RequestFieldPath::field("rich_message");
    validate_rich_message_at(&payload.rich_message, context, &mut path)?;
    if let Some(reply_markup) = &payload.reply_markup {
        validate_reply_markup_in_context(reply_markup, "reply_markup", context)?;
    }
    Ok(())
}

pub(crate) fn validate_send_rich_message_draft(
    payload: &crate::payloads::SendRichMessageDraft,
) -> Result<(), RequestValidationError> {
    if payload.draft_id == 0 {
        return Err(RequestValidationError::InvalidValue {
            path: RequestFieldPath::field("draft_id"),
            reason: InvalidValueReason::MustBeNonZero,
        });
    }

    let mut path = RequestFieldPath::field("rich_message");
    validate_rich_message_at(&payload.rich_message, RichMessageContext::Draft, &mut path)
}

pub(crate) fn validate_edit_ephemeral_message_text(
    payload: &crate::payloads::EditEphemeralMessageText,
) -> Result<(), RequestValidationError> {
    if payload.text.is_none() && payload.rich_message.is_none() {
        return Err(RequestValidationError::InvalidValue {
            path: RequestFieldPath::new(),
            reason: InvalidValueReason::MustHaveTextOrRichMessage,
        });
    }

    if let Some(rich_message) = &payload.rich_message {
        let mut path = RequestFieldPath::field("rich_message");
        validate_rich_message_at(rich_message, RichMessageContext::EphemeralEdit, &mut path)?;
    }
    if let Some(reply_markup) = &payload.reply_markup {
        validate_inline_keyboard_markup_in_context(
            reply_markup,
            "reply_markup",
            Some(RichMessageContext::EphemeralEdit),
        )?;
    }
    Ok(())
}

pub(crate) fn validate_edit_message_text(
    payload: &crate::payloads::EditMessageText,
) -> Result<(), RequestValidationError> {
    let Some(rich_message) = &payload.rich_message else { return Ok(()) };
    let mut path = RequestFieldPath::field("rich_message");
    validate_rich_message_at(rich_message, RichMessageContext::Edit, &mut path)
}

pub(crate) fn validate_edit_message_text_inline(
    payload: &crate::payloads::EditMessageTextInline,
) -> Result<(), RequestValidationError> {
    let Some(rich_message) = &payload.rich_message else { return Ok(()) };
    let mut path = RequestFieldPath::field("rich_message");
    validate_rich_message_at(rich_message, RichMessageContext::EditInline, &mut path)
}

const INLINE_BUTTON_STYLES: &[&str] = &["danger", "success", "primary"];

/// Validates the Bot API's native styles on a regular inline keyboard.
pub(crate) fn validate_inline_keyboard_markup(
    markup: &crate::types::InlineKeyboardMarkup,
    field: &'static str,
) -> Result<(), RequestValidationError> {
    validate_inline_keyboard_markup_in_context(markup, field, None)
}

fn validate_inline_keyboard_markup_in_context(
    markup: &crate::types::InlineKeyboardMarkup,
    field: &'static str,
    context: Option<RichMessageContext>,
) -> Result<(), RequestValidationError> {
    let mut path = RequestFieldPath::field(field);
    for (row_index, row) in markup.inline_keyboard.iter().enumerate() {
        path.push_index(row_index);
        for (button_index, button) in row.iter().enumerate() {
            path.push_index(button_index);
            if let Some(style) = &button.style {
                if !INLINE_BUTTON_STYLES.contains(&style.as_str()) {
                    let mut style_path = path.clone();
                    style_path.push_field("style");
                    return Err(RequestValidationError::InvalidValue {
                        path: style_path,
                        reason: InvalidValueReason::MustBeOneOf(INLINE_BUTTON_STYLES),
                    });
                }
            }
            if let Some(context) = context {
                if matches!(
                    context,
                    RichMessageContext::EphemeralSend | RichMessageContext::EphemeralEdit
                ) && matches!(&button.kind, crate::types::InlineKeyboardButtonKind::LoginUrl(_))
                {
                    let mut login_url_path = path.clone();
                    login_url_path.push_field("login_url");
                    return Err(RequestValidationError::UnsupportedInContext {
                        path: login_url_path,
                        context,
                    });
                }
            }
            path.pop();
        }
        path.pop();
    }
    Ok(())
}

/// Applies keyboard validation to payload fields without making the payload
/// generator aware of every request that can carry a reply markup.
pub(crate) fn validate_payload_field<T: 'static>(
    value: &T,
    field: &'static str,
) -> Result<(), RequestValidationError> {
    let value = value as &dyn std::any::Any;
    if let Some(markup) = value.downcast_ref::<crate::types::InlineKeyboardMarkup>() {
        return validate_inline_keyboard_markup(markup, field);
    }
    if let Some(Some(markup)) = value.downcast_ref::<Option<crate::types::InlineKeyboardMarkup>>() {
        return validate_inline_keyboard_markup(markup, field);
    }
    if let Some(markup) = value.downcast_ref::<crate::types::ReplyMarkup>() {
        return validate_reply_markup(markup, field);
    }
    if let Some(Some(markup)) = value.downcast_ref::<Option<crate::types::ReplyMarkup>>() {
        return validate_reply_markup(markup, field);
    }
    Ok(())
}

/// Validates an inline keyboard when it is wrapped in `ReplyMarkup`.
pub(crate) fn validate_reply_markup(
    markup: &crate::types::ReplyMarkup,
    field: &'static str,
) -> Result<(), RequestValidationError> {
    if let crate::types::ReplyMarkup::InlineKeyboard(markup) = markup {
        validate_inline_keyboard_markup(markup, field)?;
    }
    Ok(())
}

fn validate_reply_markup_in_context(
    markup: &crate::types::ReplyMarkup,
    field: &'static str,
    context: RichMessageContext,
) -> Result<(), RequestValidationError> {
    if let crate::types::ReplyMarkup::InlineKeyboard(markup) = markup {
        validate_inline_keyboard_markup_in_context(markup, field, Some(context))?;
    }
    Ok(())
}

const BUTTON_STYLES: &[&str] = &["danger", "success", "primary", "link"];
const BUTTON_ALIGNS: &[&str] = &["left", "center", "right"];

fn validate_buttons(
    value: &crate::types::InputRichBlockButtons,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut buttons_path = path.clone();
    buttons_path.push_field("buttons");
    if !(1..=8).contains(&value.buttons.len()) {
        return Err(RequestValidationError::InvalidValue {
            path: buttons_path,
            reason: InvalidValueReason::MustBeInRange { min: 1, max: 8 },
        });
    }

    if let Some(align) = &value.align {
        if !BUTTON_ALIGNS.contains(&align.as_str()) {
            let mut field_path = path.clone();
            field_path.push_field("align");
            return Err(RequestValidationError::InvalidValue {
                path: field_path,
                reason: InvalidValueReason::MustBeOneOf(BUTTON_ALIGNS),
            });
        }
    }

    for (index, button) in value.buttons.iter().enumerate() {
        let mut button_path = buttons_path.clone();
        button_path.push_index(index);
        validate_button(button, context, &button_path)?;
    }
    Ok(())
}

fn validate_button(
    button: &crate::types::RichMessageButton,
    context: RichMessageContext,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let mut text_path = path.clone();
    text_path.push_field("text");
    validate_rich_button_label(&button.text, &text_path)?;

    let action_count = [
        button.url.is_some(),
        button.callback_data.is_some(),
        button.web_app.is_some(),
        button.login_url.is_some(),
        button.switch_inline_query.is_some(),
        button.switch_inline_query_current_chat.is_some(),
        button.switch_inline_query_chosen_chat.is_some(),
        button.copy_text.is_some(),
        button.disabled.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if action_count != 1 {
        return Err(RequestValidationError::InvalidValue {
            path: path.clone(),
            reason: InvalidValueReason::MustHaveExactlyOneAction,
        });
    }

    if let Some(style) = &button.style {
        if !BUTTON_STYLES.contains(&style.as_str()) {
            let mut field_path = path.clone();
            field_path.push_field("style");
            return Err(RequestValidationError::InvalidValue {
                path: field_path,
                reason: InvalidValueReason::MustBeOneOf(BUTTON_STYLES),
            });
        }
        if style == "link" && button.callback_data.is_none() {
            let mut field_path = path.clone();
            field_path.push_field("style");
            return Err(RequestValidationError::InvalidValue {
                path: field_path,
                reason: InvalidValueReason::MustBeCallbackButton,
            });
        }
    }

    if let Some(callback_data) = &button.callback_data {
        let length = callback_data.len();
        if !(1..=64).contains(&length) {
            let mut field_path = path.clone();
            field_path.push_field("callback_data");
            return Err(RequestValidationError::InvalidValue {
                path: field_path,
                reason: InvalidValueReason::MustHaveByteLength { min: 1, max: 64 },
            });
        }
    }

    if let Some(login_url) = &button.login_url {
        if login_url.bot_username.is_some() {
            let mut field_path = path.clone();
            field_path.push_field("login_url");
            field_path.push_field("bot_username");
            return Err(RequestValidationError::InvalidValue {
                path: field_path,
                reason: InvalidValueReason::MustBeUnset,
            });
        }
        if matches!(context, RichMessageContext::EphemeralSend | RichMessageContext::EphemeralEdit)
        {
            let mut field_path = path.clone();
            field_path.push_field("login_url");
            return Err(RequestValidationError::UnsupportedInContext { path: field_path, context });
        }
    }
    Ok(())
}

fn validate_rich_button_label(
    text: &crate::types::RichText,
    path: &RequestFieldPath,
) -> Result<(), RequestValidationError> {
    use crate::types::RichTextObject;

    match text {
        crate::types::RichText::Text(_) => Ok(()),
        crate::types::RichText::List(items) => {
            for (index, item) in items.iter().enumerate() {
                let mut item_path = path.clone();
                item_path.push_index(index);
                validate_rich_button_label(item, &item_path)?;
            }
            Ok(())
        }
        crate::types::RichText::Object(RichTextObject::CustomEmoji(_)) => Ok(()),
        crate::types::RichText::Object(RichTextObject::DateTime(value)) => {
            let mut nested_path = path.clone();
            nested_path.push_field("date_time");
            nested_path.push_field("text");
            validate_rich_button_label(&value.text, &nested_path)
        }
        crate::types::RichText::Object(_) => Err(RequestValidationError::InvalidValue {
            path: path.clone(),
            reason: InvalidValueReason::MustBeRichButtonLabel,
        }),
    }
}

fn validate_inline_result_at(
    result: &crate::types::InlineQueryResult,
    context: RichMessageContext,
    path: &mut RequestFieldPath,
) -> Result<(), RequestValidationError> {
    let Some(content) = result.input_message_content_ref() else { return Ok(()) };
    path.push_field("input_message_content");
    if let crate::types::InputMessageContent::Rich(content) = content {
        path.push_field("rich_message");
        validate_rich_message_at(&content.rich_message, context, path)?;
        path.pop();
    }
    path.pop();
    Ok(())
}

fn validate_result_payload(
    result: &crate::types::InlineQueryResult,
    context: RichMessageContext,
    root: &'static str,
) -> Result<(), RequestValidationError> {
    let mut path = RequestFieldPath::field(root);
    validate_inline_result_at(result, context, &mut path)
}

pub(crate) fn validate_answer_inline_query(
    payload: &crate::payloads::AnswerInlineQuery,
) -> Result<(), RequestValidationError> {
    let mut path = RequestFieldPath::field("results");
    for (index, result) in payload.results.iter().enumerate() {
        path.push_index(index);
        validate_inline_result_at(result, RichMessageContext::InlineResult, &mut path)?;
        path.pop();
    }
    Ok(())
}

pub(crate) fn validate_answer_guest_query(
    payload: &crate::payloads::AnswerGuestQuery,
) -> Result<(), RequestValidationError> {
    validate_result_payload(&payload.result, RichMessageContext::GuestResult, "result")
}

pub(crate) fn validate_answer_web_app_query(
    payload: &crate::payloads::AnswerWebAppQuery,
) -> Result<(), RequestValidationError> {
    validate_result_payload(&payload.result, RichMessageContext::PreparedMessage, "result")
}

pub(crate) fn validate_save_prepared_inline_message(
    payload: &crate::payloads::SavePreparedInlineMessage,
) -> Result<(), RequestValidationError> {
    validate_result_payload(&payload.result, RichMessageContext::PreparedMessage, "result")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        payloads::{
            AnswerGuestQuery, AnswerInlineQuery, AnswerWebAppQuery, EditEphemeralMessageText,
            EditEphemeralMessageTextSetters, EditMessageText, EditMessageTextInline,
            SavePreparedInlineMessage, SendMessage, SendMessageDraft, SendRichMessage,
            SendRichMessageDraft,
        },
        requests::{Payload, Request, Requester},
        types::{
            EphemeralMessageParameters, FileId, InlineKeyboardButton, InlineKeyboardMarkup,
            InlineQueryId, InlineQueryResult, InlineQueryResultArticle, InputFile,
            InputMediaDocument, InputMediaPhoto, InputMediaVideo, InputMediaVoiceNote,
            InputMessageContent, InputRichBlock, InputRichBlockBlockQuotation,
            InputRichBlockButtons, InputRichBlockDetails, InputRichBlockDocument,
            InputRichBlockList, InputRichBlockListItem, InputRichBlockParagraph,
            InputRichBlockPhoto, InputRichBlockThinking, InputRichBlockVideo,
            InputRichBlockVoiceNote, InputRichMessage, InputRichMessageContent,
            InputRichMessageMedia, InputRichMessageMediaContent, LoginUrl, ReplyMarkup,
            RichMessageButton, RichText, RichTextBold, RichTextButton, UserId,
        },
        Bot,
    };

    #[test]
    fn field_path_formats_nested_fields_and_indexes() {
        let mut path = RequestFieldPath::field("rich_message");
        path.push_field("blocks");
        path.push_index(2);
        path.push_field("details");
        path.push_field("blocks");
        path.push_index(1);

        assert_eq!(path.to_string(), "rich_message.blocks[2].details.blocks[1]");
    }

    #[test]
    fn send_message_draft_rejects_zero_draft_id() {
        let payload = SendMessageDraft::new(UserId(1), 0);

        assert_eq!(
            Payload::validate(&payload),
            Err(RequestValidationError::InvalidValue {
                path: RequestFieldPath::field("draft_id"),
                reason: InvalidValueReason::MustBeNonZero,
            })
        );
        assert_eq!(
            Payload::validate(&payload).unwrap_err().to_string(),
            "invalid value at draft_id: must be non-zero"
        );
    }

    #[test]
    fn send_message_draft_accepts_non_zero_draft_id() {
        let payload = SendMessageDraft::new(UserId(1), -1);
        assert_eq!(Validate::validate(&payload), Ok(()));
    }

    #[test]
    fn rich_buttons_require_one_action_and_validate_limits() {
        let no_action =
            InputRichMessage::blocks([InputRichBlock::Buttons(InputRichBlockButtons::new([
                RichMessageButton::new("button"),
            ]))]);
        assert!(matches!(
            no_action.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustHaveExactlyOneAction,
                ..
            })
        ));

        let invalid_align = InputRichMessage::blocks([InputRichBlock::Buttons(
            InputRichBlockButtons::new([RichMessageButton::callback("button", "callback")])
                .align("diagonal"),
        )]);
        assert!(matches!(
            invalid_align.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustBeOneOf(_),
                ..
            })
        ));

        let invalid_callback =
            InputRichMessage::blocks([InputRichBlock::Buttons(InputRichBlockButtons::new([
                RichMessageButton::callback("button", "x".repeat(65)),
            ]))]);
        assert!(matches!(
            invalid_callback.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustHaveByteLength { .. },
                ..
            })
        ));

        let too_many =
            InputRichMessage::blocks([InputRichBlock::Buttons(InputRichBlockButtons::new(
                (0..9).map(|index| RichMessageButton::callback("button", index.to_string())),
            ))]);
        assert!(matches!(
            too_many.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustBeInRange { min: 1, max: 8 },
                ..
            })
        ));

        let valid = InputRichMessage::blocks([InputRichBlock::Buttons(
            InputRichBlockButtons::new([RichMessageButton::callback("button", "callback")])
                .align("center"),
        )]);
        assert_eq!(valid.validate_with(&RichMessageContext::Send), Ok(()));
    }

    #[test]
    fn ephemeral_text_edit_requires_text_or_rich_message() {
        let empty = EditEphemeralMessageText::new(UserId(1), UserId(2), 3);
        assert!(matches!(
            Validate::validate(&empty),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustHaveTextOrRichMessage,
                ..
            })
        ));

        let rich_only = EditEphemeralMessageText::new(UserId(1), UserId(2), 3)
            .rich_message(InputRichMessage::html("<b>rich</b>"));
        assert_eq!(Validate::validate(&rich_only), Ok(()));
    }

    #[tokio::test]
    async fn invalid_payload_is_rejected_before_http_dispatch() {
        let result = Bot::new("token").send_message_draft(UserId(1), 0).send().await;

        assert!(matches!(
            result,
            Err(crate::RequestError::Validation(RequestValidationError::InvalidValue { .. }))
        ));
    }

    #[tokio::test]
    async fn valid_payload_reaches_the_transport() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            let _ = std::io::Read::read(&mut stream, &mut request);
            std::io::Write::write_all(
                &mut stream,
                b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{\"ok\":true,\"result\":true}",
            )
            .unwrap();
        });

        let bot = Bot::new("token").set_api_url(format!("http://{address}/").parse().unwrap());
        let result = bot.send_message_draft(UserId(1), 1).send().await;

        assert!(result.is_ok());
        server.join().unwrap();
    }

    #[tokio::test]
    async fn invalid_rich_payload_is_rejected_before_dispatch() {
        let result =
            Bot::new("token").send_rich_message(UserId(1), thinking_message()).send().await;

        assert!(matches!(
            result,
            Err(crate::RequestError::Validation(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::Send,
                ..
            }))
        ));
    }

    fn thinking_message() -> InputRichMessage {
        InputRichMessage::blocks([InputRichBlock::Thinking(InputRichBlockThinking {
            text: RichText::from("thinking"),
        })])
    }

    #[test]
    fn thinking_is_only_valid_in_draft_context() {
        let message = thinking_message();
        assert_eq!(message.validate_with(&RichMessageContext::Draft), Ok(()));

        for context in [
            RichMessageContext::Send,
            RichMessageContext::Edit,
            RichMessageContext::EditInline,
            RichMessageContext::InlineResult,
        ] {
            let error = message.validate_with(&context).unwrap_err();
            assert_eq!(
                error,
                RequestValidationError::UnsupportedInContext {
                    path: RequestFieldPath::field("rich_message")
                        .with_field_for_test("blocks")
                        .with_index_for_test(0),
                    context,
                }
            );
        }
    }

    #[test]
    fn thinking_in_details_preserves_the_nested_path() {
        let message = InputRichMessage::blocks([InputRichBlock::Details(InputRichBlockDetails {
            summary: RichText::from("summary"),
            blocks: vec![InputRichBlock::Thinking(InputRichBlockThinking {
                text: RichText::from("thinking"),
            })],
            is_open: None,
        })]);

        let error = message.validate_with(&RichMessageContext::Send).unwrap_err();
        assert_eq!(
            error,
            RequestValidationError::UnsupportedInContext {
                path: RequestFieldPath::field("rich_message")
                    .with_field_for_test("blocks")
                    .with_index_for_test(0)
                    .with_field_for_test("details")
                    .with_field_for_test("blocks")
                    .with_index_for_test(0),
                context: RichMessageContext::Send,
            }
        );
        assert_eq!(
            error.to_string(),
            "rich_message.blocks[0].details.blocks[0] is not supported in rich message send"
        );
    }

    #[test]
    fn thinking_in_list_and_blockquote_is_checked_recursively() {
        let list = InputRichMessage::blocks([InputRichBlock::List(InputRichBlockList {
            items: vec![InputRichBlockListItem {
                blocks: vec![InputRichBlock::Thinking(InputRichBlockThinking {
                    text: RichText::from("thinking"),
                })],
                has_checkbox: None,
                is_checked: None,
                value: None,
                type_field: None,
            }],
        })]);
        let list_error = list.validate_with(&RichMessageContext::Send).unwrap_err();
        assert_eq!(
            list_error.to_string(),
            "rich_message.blocks[0].list.items[0].blocks[0] is not supported in rich message send"
        );

        let quote =
            InputRichMessage::blocks([InputRichBlock::Blockquote(InputRichBlockBlockQuotation {
                blocks: vec![InputRichBlock::Thinking(InputRichBlockThinking {
                    text: RichText::from("thinking"),
                })],
                credit: None,
            })]);
        let quote_error = quote.validate_with(&RichMessageContext::Send).unwrap_err();
        assert_eq!(
            quote_error.to_string(),
            "rich_message.blocks[0].blockquote.blocks[0] is not supported in rich message send"
        );
    }

    #[test]
    fn draft_rejects_every_direct_upload_source_but_accepts_reusable_files() {
        let sources = [
            InputFile::memory("memory"),
            InputFile::file("/tmp/file"),
            InputFile::read(tokio::io::empty()),
        ];
        for file in sources {
            let message = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
                photo: InputMediaPhoto::new(file),
                caption: None,
            })]);
            assert!(matches!(
                message.validate_with(&RichMessageContext::Draft),
                Err(RequestValidationError::DirectUploadNotAllowed { path })
                    if path.to_string() == "rich_message.blocks[0].photo.media"
            ));
        }

        let file_id = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
            photo: InputMediaPhoto::new(InputFile::file_id(FileId("file-id".to_owned()))),
            caption: None,
        })]);
        assert_eq!(file_id.validate_with(&RichMessageContext::Draft), Ok(()));

        let url = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
            photo: InputMediaPhoto::new(InputFile::url(
                "https://example.com/photo.jpg".parse().unwrap(),
            )),
            caption: None,
        })]);
        assert!(matches!(
            url.validate_with(&RichMessageContext::Draft),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].photo.media"
        ));
    }

    #[test]
    fn inline_edit_accepts_file_ids_but_rejects_urls_and_direct_uploads() {
        let file_id = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
            photo: InputMediaPhoto::new(InputFile::file_id(FileId("file-id".to_owned()))),
            caption: None,
        })]);
        assert_eq!(file_id.validate_with(&RichMessageContext::EditInline), Ok(()));

        let url = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
            photo: InputMediaPhoto::new(InputFile::url(
                "https://example.com/photo.jpg".parse().unwrap(),
            )),
            caption: None,
        })]);
        assert!(matches!(
            url.validate_with(&RichMessageContext::EditInline),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].photo.media"
        ));

        let upload = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
            photo: InputMediaPhoto::new(InputFile::memory("photo")),
            caption: None,
        })]);
        assert!(matches!(
            upload.validate_with(&RichMessageContext::EditInline),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].photo.media"
        ));
    }

    #[test]
    fn rich_text_buttons_are_validated_recursively() {
        let invalid_action =
            InputRichMessage::blocks([InputRichBlock::Paragraph(InputRichBlockParagraph {
                text: RichText::from(RichTextButton {
                    button: Box::new(RichMessageButton::new("button")),
                }),
            })]);
        assert!(matches!(
            invalid_action.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustHaveExactlyOneAction,
                ..
            })
        ));

        let invalid_label =
            InputRichMessage::blocks([InputRichBlock::Paragraph(InputRichBlockParagraph {
                text: RichText::from(RichTextButton {
                    button: Box::new(RichMessageButton::callback(
                        RichText::from(RichTextBold::new("bold")),
                        "callback",
                    )),
                }),
            })]);
        assert!(matches!(
            invalid_label.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustBeRichButtonLabel,
                ..
            })
        ));
    }

    #[test]
    fn rich_button_login_url_constraints_are_context_aware() {
        let login_url = LoginUrl {
            url: "https://example.com/login".parse().unwrap(),
            forward_text: None,
            bot_username: Some("other_bot".to_owned()),
            request_write_access: None,
        };
        let message =
            InputRichMessage::blocks([InputRichBlock::Buttons(InputRichBlockButtons::new([
                RichMessageButton {
                    text: RichText::from("login"),
                    style: None,
                    url: None,
                    callback_data: None,
                    web_app: None,
                    login_url: Some(login_url.clone()),
                    switch_inline_query: None,
                    switch_inline_query_current_chat: None,
                    switch_inline_query_chosen_chat: None,
                    copy_text: None,
                    disabled: None,
                },
            ]))]);
        assert!(matches!(
            message.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustBeUnset,
                ..
            })
        ));

        let ephemeral =
            InputRichMessage::blocks([InputRichBlock::Buttons(InputRichBlockButtons::new([
                RichMessageButton {
                    text: RichText::from("login"),
                    style: None,
                    url: None,
                    callback_data: None,
                    web_app: None,
                    login_url: Some(LoginUrl { bot_username: None, ..login_url }),
                    switch_inline_query: None,
                    switch_inline_query_current_chat: None,
                    switch_inline_query_chosen_chat: None,
                    copy_text: None,
                    disabled: None,
                },
            ]))]);
        assert!(matches!(
            ephemeral.validate_with(&RichMessageContext::EphemeralEdit),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::EphemeralEdit,
                ..
            })
        ));
        assert!(matches!(
            ephemeral.validate_with(&RichMessageContext::EphemeralSend),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::EphemeralSend,
                ..
            })
        ));
    }

    #[test]
    fn inline_and_prepared_rich_results_only_accept_file_ids() {
        for context in [
            RichMessageContext::InlineResult,
            RichMessageContext::GuestResult,
            RichMessageContext::PreparedMessage,
        ] {
            let file_id = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
                photo: InputMediaPhoto::new(InputFile::file_id(FileId("file-id".to_owned()))),
                caption: None,
            })]);
            assert_eq!(file_id.validate_with(&context), Ok(()));

            for file in [
                InputFile::memory("photo"),
                InputFile::url("https://example.com/photo.jpg".parse().unwrap()),
            ] {
                let message =
                    InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
                        photo: InputMediaPhoto::new(file),
                        caption: None,
                    })]);
                assert!(matches!(
                    message.validate_with(&context),
                    Err(RequestValidationError::DirectUploadNotAllowed { path })
                        if path.to_string() == "rich_message.blocks[0].photo.media"
                ));
            }
        }
    }

    #[test]
    fn ephemeral_messages_reject_login_urls_in_both_keyboard_kinds() {
        let login_url = LoginUrl {
            url: "https://example.com/login".parse().unwrap(),
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        };
        let rich =
            InputRichMessage::blocks([InputRichBlock::Buttons(InputRichBlockButtons::new([
                RichMessageButton {
                    text: RichText::from("login"),
                    style: None,
                    url: None,
                    callback_data: None,
                    web_app: None,
                    login_url: Some(login_url.clone()),
                    switch_inline_query: None,
                    switch_inline_query_current_chat: None,
                    switch_inline_query_chosen_chat: None,
                    copy_text: None,
                    disabled: None,
                },
            ]))]);
        let mut send = SendRichMessage::new(UserId(1), rich);
        send.ephemeral_message_parameters = Some(EphemeralMessageParameters::new(UserId(2)));
        assert!(matches!(
            Payload::validate(&send),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::EphemeralSend,
                ..
            })
        ));

        let mut send = SendRichMessage::new(UserId(1), InputRichMessage::html("text"));
        send.ephemeral_message_parameters = Some(EphemeralMessageParameters::new(UserId(2)));
        send.reply_markup = Some(ReplyMarkup::inline_kb([[InlineKeyboardButton::login(
            "login",
            login_url.clone(),
        )]]));
        assert!(matches!(
            Payload::validate(&send),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::EphemeralSend,
                path,
            }) if path.to_string() == "reply_markup[0][0].login_url"
        ));

        let mut edit = EditEphemeralMessageText::new(UserId(1), UserId(2), 3);
        edit.text = Some("text".to_owned());
        edit.reply_markup =
            Some(InlineKeyboardMarkup::new([[InlineKeyboardButton::login("login", login_url)]]));
        assert!(matches!(
            Payload::validate(&edit),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::EphemeralEdit,
                path,
            }) if path.to_string() == "reply_markup[0][0].login_url"
        ));
    }

    #[test]
    fn rich_media_ids_follow_tg_link_identifier_constraints() {
        let message = InputRichMessage::html("<img src=\"tg://photo?id=bad id\">").media([
            InputRichMessageMedia::new(
                "bad id",
                InputRichMessageMediaContent::Photo(InputMediaPhoto::new(InputFile::file_id(
                    FileId("photo".to_owned()),
                ))),
            ),
        ]);

        assert!(matches!(
            message.validate_with(&RichMessageContext::Send),
            Err(RequestValidationError::InvalidValue {
                path,
                reason: InvalidValueReason::MustBeAsciiIdentifier,
            }) if path.to_string() == "rich_message.media[0].id"
        ));
    }

    #[test]
    fn inline_keyboard_styles_are_validated_before_dispatch() {
        let mut payload = SendMessage::new(UserId(1), "text");
        payload.reply_markup =
            Some(ReplyMarkup::inline_kb([[
                InlineKeyboardButton::callback("button", "callback").style("link")
            ]]));

        assert!(matches!(
            Payload::validate(&payload),
            Err(RequestValidationError::InvalidValue {
                reason: InvalidValueReason::MustBeOneOf(_),
                ..
            })
        ));
    }

    #[test]
    fn draft_document_upload_errors_keep_exact_nested_paths() {
        let media_error =
            InputRichMessage::blocks([InputRichBlock::Document(InputRichBlockDocument {
                document: InputMediaDocument::new(InputFile::memory("document")),
                caption: None,
            })]);
        assert!(matches!(
            media_error.validate_with(&RichMessageContext::Draft),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].document.media"
        ));

        let thumbnail_error =
            InputRichMessage::blocks([InputRichBlock::Document(InputRichBlockDocument {
                document: InputMediaDocument::new(InputFile::file_id(FileId("document".into())))
                    .thumbnail(InputFile::memory("thumbnail")),
                caption: None,
            })]);
        assert!(matches!(
            thumbnail_error.validate_with(&RichMessageContext::Draft),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].document.thumbnail"
        ));
    }

    #[test]
    fn draft_rejects_nested_video_thumbnail_and_cover_with_exact_paths() {
        let thumbnail = InputRichMessage::blocks([InputRichBlock::Video(InputRichBlockVideo {
            video: InputMediaVideo::new(InputFile::file_id(FileId("video".to_owned())))
                .thumbnail(InputFile::memory("thumbnail")),
            caption: None,
        })]);
        assert!(matches!(
            thumbnail.validate_with(&RichMessageContext::Draft),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].video.thumbnail"
        ));

        let cover = InputRichMessage::blocks([InputRichBlock::Video(InputRichBlockVideo {
            video: InputMediaVideo::new(InputFile::file_id(FileId("video".to_owned())))
                .cover(InputFile::memory("cover")),
            caption: None,
        })]);
        assert!(matches!(
            cover.validate_with(&RichMessageContext::Draft),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].video.cover"
        ));
    }

    #[test]
    fn draft_rejects_top_level_media_and_voice_note_uploads() {
        let top_level = InputRichMessage::html("<img src=\"tg://photo?id=photo\">").media([
            InputRichMessageMedia::new(
                "photo",
                InputRichMessageMediaContent::Photo(InputMediaPhoto::new(InputFile::memory(
                    "photo",
                ))),
            ),
        ]);
        assert!(matches!(
            top_level.validate_with(&RichMessageContext::Draft),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.media[0].media.media"
        ));

        let voice =
            InputRichMessage::blocks([InputRichBlock::VoiceNote(InputRichBlockVoiceNote {
                voice_note: InputMediaVoiceNote::new(InputFile::memory("voice")),
                caption: None,
            })]);
        assert!(matches!(
            voice.validate_with(&RichMessageContext::Draft),
            Err(RequestValidationError::DirectUploadNotAllowed { path })
                if path.to_string() == "rich_message.blocks[0].voice_note.media"
        ));
    }

    #[test]
    fn validation_does_not_initialize_attachment_ids() {
        let file = InputFile::memory("photo");
        let message = InputRichMessage::blocks([InputRichBlock::Photo(InputRichBlockPhoto {
            photo: InputMediaPhoto::new(file.clone()),
            caption: None,
        })]);
        assert!(!file.attachment_id_initialized());
        let _ = message.validate_with(&RichMessageContext::Draft);
        assert!(!file.attachment_id_initialized());
    }

    #[test]
    fn rich_payload_hooks_use_their_declared_contexts() {
        let thinking = thinking_message();
        assert!(matches!(
            Payload::validate(&SendRichMessage::new(UserId(1), thinking.clone())),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::Send,
                ..
            })
        ));
        assert_eq!(Payload::validate(&SendRichMessageDraft::new(UserId(1), 1, thinking)), Ok(()));
        assert!(matches!(
            Payload::validate(&EditMessageText::rich(
                UserId(1),
                crate::types::MessageId(1),
                thinking_message(),
            )),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::Edit,
                ..
            })
        ));
        assert!(matches!(
            Payload::validate(&EditMessageTextInline::rich("inline", thinking_message())),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::EditInline,
                ..
            })
        ));
    }

    #[test]
    fn inline_result_hooks_validate_rich_content() {
        let result = InlineQueryResult::Article(InlineQueryResultArticle::new(
            "id",
            "title",
            InputMessageContent::Rich(InputRichMessageContent::new(thinking_message())),
        ));
        assert!(matches!(
            Payload::validate(&AnswerInlineQuery::new(
                InlineQueryId("query".to_owned()),
                [result.clone()],
            )),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::InlineResult,
                ..
            })
        ));
        assert!(matches!(
            Payload::validate(&AnswerGuestQuery::new("guest", result.clone())),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::GuestResult,
                ..
            })
        ));
        assert!(matches!(
            Payload::validate(&AnswerWebAppQuery::new("web-app", result.clone())),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::PreparedMessage,
                ..
            })
        ));
        assert!(matches!(
            Payload::validate(&SavePreparedInlineMessage::new(UserId(1), result)),
            Err(RequestValidationError::UnsupportedInContext {
                context: RichMessageContext::PreparedMessage,
                ..
            })
        ));
    }
}
