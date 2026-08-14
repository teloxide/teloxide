use serde::{de::Error as _, Deserialize, Deserializer, Serialize};

use crate::types::PhotoSize;

/// Rich, block-structured message content.
///
/// Introduced together with Bot API 10.1's "Rich Messages" feature. A rich
/// message is made up of [`RichBlock`]s (paragraphs, headings, lists,
/// tables, media, ...), which in turn contain [`RichText`] (plain text mixed
/// with bold/italic/links/... formatting).
///
/// Block and text node kinds that teloxide doesn't (yet) know about are
/// preserved as [`RichBlock::Other`]/[`RichText::Other`] instead of failing
/// deserialization of the whole [`Message`], so bots keep working even as
/// Telegram adds new rich content kinds.
///
/// [The official docs](https://core.telegram.org/bots/api#richmessage).
/// [`Message`]: crate::types::Message
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichMessage {
    /// The blocks that make up the rich message.
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
}

impl RichMessage {
    pub fn new<B>(blocks: B) -> Self
    where
        B: IntoIterator<Item = RichBlock>,
    {
        Self { blocks: blocks.into_iter().collect() }
    }

    /// Renders the rich message as full HTML markup: inline formatting uses
    /// the same tags as Telegram's [HTML formatted] messages (`<b>`, `<i>`,
    /// `<a>`, ...), while structural blocks use ordinary HTML elements
    /// (`<h1>`-`<h6>`, `<ul>`/`<ol>`/`<li>`, `<table>`, `<hr>`,
    /// `<details>`, ...).
    ///
    /// Note: because of the structural tags, this output is meant for
    /// display (e.g. a web page or log viewer), not for sending back to
    /// Telegram via `parse_mode: Html` — Telegram's HTML parser only
    /// accepts a small whitelist of inline tags and will reject `<table>`,
    /// `<ul>`, `<hr>`, etc. Use [`plain_text`] or [`to_markdown`] for
    /// Telegram-safe rendering instead.
    ///
    /// [HTML formatted]: https://core.telegram.org/bots/api#html-style
    /// [`plain_text`]: RichMessage::plain_text
    /// [`to_markdown`]: RichMessage::to_markdown
    pub fn to_html(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.write_html(&mut out);
        }
        out.trim_end_matches('\n').to_owned()
    }

    /// Renders the rich message as Telegram [MarkdownV2 formatted] text.
    ///
    /// [MarkdownV2 formatted]: https://core.telegram.org/bots/api#markdownv2-style
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.write_markdown(&mut out);
        }
        out.trim_end_matches('\n').to_owned()
    }

    /// Renders the rich message as plain text, discarding all formatting.
    pub fn plain_text(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.write_plain(&mut out);
        }
        out.trim_end_matches('\n').to_owned()
    }
}

/// A single structural block of a [`RichMessage`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "serde_json::Value")]
pub enum RichBlock {
    Paragraph(RichBlockParagraph),
    Heading(RichBlockHeading),
    Blockquote(RichBlockBlockquote),
    Pullquote(RichBlockPullquote),
    Pre(RichBlockPre),
    Footer(RichBlockFooter),
    Divider,
    List(RichBlockList),
    Details(RichBlockDetails),
    Table(RichBlockTable),
    Photo(RichBlockPhoto),
    /// A block `type` not (yet) understood by teloxide (e.g. `audio`,
    /// `video`, `map`, `collage`, `slideshow`, `thinking`, ...). The raw
    /// JSON fields (other than `type`) are preserved verbatim.
    Other {
        kind: String,
        raw: serde_json::Value,
    },
}

