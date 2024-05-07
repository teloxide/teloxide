use super::{Tag, TagWriter};
use std::fmt::Write;

pub struct Html;

impl TagWriter for Html {
    const BOLD_START: &'static str = "<b>";
    const BOLD_END: &'static str = "</b>";
    const ITALIC_START: &'static str = "<i>";
    const ITALIC_END: &'static str = "</i>";
    const UNDERLINE_START: &'static str = "<u>";
    const UNDERLINE_END: &'static str = "</u>";
    const STRIKETHROUGH_START: &'static str = "<s>";
    const STRIKETHROUGH_END: &'static str = "</s>";
    const SPOILER_START: &'static str = "<tg-spoiler>";
    const SPOILER_END: &'static str = "</tg-spoiler>";
    const CODE_START: &'static str = "<code>";
    const CODE_END: &'static str = "</code>";
    const PRE_NO_LANG_START: &'static str = "<pre>";
    const PRE_NO_LANG_END: &'static str = "</pre>";
    const PRE_START: &'static str = "<pre><code class=\"language-";
    const PRE_MIDDLE: &'static str = "\">";
    const PRE_END: &'static str = "</code></pre>";
    const TEXT_LINK_START: &'static str = "<a href=\"";
    const TEXT_LINK_MIDDLE: &'static str = "\">";
    const TEXT_LINK_END: &'static str = "</a>";
    const TEXT_MENTION_START: &'static str = "<a href=\"tg://user?id=";
    const TEXT_MENTION_MIDDLE: &'static str = "\">";
    const TEXT_MENTION_END: &'static str = "</a>";
    const CUSTOM_EMOJI_START: &'static str = "<tg-emoji emoji-id=\"";
    const CUSTOM_EMOJI_MIDDLE: &'static str = "\">";
    const CUSTOM_EMOJI_END: &'static str = "</tg-emoji>";

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
            Tag::TextLinkStart(url) => {
                write!(buf, "{}{}{}", Self::TEXT_LINK_START, url, Self::TEXT_LINK_MIDDLE).unwrap()
            }
            Tag::TextLinkEnd(_) => buf.push_str(Self::TEXT_LINK_END),
            Tag::TextMentionStart(id) => {
                write!(buf, "{}{}{}", Self::TEXT_MENTION_START, id, Self::TEXT_MENTION_MIDDLE)
                    .unwrap()
            }
            Tag::TextMentionEnd(_) => buf.push_str(Self::TEXT_MENTION_END),
            Tag::CustomEmojiStart(custom_emoji_id) => write!(
                buf,
                "{}{}{}",
                Self::CUSTOM_EMOJI_START,
                custom_emoji_id,
                Self::CUSTOM_EMOJI_MIDDLE
            )
            .unwrap(),
            Tag::CustomEmojiEnd(_) => buf.push_str(Self::CUSTOM_EMOJI_END),
        }
    }

    fn write_char(ch: char, buf: &mut String) {
        match ch {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            c => buf.push(c),
        }
    }
}
