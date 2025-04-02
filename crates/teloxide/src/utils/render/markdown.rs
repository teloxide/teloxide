use std::fmt::Write;

use crate::utils::markdown::ESCAPE_CHARS;

use super::{ComplexTag, Kind, NewLineRepeatedTag, Place, SimpleTag, Tag, TagWriter};

pub static MARKDOWN: TagWriter = TagWriter {
    bold: SimpleTag::new("*", "*"),
    blockquote: NewLineRepeatedTag::new("**>", ">", ""),
    expandable_blockquote: NewLineRepeatedTag::new("**>", ">", "||"),
    italic: SimpleTag::new("_\r", "_\r"),
    underline: SimpleTag::new("__\r", "__\r"),
    strikethrough: SimpleTag::new("~", "~"),
    spoiler: SimpleTag::new("||", "||"),
    code: SimpleTag::new("`", "`"),
    pre_no_lang: SimpleTag::new("```\n", "```\n"),
    pre: ComplexTag::new("```", "\n", "```\n"),
    text_link: ComplexTag::new("[", "](", ")"),
    text_mention: ComplexTag::new("[", "](tg://user?id=", ")"),
    custom_emoji: ComplexTag::new("[", "](tg://emoji?id=", ")"),
    write_tag_fn: write_tag,
    write_char_fn: write_char,
};

fn write_tag(tag: &Tag, buf: &mut String) {
    match tag.kind {
        Kind::Bold => buf.push_str(MARKDOWN.bold.get_tag(tag.place)),
        Kind::Blockquote => match tag.place {
            Place::Start => buf.push_str(MARKDOWN.blockquote.start),
            Place::MidNewLine => buf.push_str(MARKDOWN.blockquote.repeat),
            Place::End => buf.push_str(MARKDOWN.blockquote.end),
        },
        Kind::ExpandableBlockquote => match tag.place {
            Place::Start => buf.push_str(MARKDOWN.expandable_blockquote.start),
            Place::MidNewLine => buf.push_str(MARKDOWN.expandable_blockquote.repeat),
            Place::End => buf.push_str(MARKDOWN.expandable_blockquote.end),
        },
        Kind::Italic => buf.push_str(MARKDOWN.italic.get_tag(tag.place)),
        Kind::Underline => buf.push_str(MARKDOWN.underline.get_tag(tag.place)),
        Kind::Strikethrough => buf.push_str(MARKDOWN.strikethrough.get_tag(tag.place)),
        Kind::Spoiler => buf.push_str(MARKDOWN.spoiler.get_tag(tag.place)),
        Kind::Code => buf.push_str(MARKDOWN.code.get_tag(tag.place)),
        Kind::Pre(lang) => match tag.place {
            Place::Start => match lang {
                Some(lang) => {
                    write!(buf, "{}{}{}", MARKDOWN.pre.start, lang, MARKDOWN.pre.middle).unwrap()
                }
                None => buf.push_str(MARKDOWN.pre_no_lang.start),
            },
            Place::MidNewLine => unreachable!(),
            Place::End => buf.push_str(lang.map_or(MARKDOWN.pre_no_lang.end, |_| MARKDOWN.pre.end)),
        },
        Kind::TextLink(url) => match tag.place {
            Place::Start => buf.push_str(MARKDOWN.text_link.start),
            Place::MidNewLine => unreachable!(),
            Place::End => {
                write!(buf, "{}{}{}", MARKDOWN.text_link.middle, url, MARKDOWN.text_link.end)
                    .unwrap()
            }
        },
        Kind::TextMention(id) => match tag.place {
            Place::Start => buf.push_str(MARKDOWN.text_mention.start),
            Place::MidNewLine => unreachable!(),
            Place::End => {
                write!(buf, "{}{}{}", MARKDOWN.text_mention.middle, id, MARKDOWN.text_mention.end)
                    .unwrap()
            }
        },
        Kind::CustomEmoji(custom_emoji_id) => match tag.place {
            Place::Start => buf.push_str(MARKDOWN.custom_emoji.start),
            Place::MidNewLine => unreachable!(),
            Place::End => write!(
                buf,
                "{}{}{}",
                MARKDOWN.custom_emoji.middle, custom_emoji_id, MARKDOWN.custom_emoji.end
            )
            .unwrap(),
        },
    }
}

fn write_char(ch: char, buf: &mut String) {
    if ESCAPE_CHARS.contains(&ch) {
        buf.push('\\');
    }
    buf.push(ch);
}