impl<'de> Deserialize<'de> for RichBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let kind = value.get("type").and_then(|t| t.as_str()).unwrap_or_default().to_owned();

        let parsed = match kind.as_str() {
            "paragraph" => serde_json::from_value(value.clone()).ok().map(RichBlock::Paragraph),
            "heading" => serde_json::from_value(value.clone()).ok().map(RichBlock::Heading),
            "blockquote" => serde_json::from_value(value.clone()).ok().map(RichBlock::Blockquote),
            "pullquote" => serde_json::from_value(value.clone()).ok().map(RichBlock::Pullquote),
            "pre" => serde_json::from_value(value.clone()).ok().map(RichBlock::Pre),
            "footer" => serde_json::from_value(value.clone()).ok().map(RichBlock::Footer),
            "divider" => Some(RichBlock::Divider),
            "list" => serde_json::from_value(value.clone()).ok().map(RichBlock::List),
            "details" => serde_json::from_value(value.clone()).ok().map(RichBlock::Details),
            "table" => serde_json::from_value(value.clone()).ok().map(RichBlock::Table),
            "photo" => serde_json::from_value(value.clone()).ok().map(RichBlock::Photo),
            _ => None,
        };

        if let Some(block) = parsed {
            return Ok(block);
        }

        let mut raw = value;
        if let Some(obj) = raw.as_object_mut() {
            obj.remove("type");
        }
        Ok(RichBlock::Other { kind, raw })
    }
}

impl From<RichBlock> for serde_json::Value {
    fn from(block: RichBlock) -> Self {
        fn tagged<T: Serialize>(kind: &str, payload: &T) -> serde_json::Value {
            let mut value = serde_json::to_value(payload).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = value.as_object_mut() {
                obj.insert("type".to_owned(), serde_json::Value::String(kind.to_owned()));
            }
            value
        }

        match block {
            RichBlock::Paragraph(b) => tagged("paragraph", &b),
            RichBlock::Heading(b) => tagged("heading", &b),
            RichBlock::Blockquote(b) => tagged("blockquote", &b),
            RichBlock::Pullquote(b) => tagged("pullquote", &b),
            RichBlock::Pre(b) => tagged("pre", &b),
            RichBlock::Footer(b) => tagged("footer", &b),
            RichBlock::Divider => {
                serde_json::json!({ "type": "divider" })
            }
            RichBlock::List(b) => tagged("list", &b),
            RichBlock::Details(b) => tagged("details", &b),
            RichBlock::Table(b) => tagged("table", &b),
            RichBlock::Photo(b) => tagged("photo", &b),
            RichBlock::Other { kind, raw } => {
                let mut raw = raw;
                if let Some(obj) = raw.as_object_mut() {
                    obj.insert("type".to_owned(), serde_json::Value::String(kind));
                }
                raw
            }
        }
    }
}

// `serde_json::Value` (used by the `Other` fallback variant) doesn't
// implement `Hash`, so this hashes each variant's discriminant together with
// its JSON representation instead of deriving.
impl std::hash::Hash for RichBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        serde_json::Value::from(self.clone()).to_string().hash(state);
    }
}

impl RichBlock {
    pub fn paragraph<T: Into<RichText>>(text: T) -> Self {
        Self::Paragraph(RichBlockParagraph { text: text.into() })
    }

    pub fn heading<T: Into<RichText>>(text: T, size: u8) -> Self {
        Self::Heading(RichBlockHeading { text: text.into(), size })
    }

    pub fn divider() -> Self {
        Self::Divider
    }

