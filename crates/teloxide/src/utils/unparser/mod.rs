//! Message entity unparser

use std::cmp::Ordering;

use teloxide_core::types::{MessageEntity, MessageEntityKind as MEK};

use html::Html;
use markdown::Markdown;

pub use helper::MessageTextUnparser;

mod helper;
mod html;
mod markdown;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Unparser<'a> {
    text: &'a str,
    pos_tags: Vec<(Position, Tag<'a>)>,
}

impl<'a> Unparser<'a> {
    /// Creates a new `Unparser` instance with the given text and entities.
    ///
    /// The `Unparser` is responsible for parsing the text and entities to
    /// produce the final formatted output. This constructor sets up the
    /// initial state needed for the parsing process.
    ///
    /// # Arguments
    ///
    /// - `text`: The input text to be parsed.
    /// - `entities`: The message entities (formatting, links, etc.) to be
    ///   applied to the text.
    ///
    /// # Returns
    ///
    /// A new [`Unparser`] instance.
    #[must_use]
    pub fn new(text: &'a str, entities: &'a [MessageEntity]) -> Self {
        // get the needed size for the new tags that we want to parse from entities
        let needed_size: usize = entities
            .iter()
            .map(|e| match e.kind {
                MEK::Bold
                | MEK::Italic
                | MEK::Underline
                | MEK::Strikethrough
                | MEK::Spoiler
                | MEK::Code
                | MEK::Pre { .. }
                | MEK::TextLink { .. }
                | MEK::TextMention { .. }
                | MEK::CustomEmoji { .. } => 2,
                _ => 0,
            })
            .sum();

        let mut pos_tags = Vec::with_capacity(needed_size);

        macro_rules! before_after {
            ($before:expr, $after:expr, $index:expr, $entity:expr $(,)?) => {{
                pos_tags.push((
                    Position { offset: $entity.offset, side: Side::Before, index: $index },
                    $before,
                ));
                pos_tags.push((
                    Position {
                        offset: $entity.offset + $entity.length,
                        side: Side::After,
                        index: $index,
                    },
                    $after,
                ));
            }};
        }

        entities.iter().enumerate().for_each(|(index, entity)| match &entity.kind {
            MEK::Bold => {
                before_after!(Tag::BoldStart, Tag::BoldEnd, index, entity)
            }
            MEK::Italic => {
                before_after!(Tag::ItalicStart, Tag::ItalicEnd, index, entity)
            }
            MEK::Underline => {
                before_after!(Tag::UnderlineStart, Tag::UnderlineEnd, index, entity)
            }
            MEK::Strikethrough => {
                before_after!(Tag::StrikethroughStart, Tag::StrikethroughEnd, index, entity,)
            }
            MEK::Spoiler => {
                before_after!(Tag::SpoilerStart, Tag::SpoilerEnd, index, entity)
            }
            MEK::Code => {
                before_after!(Tag::CodeStart, Tag::CodeEnd, index, entity)
            }
            MEK::Pre { language } => before_after!(
                Tag::PreStart(language.as_ref().map(String::as_str)),
                Tag::PreEnd(language.is_some()),
                index,
                entity,
            ),
            MEK::TextLink { url } => before_after!(
                Tag::TextLinkStart(url.as_str()),
                Tag::TextLinkEnd(url.as_str()),
                index,
                entity,
            ),
            MEK::TextMention { user } => before_after!(
                Tag::TextMentionStart(user.id.0),
                Tag::TextMentionEnd(user.id.0),
                index,
                entity,
            ),
            MEK::CustomEmoji { custom_emoji_id } => before_after!(
                Tag::CustomEmojiStart(custom_emoji_id),
                Tag::CustomEmojiEnd(custom_emoji_id),
                index,
                entity,
            ),
            _ => (),
        });

        pos_tags.sort_unstable_by(|(p1, _), (p2, _)| p1.cmp(p2));

        Self { text, pos_tags }
    }

