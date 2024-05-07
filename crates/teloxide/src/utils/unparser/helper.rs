//! A helprt trait for unparsing text and entities back to rendered html or
//! markdown

use teloxide_core::types::{
    MediaAnimation, MediaAudio, MediaDocument, MediaKind, MediaPhoto, MediaText, MediaVideo,
    MediaVoice, Message, MessageCommon, MessageKind,
};

use super::Unparser;

/// The [`MessageTextUnparser`] trait provides methods to generate HTML and
/// Markdown representations of the text and captions in a Telegram message.
pub trait MessageTextUnparser {
    /// Returns the HTML representation of the message text, if the message
    /// contains text. This method will parse the text and any entities
    /// (such as bold, italic, links, etc.) and return the HTML-formatted
    /// string.
    fn html_text(&self) -> Option<String>;
    /// Returns the Markdown representation of the message text, if the message
    /// contains text. This method will parse the text and any entities
    /// (such as bold, italic, links, etc.) and return the
    /// Markdown-formatted string.
    fn markdown_text(&self) -> Option<String>;
    /// Returns the HTML representation of the message caption, if the message
    /// contains caption. This method will parse the caption and any
    /// entities (such as bold, italic, links, etc.) and return the
    /// HTML-formatted string.
    fn html_caption(&self) -> Option<String>;
    /// Returns the Markdown representation of the message caption, if the
    /// message contains caption. This method will parse the caption and any
    /// entities (such as bold, italic, links, etc.) and return the
    /// Markdown-formatted string.
    fn markdown_caption(&self) -> Option<String>;
}

impl MessageTextUnparser for Message {
    fn html_text(&self) -> Option<String> {
        match &self.kind {
            MessageKind::Common(MessageCommon {
                media_kind: MediaKind::Text(MediaText { text, entities, .. }),
                ..
            }) => Some(Unparser::new(text, entities).as_html()),
            _ => None,
        }
    }

    fn markdown_text(&self) -> Option<String> {
        match &self.kind {
            MessageKind::Common(MessageCommon {
                media_kind: MediaKind::Text(MediaText { text, entities, .. }),
                ..
            }) => Some(Unparser::new(text, entities).as_markdown()),
            _ => None,
        }
    }

    fn html_caption(&self) -> Option<String> {
        match &self.kind {
            MessageKind::Common(MessageCommon { media_kind, .. }) => match media_kind {
                MediaKind::Animation(MediaAnimation { caption, caption_entities, .. })
                | MediaKind::Audio(MediaAudio { caption, caption_entities, .. })
                | MediaKind::Document(MediaDocument { caption, caption_entities, .. })
                | MediaKind::Photo(MediaPhoto { caption, caption_entities, .. })
                | MediaKind::Video(MediaVideo { caption, caption_entities, .. })
                | MediaKind::Voice(MediaVoice { caption, caption_entities, .. }) => {
                    caption.as_ref().map(|c| Unparser::new(c, caption_entities).as_html())
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn markdown_caption(&self) -> Option<String> {
        match &self.kind {
            MessageKind::Common(MessageCommon { media_kind, .. }) => match media_kind {
                MediaKind::Animation(MediaAnimation { caption, caption_entities, .. })
                | MediaKind::Audio(MediaAudio { caption, caption_entities, .. })
                | MediaKind::Document(MediaDocument { caption, caption_entities, .. })
                | MediaKind::Photo(MediaPhoto { caption, caption_entities, .. })
                | MediaKind::Video(MediaVideo { caption, caption_entities, .. })
                | MediaKind::Voice(MediaVoice { caption, caption_entities, .. }) => {
                    caption.as_ref().map(|c| Unparser::new(c, caption_entities).as_markdown())
                }
                _ => None,
            },
            _ => None,
        }
    }
}
