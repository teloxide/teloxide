use std::fmt::Write;

use super::{ComplexTag, SimpleTag, Tag, TagWriter};

pub struct Html;

impl TagWriter for Html {
    const BOLD: SimpleTag = SimpleTag::new("<b>", "</b>");
    const ITALIC: SimpleTag = SimpleTag::new("<i>", "</i>");
    const UNDERLINE: SimpleTag = SimpleTag::new("<u>", "</u>");
    const STRIKETHROUGH: SimpleTag = SimpleTag::new("<s>", "</s>");
    const SPOILER: SimpleTag = SimpleTag::new("<tg-spoiler>", "</tg-spoiler>");
    const CODE: SimpleTag = SimpleTag::new("<code>", "</code>");
    const PRE_NO_LANG: SimpleTag = SimpleTag::new("<pre>", "</pre>");
    const PRE: ComplexTag = ComplexTag::new("<pre><code class=\"language-", "\">", "</code></pre>");
    const TEXT_LINK: ComplexTag = ComplexTag::new("<a href=\"", "\">", "</a>");
    const TEXT_MENTION: ComplexTag = ComplexTag::new("<a href=\"tg://user?id=", "\">", "</a>");
    const CUSTOM_EMOJI: ComplexTag = ComplexTag::new("<tg-emoji emoji-id=\"", "\">", "</tg-emoji>");

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
            Tag::TextLinkStart(url) => {
                write!(buf, "{}{}{}", Self::TEXT_LINK.start, url, Self::TEXT_LINK.middle).unwrap()
            }
            Tag::TextLinkEnd(_) => buf.push_str(Self::TEXT_LINK.end),
            Tag::TextMentionStart(id) => {
                write!(buf, "{}{}{}", Self::TEXT_MENTION.start, id, Self::TEXT_MENTION.middle)
                    .unwrap()
            }
            Tag::TextMentionEnd(_) => buf.push_str(Self::TEXT_MENTION.end),
            Tag::CustomEmojiStart(custom_emoji_id) => write!(
                buf,
                "{}{}{}",
                Self::CUSTOM_EMOJI.start,
                custom_emoji_id,
                Self::CUSTOM_EMOJI.middle
            )
            .unwrap(),
            Tag::CustomEmojiEnd(_) => buf.push_str(Self::CUSTOM_EMOJI.end),
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