    fn write_html(&self, out: &mut String) {
        match self {
            RichBlock::Paragraph(b) => {
                out.push_str("<p>");
                b.text.write_html(out);
                out.push_str("</p>\n");
            }
            RichBlock::Heading(b) => {
                let level = b.size.clamp(1, 6);
                out.push_str(&format!("<h{level}>"));
                b.text.write_html(out);
                out.push_str(&format!("</h{level}>\n"));
            }
            RichBlock::Blockquote(b) => {
                out.push_str("<blockquote>\n");
                for inner in &b.blocks {
                    inner.write_html(out);
                }
                if let Some(credit) = &b.credit {
                    out.push_str("<cite>");
                    out.push_str(&escape_html(credit));
                    out.push_str("</cite>\n");
                }
                out.push_str("</blockquote>\n");
            }
            RichBlock::Pullquote(b) => {
                out.push_str("<blockquote><p>");
                b.text.write_html(out);
                out.push_str("</p>");
                if let Some(credit) = &b.credit {
                    out.push_str("<cite>");
                    out.push_str(&escape_html(credit));
                    out.push_str("</cite>");
                }
                out.push_str("</blockquote>\n");
            }
            RichBlock::Pre(b) => {
                out.push_str("<pre><code>");
                b.text.write_html(out);
                out.push_str("</code></pre>\n");
            }
            RichBlock::Footer(b) => {
                out.push_str("<footer><small>");
                b.text.write_html(out);
                out.push_str("</small></footer>\n");
            }
            RichBlock::Divider => out.push_str("<hr>\n"),
            RichBlock::List(b) => {
                let ordered = b.items.first().is_some_and(|item| item.kind.is_some());
                let tag = if ordered { "ol" } else { "ul" };
                out.push_str(&format!("<{tag}>\n"));
                for item in &b.items {
                    out.push_str("<li>");
                    if item.has_checkbox {
                        out.push_str("<input type=\"checkbox\" disabled");
                        if item.is_checked {
                            out.push_str(" checked");
                        }
                        out.push('>');
                    }
                    for inner in &item.blocks {
                        inner.write_html(out);
                    }
                    out.push_str("</li>\n");
                }
                out.push_str(&format!("</{tag}>\n"));
            }
            RichBlock::Details(b) => {
                out.push_str("<details><summary>");
                out.push_str(&escape_html(&b.summary));
                out.push_str("</summary>\n");
                for inner in &b.blocks {
                    inner.write_html(out);
                }
                out.push_str("</details>\n");
            }
            RichBlock::Table(b) => {
                out.push_str("<table>\n");
                if let Some(caption) = &b.caption {
                    out.push_str("<caption>");
                    out.push_str(&escape_html(caption));
                    out.push_str("</caption>\n");
                }
                for row in &b.cells {
                    out.push_str("<tr>");
                    for cell in row {
                        let tag = if cell.is_header { "th" } else { "td" };
                        out.push_str(&format!("<{tag}>"));
                        out.push_str(&escape_html(&cell.text));
                        out.push_str(&format!("</{tag}>"));
                    }
                    out.push_str("</tr>\n");
                }
                out.push_str("</table>\n");
            }
            RichBlock::Photo(b) => {
                if let Some(caption) = &b.caption {
                    out.push_str("<figure><figcaption>");
                    caption.text.write_html(out);
                    out.push_str("</figcaption></figure>\n");
                }
            }
            RichBlock::Other { .. } => {}
        }
    }

    fn write_markdown(&self, out: &mut String) {
        match self {
            RichBlock::Paragraph(b) => {
                b.text.write_markdown(out);
                out.push_str("\n\n");
            }
            RichBlock::Heading(b) => {
                out.push_str(&"#".repeat(b.size.clamp(1, 6) as usize));
                out.push(' ');
                b.text.write_markdown(out);
                out.push_str("\n\n");
            }
            RichBlock::Blockquote(b) => {
                for inner in &b.blocks {
                    let mut buf = String::new();
                    inner.write_markdown(&mut buf);
                    for line in buf.trim_end().lines() {
                        out.push_str("> ");
                        out.push_str(line);
                        out.push('\n');
                    }
                }
                out.push('\n');
            }
            RichBlock::Pullquote(b) => {
                let mut buf = String::new();
                b.text.write_markdown(&mut buf);
                for line in buf.lines() {
                    out.push_str("> ");
                    out.push_str(line);
                    out.push('\n');
                }
                out.push('\n');
            }
            RichBlock::Pre(b) => {
                out.push_str("```");
                out.push_str(b.language.as_deref().unwrap_or(""));
                out.push('\n');
                b.text.write_plain(out);
                out.push_str("\n```\n\n");
            }
            RichBlock::Footer(b) => {
                out.push('_');
                b.text.write_markdown(out);
                out.push_str("_\n\n");
            }
            RichBlock::Divider => out.push_str("\n---\n\n"),
            RichBlock::List(b) => {
                for item in &b.items {
                    out.push_str(&item.label);
                    out.push(' ');
                    for (i, inner) in item.blocks.iter().enumerate() {
                        if i == 0 {
                            let mut buf = String::new();
                            inner.write_markdown(&mut buf);
                            out.push_str(buf.trim_end());
                            out.push('\n');
                        } else {
                            inner.write_markdown(out);
                        }
                    }
                }
                out.push('\n');
            }
            RichBlock::Details(b) => {
                out.push_str("**");
                out.push_str(&b.summary);
                out.push_str("**\n");
                for inner in &b.blocks {
                    inner.write_markdown(out);
                }
            }
            RichBlock::Table(b) => {
                for (i, row) in b.cells.iter().enumerate() {
                    let rendered: Vec<_> = row.iter().map(|c| c.text.clone()).collect();
                    out.push_str("| ");
                    out.push_str(&rendered.join(" | "));
                    out.push_str(" |\n");
                    if i == 0 {
                        out.push_str("| ");
                        out.push_str(&vec!["---"; row.len()].join(" | "));
                        out.push_str(" |\n");
                    }
                }
                out.push('\n');
            }
            RichBlock::Photo(b) => {
                if let Some(caption) = &b.caption {
                    caption.text.write_markdown(out);
                    out.push_str("\n\n");
                }
            }
            RichBlock::Other { .. } => {}
        }
    }

