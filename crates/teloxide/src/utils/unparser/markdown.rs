use std::fmt::Write;

use crate::utils::markdown::ESCAPE_CHARS;

use super::{ComplexTag, SimpleTag, Tag, TagWriter};

pub struct Markdown;

impl TagWriter for Markdown {
    const BOLD: SimpleTag = SimpleTag::new("**", "**");
    const ITALIC: SimpleTag = SimpleTag::new("_\r", "_\r");
    const UNDERLINE: SimpleTag = SimpleTag::new("__\r", "__\r");
    const STRIKETHROUGH: SimpleTag = SimpleTag::new("~", "~");
    const SPOILER: SimpleTag = SimpleTag::new("||", "||");
    const CODE: SimpleTag = SimpleTag::new("`", "`");
    const PRE_NO_LANG: SimpleTag = SimpleTag::new("```\n", "```\n");
    const PRE: ComplexTag = ComplexTag::new("```", "\n", "```\n");
    const TEXT_LINK: ComplexTag = ComplexTag::new("[", "](", ")");
    const TEXT_MENTION: ComplexTag = ComplexTag::new("[", "](tg://user?id=", ")");
    const CUSTOM_EMOJI: ComplexTag = ComplexTag::new("[", "](tg://emoji?id=", ")");

    fn write_tag(tag: &Tag, buf: &mut String) {
        match tag {
            Tag::BoldStart => buf.push_str(Self::BOLD.start),
            Tag::BoldEnd => buf.push_str(Self::BOLD.end),
            Tag::ItalicStart => buf.push_str(Self::ITALIC.start),
            Tag::ItalicEnd => buf.push_str(Self::ITALIC.end),
            Tag::UnderlineStart => buf.push_str(Self::UNDERLINE.start),
            Tag::UnderlineEnd => buf.push_str(Self::UNDERLINE.end),
            Tag::StrikethroughStart => buf.push_str(Self::STRIKETHROUGH.start),
            Tag::StrikethroughEnd => buf.push_str(Self::STRIKETHROUGH.end),
            Tag::SpoilerStart => buf.push_str(Self::SPOILER.start),
            Tag::SpoilerEnd => buf.push_str(Self::SPOILER.end),
            Tag::CodeStart => buf.push_str(Self::CODE.start),
            Tag::CodeEnd => buf.push_str(Self::CODE.end),
            Tag::PreStart(lang) => match lang {
                Some(lang) => {
                    write!(buf, "{}{}{}", Self::PRE.start, lang, Self::PRE.middle).unwrap()
                }
                None => buf.push_str(Self::PRE_NO_LANG.start),
            },
            Tag::PreEnd(have_lang) => {
                buf.push_str(if *have_lang { Self::PRE.end } else { Self::PRE_NO_LANG.end })
            }
            Tag::TextLinkStart(_) => buf.push_str(Self::TEXT_LINK.start),
            Tag::TextLinkEnd(url) => {
                write!(buf, "{}{}{}", Self::TEXT_LINK.middle, url, Self::TEXT_LINK.end).unwrap()
            }
            Tag::TextMentionStart(_) => buf.push_str(Self::TEXT_MENTION.start),
            Tag::TextMentionEnd(id) => {
                write!(buf, "{}{}{}", Self::TEXT_MENTION.middle, id, Self::TEXT_MENTION.end)
                    .unwrap()
            }
            Tag::CustomEmojiStart(_) => buf.push_str(Self::CUSTOM_EMOJI.start),
            Tag::CustomEmojiEnd(custom_emoji_id) => write!(
                buf,
                "{}{}{}",
                Self::CUSTOM_EMOJI.middle,
                custom_emoji_id,
                Self::CUSTOM_EMOJI.end
            )
            .unwrap(),
        }
    }

    fn write_char(ch: char, buf: &mut String) {
        if ESCAPE_CHARS.contains(&ch) {
            buf.push('\\');
        }
        buf.push(ch);
    }
}