    /// Unparsers the text with the given [`TagWriter`] implementation.
    ///
    /// This method iterates through the text and the associated position tags,
    /// and writes the text with the appropriate tags to a buffer. The
    /// resulting buffer is then returned as a `String`.
    ///
    /// If the `pos_tags` vector is empty, the original text is returned as-is.
    #[must_use]
    fn unparse<T>(&self) -> String
    where
        T: TagWriter,
    {
        if self.pos_tags.is_empty() {
            return self.text.to_owned();
        }

        let mut buffer = String::with_capacity(self.text.len() + T::get_tags_sizes(&self.pos_tags));
        let mut pos_tags = self.pos_tags.iter();
        let mut current_tag = pos_tags.next();

        let mut prev_point = None;

        for (idx, point) in self.text.encode_utf16().enumerate() {
            loop {
                match current_tag {
                    Some((pos, tag)) if pos.offset == idx => {
                        T::write_tag(tag, &mut buffer);
                        current_tag = pos_tags.next();
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

            T::write_char(ch, &mut buffer);
        }

        if let Some((_, tag)) = current_tag {
            T::write_tag(tag, &mut buffer);
        }

        for (_, tag) in pos_tags {
            T::write_tag(tag, &mut buffer);
        }

        buffer
    }

    /// Render and return the text as Html-Formatted string.
    #[must_use]
    pub fn as_html(&self) -> String {
        self.unparse::<Html>()
    }

    /// Render and return the text as Markdown-Formatted string.
    #[must_use]
    pub fn as_markdown(&self) -> String {
        self.unparse::<Markdown>()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tag<'a> {
    BoldStart,
    BoldEnd,
    ItalicStart,
    ItalicEnd,
    UnderlineStart,
    UnderlineEnd,
    StrikethroughStart,
    StrikethroughEnd,
    SpoilerStart,
    SpoilerEnd,
    CodeStart,
    CodeEnd,
    PreStart(Option<&'a str>),
    PreEnd(bool),
    TextLinkStart(&'a str),
    TextLinkEnd(&'a str),
    TextMentionStart(u64),
    TextMentionEnd(u64),
    CustomEmojiStart(&'a str),
    CustomEmojiEnd(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    After,
    Before,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    offset: usize,
    side: Side,
    index: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset).then_with(|| self.side.cmp(&other.side)).then_with(|| {
            match self.side {
                Side::Before => self.index.cmp(&other.index),
                Side::After => other.index.cmp(&self.index),
            }
        })
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait TagWriter {
    const BOLD_START: &'static str;
    const BOLD_END: &'static str;

    const ITALIC_START: &'static str;
    const ITALIC_END: &'static str;

    const UNDERLINE_START: &'static str;
    const UNDERLINE_END: &'static str;

    const STRIKETHROUGH_START: &'static str;
    const STRIKETHROUGH_END: &'static str;

    const SPOILER_START: &'static str;
    const SPOILER_END: &'static str;

    const CODE_START: &'static str;
    const CODE_END: &'static str;

    const PRE_NO_LANG_START: &'static str;
    const PRE_NO_LANG_END: &'static str;

    const PRE_START: &'static str;
    const PRE_MIDDLE: &'static str;
    const PRE_END: &'static str;

    const TEXT_LINK_START: &'static str;
    const TEXT_LINK_MIDDLE: &'static str;
    const TEXT_LINK_END: &'static str;

    const TEXT_MENTION_START: &'static str;
    const TEXT_MENTION_MIDDLE: &'static str;
    const TEXT_MENTION_END: &'static str;

    const CUSTOM_EMOJI_START: &'static str;
    const CUSTOM_EMOJI_MIDDLE: &'static str;
    const CUSTOM_EMOJI_END: &'static str;

    // TODO: add Blockquote when its added

    /// Get the extra size needed for tags
    fn get_tags_sizes(tags: &[(Position, Tag)]) -> usize {
        tags.iter()
            .map(|(_, t)| match t {
                Tag::BoldStart => Self::BOLD_START.len(),
                Tag::BoldEnd => Self::BOLD_END.len(),
                Tag::ItalicStart => Self::ITALIC_START.len(),
                Tag::ItalicEnd => Self::ITALIC_END.len(),
                Tag::UnderlineStart => Self::UNDERLINE_START.len(),
                Tag::UnderlineEnd => Self::UNDERLINE_END.len(),
                Tag::StrikethroughStart => Self::STRIKETHROUGH_START.len(),
                Tag::StrikethroughEnd => Self::STRIKETHROUGH_END.len(),
                Tag::SpoilerStart => Self::SPOILER_START.len(),
                Tag::SpoilerEnd => Self::SPOILER_END.len(),
                Tag::CodeStart => Self::CODE_START.len(),
                Tag::CodeEnd => Self::CODE_END.len(),
                Tag::PreStart(lang) => {
                    lang.map_or(Self::PRE_NO_LANG_START.len(), |l| Self::PRE_START.len() + l.len())
                }
                Tag::PreEnd(have_lang) => {
                    if *have_lang {
                        Self::PRE_MIDDLE.len() + Self::PRE_END.len()
                    } else {
                        Self::PRE_NO_LANG_END.len()
                    }
                }
                Tag::TextLinkStart(url) => Self::TEXT_LINK_START.len() + url.len(),
                Tag::TextLinkEnd(_) => Self::TEXT_LINK_MIDDLE.len() + Self::TEXT_LINK_END.len(),
                Tag::TextMentionStart(id) => {
                    Self::TEXT_MENTION_START.len() + id.ilog10() as usize + 1
                }
                Tag::TextMentionEnd(_) => {
                    Self::TEXT_MENTION_MIDDLE.len() + Self::TEXT_MENTION_END.len()
                }
                Tag::CustomEmojiStart(custom_emoji_id) => {
                    Self::CUSTOM_EMOJI_START.len() + custom_emoji_id.len()
                }
                Tag::CustomEmojiEnd(_) => {
                    Self::CUSTOM_EMOJI_MIDDLE.len() + Self::CUSTOM_EMOJI_END.len()
                }
            })
            .sum()
    }

    /// Write the tag to buffer
    fn write_tag(tag: &Tag, buf: &mut String);
    /// Write the char to buffer and escape characters if needed
    fn write_char(ch: char, buf: &mut String);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unparser_simple() {
        let text = "Bold italic <underline_";
        let entities = vec![
            MessageEntity { kind: MEK::Bold, offset: 0, length: 4 },
            MessageEntity { kind: MEK::Italic, offset: 5, length: 6 },
            MessageEntity { kind: MEK::Underline, offset: 12, length: 10 },
        ];

        let unparser = Unparser::new(text, &entities);

        assert_eq!(unparser.as_html(), "<b>Bold</b> <i>italic</i> <u>&lt;underline</u>_");
        assert_eq!(unparser.as_markdown(), "**Bold** _\ritalic_\r __\r<underline__\r\\_");
    }

    #[test]
    fn test_unparser_pre_with_lang() {
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

        let unparser = Unparser::new(text, &entities);

        assert_eq!(
            unparser.as_html(),
            "Some <pre>pre</pre>, <code>normal</code> and <pre><code \
             class=\"language-rust\">rusty</code></pre> code",
        );
        assert_eq!(
            unparser.as_markdown(),
            "Some ```\npre```\n, `normal` and ```rust\nrusty```\n code",
        );
    }

    #[test]
    fn test_unparser_nested() {
        let text = "Some bold both italics";
        let entities = vec![
            MessageEntity { kind: MEK::Bold, offset: 5, length: 9 },
            MessageEntity { kind: MEK::Italic, offset: 10, length: 12 },
        ];

        let unparser = Unparser::new(text, &entities);

        assert_eq!(unparser.as_html(), "Some <b>bold <i>both</b> italics</i>");
        assert_eq!(unparser.as_markdown(), "Some **bold _\rboth** italics_\r");
    }

    #[test]
    fn test_unparser_complex() {
        let text = "Hi how are you?\nnested entities are cool";
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
        ];

        let unparser = Unparser::new(text, &entities);

        assert_eq!(
            unparser.as_html(),
            "<b>Hi</b> <i>how</i> <u>are</u> <s>you</s>?\n<b>n</b><b><u><s>este</s></u>d</b> \
            <a href=\"https://t.me/\">entities</a> <a href=\"tg://user?id=1234567\">are</a> <code>cool</code>"
        );
        assert_eq!(
            unparser.as_markdown(),
            "**Hi** _\rhow_\r __\rare__\r ~you~?\n**n****__\r~este~__\rd** [entities](https://t.me/) \
             [are](tg://user?id=1234567) `cool`"
        );
    }
}