    fn write_plain(&self, out: &mut String) {
        match self {
            RichBlock::Paragraph(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Footer(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Heading(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Blockquote(b) => {
                for inner in &b.blocks {
                    inner.write_plain(out);
                }
            }
            RichBlock::Pullquote(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Pre(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Divider => out.push('\n'),
            RichBlock::List(b) => {
                for item in &b.items {
                    for inner in &item.blocks {
                        inner.write_plain(out);
                    }
                }
            }
            RichBlock::Details(b) => {
                out.push_str(&b.summary);
                out.push('\n');
                for inner in &b.blocks {
                    inner.write_plain(out);
                }
            }
            RichBlock::Table(b) => {
                for row in &b.cells {
                    let rendered: Vec<_> = row.iter().map(|c| c.text.clone()).collect();
                    out.push_str(&rendered.join(" "));
                    out.push('\n');
                }
            }
            RichBlock::Photo(b) => {
                if let Some(caption) = &b.caption {
                    caption.text.write_plain(out);
                    out.push_str("\n\n");
                }
            }
            RichBlock::Other { .. } => {}
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockParagraph {
    pub text: RichText,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockHeading {
    pub text: RichText,
    pub size: u8,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockBlockquote {
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
    pub credit: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockPullquote {
    pub text: RichText,
    pub credit: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockPre {
    pub text: RichText,
    pub language: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockFooter {
    pub text: RichText,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockList {
    #[serde(default)]
    pub items: Vec<RichListItem>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichListItem {
    /// The rendered bullet/number of the item (e.g. `"1."`, `"•"`).
    pub label: String,

    #[serde(default)]
    pub blocks: Vec<RichBlock>,

    /// The list kind this item belongs to (e.g. `"1"` for numeric lists).
    #[serde(rename = "type")]
    pub kind: Option<String>,

    /// The item's numeric value, for ordered lists.
    pub value: Option<i64>,

    /// `true` if the item is rendered with a checkbox.
    #[serde(default)]
    pub has_checkbox: bool,

    /// `true` if the item's checkbox is checked.
    #[serde(default)]
    pub is_checked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockDetails {
    pub summary: String,
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockTable {
    #[serde(default)]
    pub cells: Vec<Vec<RichTableCell>>,
    pub caption: Option<String>,
    #[serde(default)]
    pub is_bordered: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTableCell {
    pub text: String,
    #[serde(default)]
    pub is_header: bool,
    pub align: Option<String>,
    pub valign: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockPhoto {
    pub photo: Vec<PhotoSize>,
    pub caption: Option<RichCaption>,
}

/// Caption text attached to a media [`RichBlock`] (e.g. [`RichBlockPhoto`]).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichCaption {
    pub text: RichText,
}

/// Formatted inline text used throughout [`RichBlock`]s.
///
/// A node is either a plain string, a sequence of nodes, or a tagged
/// formatting wrapper (bold, italic, link, ...) around another node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "serde_json::Value")]
pub enum RichText {
    Plain(String),
    Array(Vec<RichText>),
    Bold(RichTextSimple),
    Italic(RichTextSimple),
    Underline(RichTextSimple),
    Strikethrough(RichTextSimple),
    Spoiler(RichTextSimple),
    Subscript(RichTextSimple),
    Superscript(RichTextSimple),
    Marked(RichTextSimple),
    Code(RichTextSimple),
    Url(RichTextUrl),
    MathematicalExpression(RichTextMathematicalExpression),
    CustomEmoji(RichTextCustomEmoji),
    /// A `type` not (yet) understood by teloxide (e.g. `mention`,
    /// `hashtag`, `email_address`, ...). The raw JSON fields (other than
    /// `type`) are preserved verbatim.
    Other {
        kind: String,
        raw: serde_json::Value,
    },
}

impl From<&str> for RichText {
    fn from(s: &str) -> Self {
        RichText::Plain(s.to_owned())
    }
}

impl From<String> for RichText {
    fn from(s: String) -> Self {
        RichText::Plain(s)
    }
}

impl<'de> Deserialize<'de> for RichText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        RichText::from_value(value).map_err(D::Error::custom)
    }
}

impl From<RichText> for serde_json::Value {
    fn from(text: RichText) -> Self {
        fn tagged<T: Serialize>(kind: &str, payload: &T) -> serde_json::Value {
            let mut value = serde_json::to_value(payload).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = value.as_object_mut() {
                obj.insert("type".to_owned(), serde_json::Value::String(kind.to_owned()));
            }
            value
        }

        match text {
            RichText::Plain(s) => serde_json::Value::String(s),
            RichText::Array(items) => {
                serde_json::Value::Array(items.into_iter().map(Into::into).collect())
            }
            RichText::Bold(t) => tagged("bold", &t),
            RichText::Italic(t) => tagged("italic", &t),
            RichText::Underline(t) => tagged("underline", &t),
            RichText::Strikethrough(t) => tagged("strikethrough", &t),
            RichText::Spoiler(t) => tagged("spoiler", &t),
            RichText::Subscript(t) => tagged("subscript", &t),
            RichText::Superscript(t) => tagged("superscript", &t),
            RichText::Marked(t) => tagged("marked", &t),
            RichText::Code(t) => tagged("code", &t),
            RichText::Url(t) => tagged("url", &t),
            RichText::MathematicalExpression(t) => tagged("mathematical_expression", &t),
            RichText::CustomEmoji(t) => tagged("custom_emoji", &t),
            RichText::Other { kind, raw } => {
                let mut raw = raw;
                if let Some(obj) = raw.as_object_mut() {
                    obj.insert("type".to_owned(), serde_json::Value::String(kind));
                }
                raw
            }
        }
    }
}

// See the `Hash` impl for `RichBlock` for why this isn't derived.
impl std::hash::Hash for RichText {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        serde_json::Value::from(self.clone()).to_string().hash(state);
    }
}

impl RichText {
    fn from_value(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        match value {
            serde_json::Value::String(s) => Ok(RichText::Plain(s)),
            serde_json::Value::Array(items) => items
                .into_iter()
                .map(RichText::from_value)
                .collect::<Result<_, _>>()
                .map(RichText::Array),
            serde_json::Value::Object(_) => {
                let kind =
                    value.get("type").and_then(|t| t.as_str()).unwrap_or_default().to_owned();

                macro_rules! simple {
                    ($Variant:ident) => {
                        serde_json::from_value(value.clone()).ok().map(RichText::$Variant)
                    };
                }

                let parsed = match kind.as_str() {
                    "bold" => simple!(Bold),
                    "italic" => simple!(Italic),
                    "underline" => simple!(Underline),
                    "strikethrough" => simple!(Strikethrough),
                    "spoiler" => simple!(Spoiler),
                    "subscript" => simple!(Subscript),
                    "superscript" => simple!(Superscript),
                    "marked" => simple!(Marked),
                    "code" => simple!(Code),
                    "url" => serde_json::from_value(value.clone()).ok().map(RichText::Url),
                    "mathematical_expression" => serde_json::from_value(value.clone())
                        .ok()
                        .map(RichText::MathematicalExpression),
                    "custom_emoji" => {
                        serde_json::from_value(value.clone()).ok().map(RichText::CustomEmoji)
                    }
                    _ => None,
                };

                if let Some(text) = parsed {
                    return Ok(text);
                }

                let mut raw = value;
                if let Some(obj) = raw.as_object_mut() {
                    obj.remove("type");
                }
                Ok(RichText::Other { kind, raw })
            }
            other => Ok(RichText::Plain(other.to_string())),
        }
    }

    fn write_html(&self, out: &mut String) {
        match self {
            RichText::Plain(s) => out.push_str(&escape_html(s)),
            RichText::Array(items) => items.iter().for_each(|i| i.write_html(out)),
            RichText::Bold(t) => wrap_html(out, "b", &t.text),
            RichText::Italic(t) => wrap_html(out, "i", &t.text),
            RichText::Underline(t) => wrap_html(out, "u", &t.text),
            RichText::Strikethrough(t) => wrap_html(out, "s", &t.text),
            RichText::Spoiler(t) => wrap_html(out, "tg-spoiler", &t.text),
            RichText::Code(t) => wrap_html(out, "code", &t.text),
            RichText::Subscript(t) | RichText::Superscript(t) | RichText::Marked(t) => {
                t.text.write_html(out)
            }
            RichText::Url(t) => {
                out.push_str("<a href=\"");
                out.push_str(&escape_html_attr(&t.url));
                out.push_str("\">");
                t.text.write_html(out);
                out.push_str("</a>");
            }
            RichText::MathematicalExpression(t) => out.push_str(&escape_html(&t.expression)),
            RichText::CustomEmoji(t) => out.push_str(&escape_html(&t.alternative_text)),
            RichText::Other { raw, .. } => out.push_str(&escape_html(&other_plain_text(raw))),
        }
    }

    fn write_markdown(&self, out: &mut String) {
        match self {
            RichText::Plain(s) => out.push_str(&escape_markdown(s)),
            RichText::Array(items) => items.iter().for_each(|i| i.write_markdown(out)),
            RichText::Bold(t) => wrap_markdown(out, "*", &t.text),
            RichText::Italic(t) => wrap_markdown(out, "_", &t.text),
            RichText::Underline(t) => wrap_markdown(out, "__", &t.text),
            RichText::Strikethrough(t) => wrap_markdown(out, "~", &t.text),
            RichText::Spoiler(t) => wrap_markdown(out, "||", &t.text),
            RichText::Code(t) => wrap_markdown(out, "`", &t.text),
            RichText::Subscript(t) | RichText::Superscript(t) | RichText::Marked(t) => {
                t.text.write_markdown(out)
            }
            RichText::Url(t) => {
                out.push('[');
                t.text.write_markdown(out);
                out.push_str("](");
                out.push_str(&t.url);
                out.push(')');
            }
            RichText::MathematicalExpression(t) => out.push_str(&escape_markdown(&t.expression)),
            RichText::CustomEmoji(t) => out.push_str(&escape_markdown(&t.alternative_text)),
            RichText::Other { raw, .. } => out.push_str(&escape_markdown(&other_plain_text(raw))),
        }
    }

    fn write_plain(&self, out: &mut String) {
        match self {
            RichText::Plain(s) => out.push_str(s),
            RichText::Array(items) => items.iter().for_each(|i| i.write_plain(out)),
            RichText::Bold(t)
            | RichText::Italic(t)
            | RichText::Underline(t)
            | RichText::Strikethrough(t)
            | RichText::Spoiler(t)
            | RichText::Subscript(t)
            | RichText::Superscript(t)
            | RichText::Marked(t)
            | RichText::Code(t) => t.text.write_plain(out),
            RichText::Url(t) => t.text.write_plain(out),
            RichText::MathematicalExpression(t) => out.push_str(&t.expression),
            RichText::CustomEmoji(t) => out.push_str(&t.alternative_text),
            RichText::Other { raw, .. } => out.push_str(&other_plain_text(raw)),
        }
    }
}

fn other_plain_text(raw: &serde_json::Value) -> String {
    match raw.get("text") {
        Some(text) => RichText::from_value(text.clone())
            .map(|t| {
                let mut s = String::new();
                t.write_plain(&mut s);
                s
            })
            .unwrap_or_default(),
        None => String::new(),
    }
}

fn wrap_html(out: &mut String, tag: &str, text: &RichText) {
    out.push('<');
    out.push_str(tag);
    out.push('>');
    text.write_html(out);
    out.push_str("</");
    out.push_str(tag);
    out.push('>');
}

fn wrap_markdown(out: &mut String, marker: &str, text: &RichText) {
    out.push_str(marker);
    text.write_markdown(out);
    out.push_str(marker);
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn escape_html_attr(s: &str) -> String {
    escape_html(s).replace('"', "&quot;")
}

fn escape_markdown(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if "_*[]()~`>#+-=|{}.!\\".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// A formatting wrapper that carries no extra data besides the wrapped text
/// (bold, italic, underline, strikethrough, spoiler, subscript,
/// superscript, marked, code).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextSimple {
    pub text: Box<RichText>,
}

impl RichTextSimple {
    pub fn new<T: Into<RichText>>(text: T) -> Self {
        Self { text: Box::new(text.into()) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextUrl {
    pub text: Box<RichText>,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextMathematicalExpression {
    pub expression: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextCustomEmoji {
    pub custom_emoji_id: String,
    pub alternative_text: String,
}

/// Describes a rich message to be sent by a bot.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
pub struct InputRichMessage {
    /// The blocks that make up the rich message.
    pub blocks: Vec<RichBlock>,
}

impl InputRichMessage {
    pub fn new<B>(blocks: B) -> Self
    where
        B: IntoIterator<Item = RichBlock>,
    {
        Self { blocks: blocks.into_iter().collect() }
    }
}

impl From<Vec<RichBlock>> for InputRichMessage {
    fn from(blocks: Vec<RichBlock>) -> Self {
        Self::new(blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_sample_rich_message() {
        let json = serde_json::json!({
            "blocks": [
                { "type": "paragraph", "text": "test (plain)" },
                { "type": "heading", "text": "test (h1)", "size": 1 },
                {
                    "type": "blockquote",
                    "blocks": [{ "type": "paragraph", "text": "test (quote)" }],
                    "credit": "author"
                },
                { "type": "pullquote", "text": "test", "credit": "author" },
                { "type": "pre", "text": "test (code)", "language": "test" },
                { "type": "footer", "text": "test (small print)" },
                { "type": "divider" },
                { "type": "paragraph", "text": { "type": "bold", "text": "test (bold)" } },
                {
                    "type": "list",
                    "items": [{
                        "label": "1.",
                        "blocks": [{ "type": "paragraph", "text": "test (numeric)" }],
                        "type": "1",
                        "value": 1
                    }]
                },
                {
                    "type": "details",
                    "summary": "test (collapsible block)",
                    "blocks": [{ "type": "paragraph", "text": "test (collapsible block content)" }]
                },
                {
                    "type": "table",
                    "cells": [[{ "text": "1", "is_header": true }]],
                    "caption": "test (table",
                    "is_bordered": true
                },
                {
                    "type": "paragraph",
                    "text": [{ "type": "url", "text": "test (link)", "url": "https://example.com/" }, " "]
                },
                {
                    "type": "paragraph",
                    "text": { "type": "mathematical_expression", "expression": "test (expression)" }
                },
                {
                    "type": "paragraph",
                    "text": {
                        "type": "custom_emoji",
                        "custom_emoji_id": "5231290743716333071",
                        "alternative_text": "🙂"
                    }
                },
                // A block type teloxide doesn't know about yet must not break parsing.
                { "type": "thinking", "text": "..." }
            ]
        });

        let rich: RichMessage = serde_json::from_value(json).expect("should deserialize");
        assert_eq!(rich.blocks.len(), 15);
        assert!(
            matches!(rich.blocks.last(), Some(RichBlock::Other { kind, .. }) if kind == "thinking")
        );

        let html = rich.to_html();
        assert!(html.contains("<b>test (bold)</b>"));
        assert!(html.contains("<a href=\"https://example.com/\">test (link)</a>"));

        let plain = rich.plain_text();
        assert!(plain.contains("test (plain)"));
        assert!(plain.contains("test (expression)"));
    }
}
