//! A helpful trait for rendering text/caption and entities back to HTML or
//! Markdown.

use teloxide_core::types::Message;

use super::Renderer;

/// Generates HTML and Markdown representations of text and captions in a
/// Telegram message.
pub trait RenderMessageTextHelper {
    /// Returns the HTML representation of the message text, if the message
    /// contains text. This method will parse the text and any entities
    /// (such as bold, italic, links, etc.) and return the HTML-formatted
    /// string.
    #[must_use]
    fn html_text(&self) -> Option<String>;

    /// Returns the Markdown representation of the message text, if the message
    /// contains text. This method will parse the text and any entities
    /// (such as bold, italic, links, etc.) and return the
    /// Markdown-formatted string.
    #[must_use]
    fn markdown_text(&self) -> Option<String>;

    /// Returns the HTML representation of the message caption, if the message
    /// contains caption. This method will parse the caption and any
    /// entities (such as bold, italic, links, etc.) and return the
    /// HTML-formatted string.
    #[must_use]
    fn html_caption(&self) -> Option<String>;

    /// Returns the Markdown representation of the message caption, if the
    /// message contains caption. This method will parse the caption and any
    /// entities (such as bold, italic, links, etc.) and return the
    /// Markdown-formatted string.
    #[must_use]
    fn markdown_caption(&self) -> Option<String>;
}

impl RenderMessageTextHelper for Message {
    fn html_text(&self) -> Option<String> {
        self.text()
            .zip(self.entities())
            .map(|(text, entities)| Renderer::new(text, entities).as_html())
    }

    fn markdown_text(&self) -> Option<String> {
        self.text()
            .zip(self.entities())
            .map(|(text, entities)| Renderer::new(text, entities).as_markdown())
    }

    fn html_caption(&self) -> Option<String> {
        self.caption()
            .zip(self.caption_entities())
            .map(|(text, entities)| Renderer::new(text, entities).as_html())
    }

    fn markdown_caption(&self) -> Option<String> {
        self.caption()
            .zip(self.caption_entities())
            .map(|(text, entities)| Renderer::new(text, entities).as_markdown())
    }
}
