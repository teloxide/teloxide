use std::cmp::Ordering;

#[derive(Clone)]
pub struct Tag<'a> {
    pub place: Place,
    pub kind: Kind<'a>,
    pub offset: usize,
    pub index: usize,
}

impl<'a> Tag<'a> {
    #[inline(always)]
    pub const fn start(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self { place: Place::Start, kind, offset, index }
    }

    #[inline(always)]
    pub const fn mid_new_line(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self { place: Place::MidNewLine, kind, offset, index }
    }

    #[inline(always)]
    pub const fn end(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self { place: Place::End, kind, offset, index }
    }
}

impl Eq for Tag<'_> {}

impl PartialEq for Tag<'_> {
    fn eq(&self, other: &Self) -> bool {
        // We don't check kind here
        self.place == other.place && self.offset == other.offset && self.index == other.index
    }
}

impl Ord for Tag<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset).then_with(|| self.place.cmp(&other.place)).then_with(|| {
            match other.place {
                Place::Start => self.index.cmp(&other.index),
                Place::MidNewLine => self.index.cmp(&other.index),
                Place::End => other.index.cmp(&self.index),
            }
        })
    }
}

impl PartialOrd for Tag<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Place {
    // HACK: `End` needs to be first because of the `Ord` Implementation.
    // the reason is when comparing tags we want the `End` to be first if the offset
    // is the same.
    End,
    MidNewLine,
    Start,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Kind<'a> {
    Bold,
    Blockquote,
    ExpandableBlockquote,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre(Option<&'a str>),
    TextLink(&'a str),
    TextMention(u64),
    CustomEmoji(&'a str),
}

pub struct SimpleTag {
    pub start: &'static str,
    pub end: &'static str,
}

impl SimpleTag {
    #[inline]
    pub const fn new(start: &'static str, end: &'static str) -> Self {
        Self { start, end }
    }

    pub const fn get_tag(&self, place: Place) -> &'static str {
        match place {
            Place::Start => self.start,
            Place::MidNewLine => unreachable!(), // SimpleTag can't be in MidNewLine
            Place::End => self.end,
        }
    }
}

pub struct ComplexTag {
    pub start: &'static str,
    pub middle: &'static str,
    pub end: &'static str,
}

impl ComplexTag {
    #[inline]
    pub const fn new(start: &'static str, middle: &'static str, end: &'static str) -> Self {
        Self { start, middle, end }
    }
}

pub struct NewLineRepeatedTag {
    pub start: &'static str,
    pub repeat: &'static str,
    pub end: &'static str,
}

impl NewLineRepeatedTag {
    #[inline]
    pub const fn new(start: &'static str, repeat: &'static str, end: &'static str) -> Self {
        Self { start, repeat, end }
    }
}

pub struct TagWriter {
    pub bold: SimpleTag,
    pub blockquote: NewLineRepeatedTag,
    pub expandable_blockquote: NewLineRepeatedTag,
    pub italic: SimpleTag,
    pub underline: SimpleTag,
    pub strikethrough: SimpleTag,
    pub spoiler: SimpleTag,
    pub code: SimpleTag,
    pub pre_no_lang: SimpleTag,
    pub pre: ComplexTag,
    pub text_link: ComplexTag,
    pub text_mention: ComplexTag,
    pub custom_emoji: ComplexTag,
    pub write_tag_fn: fn(&Tag, buf: &mut String),
    pub write_char_fn: fn(char, buf: &mut String),
}

impl TagWriter {
    pub fn get_extra_size_for_tags(&self, tags: &[Tag]) -> usize {
        tags.iter()
            .map(|tag| match tag.kind {
                Kind::Bold => self.bold.get_tag(tag.place).len(),
                Kind::Blockquote => match tag.place {
                    Place::Start => self.blockquote.start.len(),
                    Place::MidNewLine => self.blockquote.repeat.len(),
                    Place::End => self.blockquote.end.len(),
                },
                Kind::ExpandableBlockquote => match tag.place {
                    Place::Start => self.expandable_blockquote.start.len(),
                    Place::MidNewLine => self.expandable_blockquote.repeat.len(),
                    Place::End => self.expandable_blockquote.end.len(),
                },
                Kind::Italic => self.italic.get_tag(tag.place).len(),
                Kind::Underline => self.underline.get_tag(tag.place).len(),
                Kind::Strikethrough => self.strikethrough.get_tag(tag.place).len(),
                Kind::Spoiler => self.spoiler.get_tag(tag.place).len(),
                Kind::Code => self.code.get_tag(tag.place).len(),
                Kind::Pre(lang) => match tag.place {
                    Place::Start => lang
                        .map_or(self.pre_no_lang.start.len(), |l| self.pre.start.len() + l.len()),
                    Place::MidNewLine => unreachable!(),
                    Place::End => lang.map_or(self.pre_no_lang.end.len(), |_| {
                        self.pre.middle.len() + self.pre.end.len()
                    }),
                },
                Kind::TextLink(url) => match tag.place {
                    Place::Start => self.text_link.start.len() + url.len(),
                    Place::MidNewLine => unreachable!(),
                    Place::End => self.text_link.middle.len() + self.text_link.end.len(),
                },
                Kind::TextMention(id) => match tag.place {
                    Place::Start => self.text_mention.start.len() + id.ilog10() as usize + 1,
                    Place::MidNewLine => unreachable!(),
                    Place::End => self.text_mention.middle.len() + self.text_mention.end.len(),
                },
                Kind::CustomEmoji(custom_emoji_id) => match tag.place {
                    Place::Start => self.custom_emoji.start.len() + custom_emoji_id.len(),
                    Place::MidNewLine => unreachable!(),
                    Place::End => self.custom_emoji.middle.len() + self.custom_emoji.end.len(),
                },
            })
            .sum()
    }
}
