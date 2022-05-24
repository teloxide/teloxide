use std::{cmp, ops::Range};

use serde::{Deserialize, Serialize};

use crate::types::{User, UserId};

/// This object represents one special entity in a text message.
///
/// For example, hashtags, usernames, URLs, etc.
///
/// [The official docs](https://core.telegram.org/bots/api#messageentity).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub kind: MessageEntityKind,

    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: usize,

    /// Length of the entity in UTF-16 code units.
    pub length: usize,
}

/// A "parsed" [`MessageEntity`].
///
/// [`MessageEntity`] has offsets in UTF-**16** code units, but in Rust we
/// mostly work with UTF-**8**. In order to use an entity we need to convert
/// UTF-16 offsets to UTF-8 ones. This type represents a message entity with
/// converted offsets and a reference to the text.
pub struct MessageEntityRef<'a> {
    message: &'a str,
    range: Range<usize>,
    kind: &'a MessageEntityKind,
}

impl MessageEntity {
    pub const fn new(kind: MessageEntityKind, offset: usize, length: usize) -> Self {
        Self {
            kind,
            offset,
            length,
        }
    }

    /// Create a message entity representing a bold text.
    pub const fn bold(offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::Bold,
            offset,
            length,
        }
    }

    /// Create a message entity representing an italic text.
    pub const fn italic(offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::Italic,
            offset,
            length,
        }
    }

    /// Create a message entity representing an underline text.
    pub const fn underline(offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::Underline,
            offset,
            length,
        }
    }

    /// Create a message entity representing a strikethrough text.
    pub const fn strikethrough(offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::Strikethrough,
            offset,
            length,
        }
    }

    /// Create a message entity representing a spoiler text.
    pub const fn spoiler(offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::Spoiler,
            offset,
            length,
        }
    }

    /// Create a message entity representing a monowidth text.
    pub const fn code(offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::Code,
            offset,
            length,
        }
    }

    /// Create a message entity representing a monowidth block.
    pub const fn pre(language: Option<String>, offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::Pre { language },
            offset,
            length,
        }
    }

    /// Create a message entity representing a clickable text URL.
    pub const fn text_link(url: reqwest::Url, offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::TextLink { url },
            offset,
            length,
        }
    }

    /// Create a message entity representing a text mention.
    ///
    /// # Note
    ///
    /// If you don't have a complete [`User`] value, please use
    /// [`MessageEntity::text_mention_id`] instead.
    pub fn text_mention(user: User, offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::TextMention { user },
            offset,
            length,
        }
    }

    /// Create a message entity representing a text link in the form of
    /// `tg://user/?id=...` that mentions user with `user_id`.
    pub fn text_mention_id(user_id: UserId, offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::TextLink { url: user_id.url() },
            offset,
            length,
        }
    }

    pub fn kind(mut self, val: MessageEntityKind) -> Self {
        self.kind = val;
        self
    }

    pub const fn offset(mut self, val: usize) -> Self {
        self.offset = val;
        self
    }

    pub const fn length(mut self, val: usize) -> Self {
        self.length = val;
        self
    }
}

impl<'a> MessageEntityRef<'a> {
    pub fn kind(&self) -> &'a MessageEntityKind {
        self.kind
    }

    pub fn text(&self) -> &'a str {
        &self.message[self.range.clone()]
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn start(&self) -> usize {
        self.range.start
    }

    pub fn end(&self) -> usize {
        self.range.end
    }

    pub fn len(&self) -> usize {
        self.range.len()
    }

    pub fn message_text(&self) -> &'a str {
        self.message
    }

    pub fn parse(text: &'a str, entities: &'a [MessageEntity]) -> Vec<Self> {
        // This creates entities with **wrong** offsets (UTF-16) that we later patch.
        let mut entities: Vec<_> = entities
            .iter()
            .map(|e| Self {
                message: text,
                range: e.offset..e.offset + e.length,
                kind: &e.kind,
            })
            .collect();

        // Convert offsets

        // References to all offsets that need patching
        let mut offsets: Vec<&mut usize> = entities
            .iter_mut()
            .flat_map(
                |Self {
                     range: Range { start, end },
                     ..
                 }| [start, end],
            )
            .collect();

        // Sort in decreasing order, so the smallest elements are at the end and can be
        // removed more easily
        offsets.sort_unstable_by_key(|&&mut offset| cmp::Reverse(offset));

        let _ = text
            .chars()
            .chain(['\0']) // this is needed to process offset pointing at the end of the string
            .try_fold((0, 0), |(len_utf8, len_utf16), c| {
                // Stop if there are no more offsets to patch
                if offsets.is_empty() {
                    return None;
                }

                // Patch all offsets that can be patched
                while offsets
                    .last()
                    .map(|&&mut offset| offset <= len_utf16)
                    .unwrap_or(false)
                {
                    let offset = offsets.pop().unwrap();
                    assert_eq!(*offset, len_utf16, "Invalid utf-16 offset");

                    // Patch the offset to be UTF-8
                    *offset = len_utf8;
                }

                // Update "running" length
                Some((len_utf8 + c.len_utf8(), len_utf16 + c.len_utf16()))
            });

        entities
    }
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Code,
    Pre { language: Option<String> },
    TextLink { url: reqwest::Url },
    TextMention { user: User },
    Underline,
    Strikethrough,
    Spoiler,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_kind() {
        use serde_json::from_str;

        assert_eq!(
            MessageEntity {
                kind: MessageEntityKind::TextLink {
                    url: reqwest::Url::parse("https://example.com").unwrap(),
                },
                offset: 1,
                length: 2,
            },
            from_str::<MessageEntity>(
                r#"{"type":"text_link","url":"https://example.com","offset":1,"length":2}"#
            )
            .unwrap()
        );
    }

    #[test]
    fn pre() {
        use serde_json::from_str;

        assert_eq!(
            MessageEntity {
                kind: MessageEntityKind::Pre {
                    language: Some("rust".to_string())
                },
                offset: 1,
                length: 2,
            },
            from_str::<MessageEntity>(r#"{"type":"pre","offset":1,"length":2,"language":"rust"}"#)
                .unwrap()
        );
    }

    // https://github.com/teloxide/teloxide-core/pull/145
    #[test]
    fn pre_with_none_language() {
        use serde_json::to_string;

        assert_eq!(
            to_string(&MessageEntity {
                kind: MessageEntityKind::Pre { language: None },
                offset: 1,
                length: 2,
            })
            .unwrap()
            .find("language"),
            None
        );
    }
}
