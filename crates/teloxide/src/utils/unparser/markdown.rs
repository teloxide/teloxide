use crate::utils::markdown::ESCAPE_CHARS;

use super::{Tag, TagWriter};
use std::fmt::Write;

pub struct Markdown;

impl TagWriter for Markdown {
    const BOLD_START: &'static str = "**";
    const BOLD_END: &'static str = "**";
    const ITALIC_START: &'static str = "_\r";
    const ITALIC_END: &'static str = "_\r";
    const UNDERLINE_START: &'static str = "__\r"; // we use \r here in order to be able to combine underline with other things
                                                  // like italic
    const UNDERLINE_END: &'static str = "__\r";
    const STRIKETHROUGH_START: &'static str = "~";
    const STRIKETHROUGH_END: &'static str = "~";
    const SPOILER_START: &'static str = "||";
    const SPOILER_END: &'static str = "||";
    const CODE_START: &'static str = "`";
    const CODE_END: &'static str = "`";
    const PRE_NO_LANG_START: &'static str = "```\n";
    const PRE_NO_LANG_END: &'static str = "```\n";
    const PRE_START: &'static str = "```";
    const PRE_MIDDLE: &'static str = "\n";
    const PRE_END: &'static str = "```\n";
    const TEXT_LINK_START: &'static str = "[";
    const TEXT_LINK_MIDDLE: &'static str = "](";
    const TEXT_LINK_END: &'static str = ")";
    const TEXT_MENTION_START: &'static str = "[";
    const TEXT_MENTION_MIDDLE: &'static str = "](tg://user?id=";
    const TEXT_MENTION_END: &'static str = ")";
    const CUSTOM_EMOJI_START: &'static str = "[";
    const CUSTOM_EMOJI_MIDDLE: &'static str = "](tg://emoji?id=";
    const CUSTOM_EMOJI_END: &'static str = ")";

    fn write_tag(tag: &Tag, buf: &mut String) {
        match tag {
            Tag::BoldStart => buf.push_str(Self::BOLD_START),
            Tag::BoldEnd => buf.push_str(Self::BOLD_END),
            Tag::ItalicStart => buf.push_str(Self::ITALIC_START),
            Tag::ItalicEnd => buf.push_str(Self::ITALIC_END),
            Tag::UnderlineStart => buf.push_str(Self::UNDERLINE_START),
            Tag::UnderlineEnd => buf.push_str(Self::UNDERLINE_END),
            Tag::StrikethroughStart => buf.push_str(Self::STRIKETHROUGH_START),
            Tag::StrikethroughEnd => buf.push_str(Self::STRIKETHROUGH_END),
            Tag::SpoilerStart => buf.push_str(Self::SPOILER_START),
            Tag::SpoilerEnd => buf.push_str(Self::SPOILER_END),
            Tag::CodeStart => buf.push_str(Self::CODE_START),
            Tag::CodeEnd => buf.push_str(Self::CODE_END),
            Tag::PreStart(lang) => match lang {
                Some(lang) => {
                    write!(buf, "{}{}{}", Self::PRE_START, lang, Self::PRE_MIDDLE).unwrap()
                }
                None => buf.push_str(Self::PRE_NO_LANG_START),
            },
            Tag::PreEnd(have_lang) => {
                buf.push_str(if *have_lang { Self::PRE_END } else { Self::PRE_NO_LANG_END })
            }
            Tag::TextLinkStart(_) => buf.push_str(Self::TEXT_LINK_START),
            Tag::TextLinkEnd(url) => {
                write!(buf, "{}{}{}", Self::TEXT_LINK_MIDDLE, url, Self::TEXT_LINK_END).unwrap()
            }
            Tag::TextMentionStart(_) => buf.push_str(Self::TEXT_MENTION_START),
            Tag::TextMentionEnd(id) => {
                write!(buf, "{}{}{}", Self::TEXT_MENTION_MIDDLE, id, Self::TEXT_MENTION_END)
                    .unwrap()
            }
            Tag::CustomEmojiStart(_) => buf.push_str(Self::CUSTOM_EMOJI_START),
            Tag::CustomEmojiEnd(custom_emoji_id) => write!(
                buf,
                "{}{}{}",
                Self::CUSTOM_EMOJI_MIDDLE,
                custom_emoji_id,
                Self::CUSTOM_EMOJI_END
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
