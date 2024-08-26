use std::cmp::Ordering;

#[derive(Clone)]
pub struct Tag<'a> {
    pub place: Place,
    pub kind: Kind<'a>,
    pub offset: usize,
    pub index: usize,
}

impl<'a> Tag<'a> {
    pub const fn start(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self { place: Place::Start, kind, offset, index }
    }

    pub const fn end(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self { place: Place::End, kind, offset, index }
    }
}

impl<'a> Eq for Tag<'a> {}

impl<'a> PartialEq for Tag<'a> {
    fn eq(&self, other: &Self) -> bool {
        // We don't check kind here
        self.place == other.place && self.offset == other.offset && self.index == other.index
    }
}

impl<'a> Ord for Tag<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset).then_with(|| self.place.cmp(&other.place)).then_with(|| {
            match other.place {
                Place::Start => self.index.cmp(&other.index),
                Place::End => other.index.cmp(&self.index),
            }
        })
    }
}

impl<'a> PartialOrd for Tag<'a> {
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
    Start,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Kind<'a> {
    Bold,
    Blockquote,
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
    pub const fn new(start: &'static str, end: &'static str) -> Self {
        Self { start, end }
    }

    /// Get tag size based on place
    pub const fn get_tag(&self, place: Place) -> &'static str {
        match place {
            Place::Start => self.start,
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
    pub const fn new(start: &'static str, middle: &'static str, end: &'static str) -> Self {
        Self { start, middle, end }
    }
}

pub struct TagWriter {
    pub bold: SimpleTag,
    pub blockquote: SimpleTag,
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

    /// Write the tag to buffer
    pub write_tag_fn: fn(&Tag, buf: &mut String),
    /// Write the char to buffer and escape characters if needed
    pub write_char_fn: fn(char, buf: &mut String),
}

impl TagWriter {
    /// Get the extra size needed for tags
    pub fn get_tags_sizes(&self, tags: &[Tag]) -> usize {
        tags.iter()
            .map(|tag| match tag.kind {
                Kind::Bold => self.bold.get_tag(tag.place).len(),
                Kind::Blockquote => self.blockquote.get_tag(tag.place).len(),
                Kind::Italic => self.italic.get_tag(tag.place).len(),
                Kind::Underline => self.underline.get_tag(tag.place).len(),
                Kind::Strikethrough => self.strikethrough.get_tag(tag.place).len(),
                Kind::Spoiler => self.spoiler.get_tag(tag.place).len(),
                Kind::Code => self.code.get_tag(tag.place).len(),
                Kind::Pre(lang) => match tag.place {
                    Place::Start => lang
                        .map_or(self.pre_no_lang.start.len(), |l| self.pre.start.len() + l.len()),
                    Place::End => lang.map_or(self.pre_no_lang.end.len(), |_| {
                        self.pre.middle.len() + self.pre.end.len()
                    }),
                },
                Kind::TextLink(url) => match tag.place {
                    Place::Start => self.text_link.start.len() + url.len(),
                    Place::End => self.text_link.middle.len() + self.text_link.end.len(),
                },
                Kind::TextMention(id) => match tag.place {
                    Place::Start => self.text_mention.start.len() + id.ilog10() as usize + 1,
                    Place::End => self.text_mention.middle.len() + self.text_mention.end.len(),
                },
                Kind::CustomEmoji(custom_emoji_id) => match tag.place {
                    Place::Start => self.custom_emoji.start.len() + custom_emoji_id.len(),
                    Place::End => self.custom_emoji.middle.len() + self.custom_emoji.end.len(),
                },
            })
            .sum()
    }
}
