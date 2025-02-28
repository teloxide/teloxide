//! Utils for rendering HTML and Markdown output.

use teloxide_core::types::{MessageEntity, MessageEntityKind as MEK};

use tag::*;

pub use helper::RenderMessageTextHelper;

mod helper;
mod html;
mod markdown;
mod tag;

/// Parses text and message entities to produce the final formatted output.
#[derive(Clone, Eq, PartialEq)]
pub struct Renderer<'a> {
    text: &'a str,
    tags: Vec<Tag<'a>>,
}

impl<'a> Renderer<'a> {
    /// Creates a new [`Renderer`] instance with given text and message
    /// entities.
    ///
    /// # Arguments
    ///
    /// - `text`: The input text to be parsed.
    /// - `entities`: The message entities (formatting, links, etc.) to be
    ///   applied to the text.
    #[must_use]
    pub fn new(text: &'a str, entities: &'a [MessageEntity]) -> Self {
        // get the needed size for the new tags that we want to parse from entities
        let needed_size: usize = entities
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    MEK::Bold
                        | MEK::Blockquote
                        | MEK::ExpandableBlockquote
                        | MEK::Italic
                        | MEK::Underline
                        | MEK::Strikethrough
                        | MEK::Spoiler
                        | MEK::Code
                        | MEK::Pre { .. }
                        | MEK::TextLink { .. }
                        | MEK::TextMention { .. }
                        | MEK::CustomEmoji { .. }
                )
            })
            .count()
            * 2; // 2 because we insert two tags for each entity

        let mut tags = Vec::with_capacity(needed_size);

        for (index, entity) in entities.iter().enumerate() {
            let kind = match &entity.kind {
                MEK::Bold => Kind::Bold,
                MEK::Blockquote => Kind::Blockquote,
                MEK::ExpandableBlockquote => Kind::ExpandableBlockquote,
                MEK::Italic => Kind::Italic,
                MEK::Underline => Kind::Underline,
                MEK::Strikethrough => Kind::Strikethrough,
                MEK::Spoiler => Kind::Spoiler,
                MEK::Code => Kind::Code,
                MEK::Pre { language } => Kind::Pre(language.as_ref().map(String::as_str)),
                MEK::TextLink { url } => Kind::TextLink(url.as_str()),
                MEK::TextMention { user } => Kind::TextMention(user.id.0),
                MEK::CustomEmoji { custom_emoji_id } => Kind::CustomEmoji(custom_emoji_id),
                _ => continue,
            };

            // FIXME: maybe instead of clone store all the `kind`s in a seperate
            // vector and then just store the index here?
            tags.push(Tag::start(kind.clone(), entity.offset, index));

            if matches!(kind, Kind::Blockquote | Kind::ExpandableBlockquote) {
                let new_lines_indexes: Vec<usize> = text
                    .chars()
                    .skip(entity.offset)
                    .take(entity.length)
                    .enumerate()
                    .filter_map(|(idx, c)| (c == '\n').then_some(idx))
                    .collect();

                for new_line_index in new_lines_indexes.iter() {
                    tags.push(Tag::mid_new_line(
                        kind.clone(),
                        entity.offset + new_line_index + 1,
                        index,
                    ));
                }
            }

            tags.push(Tag::end(kind, entity.offset + entity.length, index));
        }

        tags.sort_unstable();

        Self { text, tags }
    }

    /// Renders text with a given [`TagWriter`].
    ///
    /// This method iterates through the text and the associated position tags
    /// and writes the text with the appropriate tags to a buffer, which is then
    /// returned as a `String`.
    ///
    /// If input have no tags we just return the original text as-is.
    #[must_use]
    fn format(&self, writer: &TagWriter) -> String {
        if self.tags.is_empty() {
            return self.text.to_owned();
        }

        let mut buffer =
            String::with_capacity(self.text.len() + writer.get_extra_size_for_tags(&self.tags));
        let mut tags = self.tags.iter();
        let mut current_tag = tags.next();

        let mut prev_point = None;

        for (idx, point) in self.text.encode_utf16().enumerate() {
            loop {
                match current_tag {
                    Some(tag) if tag.offset == idx => {
                        (writer.write_tag_fn)(tag, &mut buffer);
                        current_tag = tags.next();
                    }
                    _ => break,
                }
            }

            let ch = if let Some(previous) = prev_point.take() {
                char::decode_utf16([previous, point]).next().unwrap().unwrap()
            } else {
                match char::decode_utf16([point]).next().unwrap() {
                    Ok(c) => c,
                    Err(unpaired) => {
                        prev_point = Some(unpaired.unpaired_surrogate());
                        continue;
                    }
                }
            };

            (writer.write_char_fn)(ch, &mut buffer);
        }

        for tag in current_tag.into_iter().chain(tags) {
            (writer.write_tag_fn)(tag, &mut buffer);
        }

        buffer
    }

    /// Renders and returns the text as an **HTML-formatted** string.
    #[must_use]
    #[inline]
    pub fn as_html(&self) -> String {
        self.format(&html::HTML)
    }

    /// Renders and returns the text as a **MarkdownV2-formatted** string.
    #[must_use]
    #[inline]
    pub fn as_markdown(&self) -> String {
        self.format(&markdown::MARKDOWN)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_simple() {
        let text = "Bold italic <underline_";
        let entities = vec![
            MessageEntity { kind: MEK::Bold, offset: 0, length: 4 },
            MessageEntity { kind: MEK::Italic, offset: 5, length: 6 },
            MessageEntity { kind: MEK::Underline, offset: 12, length: 10 },
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(render.as_html(), "<b>Bold</b> <i>italic</i> <u>&lt;underline</u>_");
        assert_eq!(render.as_markdown(), "*Bold* _\ritalic_\r __\r<underline__\r\\_");
    }

    #[test]
    fn test_render_pre_with_lang() {
        let text = "Some pre, normal and rusty code";
        let entities = vec![
            MessageEntity { kind: MEK::Pre { language: None }, offset: 5, length: 3 },
            MessageEntity { kind: MEK::Code, offset: 10, length: 6 },
            MessageEntity {
                kind: MEK::Pre { language: Some("rust".to_owned()) },
                offset: 21,
                length: 5,
            },
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(
            render.as_html(),
            "Some <pre>pre</pre>, <code>normal</code> and <pre><code \
             class=\"language-rust\">rusty</code></pre> code",
        );
        assert_eq!(
            render.as_markdown(),
            "Some ```\npre```\n, `normal` and ```rust\nrusty```\n code",
        );
    }

    #[test]
    fn test_render_nested() {
        let text = "Some bold both italics";
        let entities = vec![
            MessageEntity { kind: MEK::Bold, offset: 5, length: 9 },
            MessageEntity { kind: MEK::Italic, offset: 10, length: 12 },
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(render.as_html(), "Some <b>bold <i>both</b> italics</i>");
        assert_eq!(render.as_markdown(), "Some *bold _\rboth* italics_\r");
    }

    #[test]
    fn test_render_complex() {
        let text = "Hi how are you?\nnested entities are cool\nIm in a Blockquote!\nIm in a \
                    multiline Blockquote!\n\nIm in a multiline Blockquote!\nIm in an expandable \
                    Blockquote!\nIm in an expandable multiline Blockquote!\n\nIm in an expandable \
                    multiline Blockquote!";
        let entities = vec![
            MessageEntity { kind: MEK::Bold, offset: 0, length: 2 },
            MessageEntity { kind: MEK::Italic, offset: 3, length: 3 },
            MessageEntity { kind: MEK::Underline, offset: 7, length: 3 },
            MessageEntity { kind: MEK::Strikethrough, offset: 11, length: 3 },
            MessageEntity { kind: MEK::Bold, offset: 16, length: 1 },
            MessageEntity { kind: MEK::Bold, offset: 17, length: 5 },
            MessageEntity { kind: MEK::Underline, offset: 17, length: 4 },
            MessageEntity { kind: MEK::Strikethrough, offset: 17, length: 4 },
            MessageEntity {
                kind: MEK::TextLink { url: reqwest::Url::parse("https://t.me/").unwrap() },
                offset: 23,
                length: 8,
            },
            MessageEntity {
                kind: MEK::TextLink { url: reqwest::Url::parse("tg://user?id=1234567").unwrap() },
                offset: 32,
                length: 3,
            },
            MessageEntity { kind: MEK::Code, offset: 36, length: 4 },
            MessageEntity { kind: MEK::Blockquote, offset: 41, length: 19 },
            MessageEntity { kind: MEK::Blockquote, offset: 61, length: 60 },
            MessageEntity { kind: MEK::ExpandableBlockquote, offset: 122, length: 31 },
            MessageEntity { kind: MEK::ExpandableBlockquote, offset: 154, length: 84 },
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(
            render.as_html(),
            "<b>Hi</b> <i>how</i> <u>are</u> <s>you</s>?\n<b>n</b><b><u><s>este</s></u>d</b> \
            <a href=\"https://t.me/\">entities</a> <a href=\"tg://user?id=1234567\">are</a> <code>cool</code>\n\
            <blockquote>Im in a Blockquote!</blockquote>\n\
            <blockquote>Im in a multiline Blockquote!\n\nIm in a multiline Blockquote!</blockquote>\n\
            <blockquote expandable>Im in an expandable Blockquote!</blockquote>\n\
            <blockquote expandable>Im in an expandable multiline Blockquote!\n\nIm in an expandable multiline Blockquote!</blockquote>"
        );
        assert_eq!(
            render.as_markdown(),
            "*Hi* _\rhow_\r __\rare__\r ~you~?\n*n**__\r~este~__\rd* [entities](https://t.me/) \
             [are](tg://user?id=1234567) `cool`\n**>Im in a Blockquote\\!\n**>Im in a multiline \
             Blockquote\\!\n>\n>Im in a multiline Blockquote\\!\n**>Im in an expandable \
             Blockquote\\!||\n**>Im in an expandable multiline Blockquote\\!\n>\n>Im in an \
             expandable multiline Blockquote\\!||"
        );
    }
}
