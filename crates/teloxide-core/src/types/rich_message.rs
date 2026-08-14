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
/// deserialization of the whole `Message`, so bots keep working even as
/// Telegram adds new rich content kinds.
///
/// [The official docs](https://core.telegram.org/bots/api#richmessage).
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
                // `<aside>` (not `<blockquote>`) because a pull quote is a
                // separate, decorative excerpt, not the same thing as a
                // regular quote block.
                out.push_str("<aside><p>");
                b.text.write_html(out);
                out.push_str("</p>");
                if let Some(credit) = &b.credit {
                    out.push_str("<cite>");
                    out.push_str(&escape_html(credit));
                    out.push_str("</cite>");
                }
                out.push_str("</aside>\n");
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
                        cell.text.write_html(out);
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
                // `> — credit`: not standard Markdown, but a widely
                // understood attribution convention; `InputRichMessage::parse`
                // recognizes it too.
                if let Some(credit) = &b.credit {
                    out.push_str("> — ");
                    out.push_str(&escape_markdown(credit));
                    out.push('\n');
                }
                out.push('\n');
            }
            RichBlock::Pullquote(b) => {
                // `>>` (not `>`) to distinguish a pull quote from a regular
                // blockquote — Markdown has no native syntax for this, so
                // `InputRichMessage::parse` relies on this convention too.
                let mut buf = String::new();
                b.text.write_markdown(&mut buf);
                for line in buf.lines() {
                    out.push_str(">> ");
                    out.push_str(line);
                    out.push('\n');
                }
                if let Some(credit) = &b.credit {
                    out.push_str(">> — ");
                    out.push_str(&escape_markdown(credit));
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
                    let rendered: Vec<_> = row
                        .iter()
                        .map(|c| {
                            let mut s = String::new();
                            c.text.write_markdown(&mut s);
                            s
                        })
                        .collect();
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
                    let rendered: Vec<_> = row
                        .iter()
                        .map(|c| {
                            let mut s = String::new();
                            c.text.write_plain(&mut s);
                            s
                        })
                        .collect();
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
    pub text: RichText,
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

impl Default for RichText {
    fn default() -> Self {
        RichText::Plain(String::new())
    }
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
            RichText::MathematicalExpression(t) => {
                out.push_str("<tg-math>");
                out.push_str(&escape_html(&t.expression));
                out.push_str("</tg-math>");
            }
            RichText::CustomEmoji(t) => {
                out.push_str("<tg-emoji emoji-id=\"");
                out.push_str(&escape_html_attr(&t.custom_emoji_id));
                out.push_str("\">");
                out.push_str(&escape_html(&t.alternative_text));
                out.push_str("</tg-emoji>");
            }
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
            RichText::MathematicalExpression(t) => {
                // `$expr$`: the universal inline-LaTeX convention. The
                // expression itself is left unescaped — it's raw LaTeX, not
                // Markdown, and escaping it would mangle things like `\frac{}`.
                out.push('$');
                out.push_str(&t.expression);
                out.push('$');
            }
            RichText::CustomEmoji(t) => {
                // Telegram's real MarkdownV2 syntax for a custom (Premium)
                // emoji: `![fallback](tg://emoji?id=...)`.
                out.push_str("![");
                out.push_str(&t.alternative_text);
                out.push_str("](tg://emoji?id=");
                out.push_str(&t.custom_emoji_id);
                out.push(')');
            }
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

    /// Parses `text` as either Markdown or HTML (selected by `mode`) into a
    /// rich message, ready to be sent with
    /// [`Requester::send_rich_message`](crate::requests::Requester::send_rich_message).
    ///
    /// The parsers are best-effort: unrecognized or malformed markup is
    /// never an error, it's simply carried through as literal text, so this
    /// never fails.
    ///
    /// See [`RichParseMode`] for exactly which syntax/tags are recognized.
    #[must_use]
    pub fn parse(text: &str, mode: RichParseMode) -> Self {
        let blocks = match mode {
            RichParseMode::Markdown => parse::markdown(text),
            RichParseMode::Html => parse::html(text),
        };
        Self { blocks }
    }
}

impl From<Vec<RichBlock>> for InputRichMessage {
    fn from(blocks: Vec<RichBlock>) -> Self {
        Self::new(blocks)
    }
}

/// Selects which markup [`InputRichMessage::parse`] should read `text` as.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RichParseMode {
    /// Parse [Markdown]: `*bold*`, `_italic_`, `__underline__`,
    /// `~strikethrough~`, `||spoiler||`, `` `code` ``, `[text](url)`,
    /// `#`..`######` headings, `>` blockquotes, fenced code blocks
    /// (` ```lang `), `-`/`*`/`1.` lists, `---` dividers, and pipe tables
    /// (`| a | b |` with a `| --- | --- |` separator row). This is the same
    /// flavor produced by [`RichMessage::to_markdown`].
    ///
    /// [Markdown]: https://en.wikipedia.org/wiki/Markdown
    Markdown,

    /// Parse HTML: `<b>`/`<strong>`, `<i>`/`<em>`, `<u>`/`<ins>`,
    /// `<s>`/`<strike>`/`<del>`, `<tg-spoiler>`, `<code>`, `<a href>`,
    /// `<h1>`-`<h6>`, `<p>`, `<blockquote>`, `<pre>`, `<hr>`,
    /// `<ul>`/`<ol>`/`<li>` (including `<input type="checkbox">`),
    /// `<table>`/`<caption>`/`<tr>`/`<th>`/`<td>`, and
    /// `<details>`/`<summary>`. This is the same tag set produced by
    /// [`RichMessage::to_html`].
    Html,
}

/// Best-effort Markdown/HTML → [`RichBlock`] parsers backing
/// [`InputRichMessage::parse`].
///
/// Both parsers are lenient: unrecognized syntax is never an error, it just
/// falls through to literal text.
mod parse {
    use super::{
        RichBlock, RichBlockBlockquote, RichBlockDetails, RichBlockHeading, RichBlockList,
        RichBlockParagraph, RichBlockPre, RichBlockPullquote, RichBlockTable, RichListItem,
        RichTableCell, RichText, RichTextCustomEmoji, RichTextMathematicalExpression,
        RichTextSimple, RichTextUrl,
    };

    // ---------------------------------------------------------------------
    // Markdown
    // ---------------------------------------------------------------------

    pub(super) fn markdown(text: &str) -> Vec<RichBlock> {
        let lines: Vec<&str> = text.lines().collect();
        markdown_blocks(&lines)
    }

    fn markdown_blocks(lines: &[&str]) -> Vec<RichBlock> {
        let mut blocks = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            if line.trim().is_empty() {
                i += 1;
                continue;
            }

            // Fenced code block: ```lang ... ```
            if let Some(rest) = line.trim_start().strip_prefix("```") {
                let language = {
                    let l = rest.trim();
                    (!l.is_empty()).then(|| l.to_owned())
                };
                let mut code_lines = Vec::new();
                i += 1;
                while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                    code_lines.push(lines[i]);
                    i += 1;
                }
                if i < lines.len() {
                    i += 1; // closing fence
                }
                blocks.push(RichBlock::Pre(RichBlockPre {
                    text: RichText::Plain(code_lines.join("\n")),
                    language,
                }));
                continue;
            }

            // Heading: # .. ######
            if let Some((size, rest)) = parse_heading(line) {
                blocks
                    .push(RichBlock::Heading(RichBlockHeading { text: parse_inline(rest), size }));
                i += 1;
                continue;
            }

            // Table: a `|` row followed by a `| --- | --- |` separator row.
            if line.contains('|') && lines.get(i + 1).is_some_and(|l| is_table_separator(l)) {
                let mut rows = vec![split_table_row(line)];
                i += 2;
                while i < lines.len() && lines[i].contains('|') && !lines[i].trim().is_empty() {
                    rows.push(split_table_row(lines[i]));
                    i += 1;
                }
                let cells = rows
                    .into_iter()
                    .enumerate()
                    .map(|(row_idx, row)| {
                        row.into_iter()
                            .map(|text| RichTableCell {
                                text: parse_inline(&text),
                                is_header: row_idx == 0,
                                align: None,
                                valign: None,
                            })
                            .collect()
                    })
                    .collect();
                blocks.push(RichBlock::Table(RichBlockTable {
                    cells,
                    caption: None,
                    is_bordered: true,
                }));
                continue;
            }

            // Divider: ---, ***, or ___ (3+ of the same char).
            if is_divider(line.trim()) {
                blocks.push(RichBlock::Divider);
                i += 1;
                continue;
            }

            // Pull quote: consecutive `>> ...` lines (not standard Markdown;
            // matches what `RichText::write_markdown` emits for `Pullquote`,
            // since regular Markdown has no dedicated pull-quote syntax).
            if line.trim_start().starts_with(">>") {
                let mut quote_lines = Vec::new();
                while i < lines.len() && lines[i].trim_start().starts_with(">>") {
                    let l = lines[i].trim_start().strip_prefix(">>").unwrap_or("");
                    quote_lines.push(l.strip_prefix(' ').unwrap_or(l));
                    i += 1;
                }
                let credit = extract_credit(&mut quote_lines);
                blocks.push(RichBlock::Pullquote(RichBlockPullquote {
                    text: parse_inline(&quote_lines.join(" ")),
                    credit,
                }));
                continue;
            }

            // Blockquote: consecutive `> ...` lines.
            if line.trim_start().starts_with('>') {
                let mut quote_lines = Vec::new();
                while i < lines.len() && lines[i].trim_start().starts_with('>') {
                    let l = lines[i].trim_start().strip_prefix('>').unwrap_or("");
                    quote_lines.push(l.strip_prefix(' ').unwrap_or(l));
                    i += 1;
                }
                let credit = extract_credit(&mut quote_lines);
                blocks.push(RichBlock::Blockquote(RichBlockBlockquote {
                    blocks: markdown_blocks(&quote_lines),
                    credit,
                }));
                continue;
            }

            // List: consecutive `- `/`* `/`+ `/`1. ` lines.
            if parse_list_marker(line).is_some() {
                let mut items = Vec::new();
                while let Some((marker, rest)) = lines.get(i).and_then(|l| parse_list_marker(l)) {
                    let (label, kind, value) = match marker {
                        ListMarker::Bullet => ("•".to_owned(), None, None),
                        ListMarker::Number(n) => (format!("{n}."), Some("1".to_owned()), Some(n)),
                    };
                    items.push(RichListItem {
                        label,
                        blocks: vec![RichBlock::Paragraph(RichBlockParagraph {
                            text: parse_inline(rest),
                        })],
                        kind,
                        value,
                        has_checkbox: false,
                        is_checked: false,
                    });
                    i += 1;
                }
                blocks.push(RichBlock::List(RichBlockList { items }));
                continue;
            }

            // Paragraph: consecutive plain lines until a blank line or the next special
            // block.
            let mut para_lines = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty()
                    || l.trim_start().starts_with("```")
                    || parse_heading(l).is_some()
                    || is_divider(l.trim())
                    || l.trim_start().starts_with('>')
                    || parse_list_marker(l).is_some()
                {
                    break;
                }
                para_lines.push(l.trim());
                i += 1;
            }
            blocks.push(RichBlock::Paragraph(RichBlockParagraph {
                text: parse_inline(&para_lines.join(" ")),
            }));
        }

        blocks
    }

    /// Strips a trailing `— credit` attribution line (and any blank lines
    /// before it) off the end of `lines`, returning the credit if found.
    fn extract_credit(lines: &mut Vec<&str>) -> Option<String> {
        while matches!(lines.last(), Some(l) if l.trim().is_empty()) {
            lines.pop();
        }
        let credit = lines.last().and_then(|l| l.trim().strip_prefix('—'))?.trim().to_owned();
        lines.pop();
        Some(credit)
    }

    fn parse_heading(line: &str) -> Option<(u8, &str)> {
        let trimmed = line.trim_start();
        let hashes = trimmed.chars().take_while(|&c| c == '#').count();
        if hashes == 0 || hashes > 6 {
            return None;
        }
        let rest = &trimmed[hashes..];
        rest.starts_with(' ').then(|| (hashes as u8, rest.trim()))
    }

    fn is_divider(s: &str) -> bool {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        let Some(first) = s.chars().next() else { return false };
        s.len() >= 3 && matches!(first, '-' | '*' | '_') && s.chars().all(|c| c == first)
    }

    fn is_table_separator(line: &str) -> bool {
        let cells: Vec<&str> = line.split('|').map(str::trim).filter(|c| !c.is_empty()).collect();
        !cells.is_empty()
            && cells.iter().all(|c| {
                let c = c.trim_start_matches(':').trim_end_matches(':');
                !c.is_empty() && c.chars().all(|ch| ch == '-')
            })
    }

    fn split_table_row(line: &str) -> Vec<String> {
        let trimmed = line.trim().trim_start_matches('|').trim_end_matches('|');
        trimmed.split('|').map(|c| c.trim().to_owned()).collect()
    }

    enum ListMarker {
        Bullet,
        Number(i64),
    }

    fn parse_list_marker(line: &str) -> Option<(ListMarker, &str)> {
        let trimmed = line.trim_start();
        // `• ` is included because `RichText::write_markdown` (and Telegram
        // itself, for incoming messages) renders bullet items with it.
        for prefix in ["- ", "* ", "+ ", "• "] {
            if let Some(rest) = trimmed.strip_prefix(prefix) {
                return Some((ListMarker::Bullet, rest.trim()));
            }
        }
        let digits_end = trimmed.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
        if digits_end > 0 {
            let (num, rest) = trimmed.split_at(digits_end);
            if let Some(rest) = rest.strip_prefix(". ") {
                if let Ok(n) = num.parse::<i64>() {
                    return Some((ListMarker::Number(n), rest.trim()));
                }
            }
        }
        None
    }

    type Wrap = fn(RichText) -> RichText;

    // Matches the syntax `RichText::write_markdown` produces (Telegram's
    // MarkdownV2: single `*`/`_`/`~` markers, double `__`/`||`).
    const DOUBLE_DELIMS: &[(&str, Wrap)] = &[("__", underline), ("||", spoiler)];
    const SINGLE_DELIMS: &[(char, Wrap)] = &[('*', bold), ('~', strikethrough), ('_', italic)];

    fn bold(t: RichText) -> RichText {
        RichText::Bold(RichTextSimple::new(t))
    }
    fn italic(t: RichText) -> RichText {
        RichText::Italic(RichTextSimple::new(t))
    }
    fn underline(t: RichText) -> RichText {
        RichText::Underline(RichTextSimple::new(t))
    }
    fn strikethrough(t: RichText) -> RichText {
        RichText::Strikethrough(RichTextSimple::new(t))
    }
    fn spoiler(t: RichText) -> RichText {
        RichText::Spoiler(RichTextSimple::new(t))
    }

    fn collapse(nodes: Vec<RichText>) -> RichText {
        match nodes.len() {
            0 => RichText::Plain(String::new()),
            1 => nodes.into_iter().next().unwrap(),
            _ => RichText::Array(nodes),
        }
    }

    fn parse_inline(s: &str) -> RichText {
        let chars: Vec<char> = s.chars().collect();
        collapse(parse_inline_chars(&chars, 0, chars.len()))
    }

    fn parse_inline_chars(chars: &[char], start: usize, end: usize) -> Vec<RichText> {
        let mut nodes = Vec::new();
        let mut buf = String::new();
        let mut i = start;

        while i < end {
            match try_match_delim(chars, i, end) {
                Some((node, next)) => {
                    if !buf.is_empty() {
                        nodes.push(RichText::Plain(std::mem::take(&mut buf)));
                    }
                    nodes.push(node);
                    i = next;
                }
                None => {
                    buf.push(chars[i]);
                    i += 1;
                }
            }
        }
        if !buf.is_empty() {
            nodes.push(RichText::Plain(buf));
        }
        nodes
    }

    fn try_match_delim(chars: &[char], i: usize, end: usize) -> Option<(RichText, usize)> {
        // Backslash escape: `\X` is always literal `X`, regardless of what X is.
        if chars[i] == '\\' && i + 1 < end {
            return Some((RichText::Plain(chars[i + 1].to_string()), i + 2));
        }

        // Inline code: `...` (not recursively parsed).
        if chars[i] == '`' {
            let close = find_char(chars, i + 1, end, '`')?;
            let content: String = chars[i + 1..close].iter().collect();
            return Some((
                RichText::Code(RichTextSimple::new(RichText::Plain(content))),
                close + 1,
            ));
        }

        // Math expression: $expr$ (raw LaTeX, not recursively parsed).
        if chars[i] == '$' {
            let close = find_char(chars, i + 1, end, '$')?;
            let expression: String = chars[i + 1..close].iter().collect();
            return Some((
                RichText::MathematicalExpression(RichTextMathematicalExpression { expression }),
                close + 1,
            ));
        }

        // Custom (Premium) emoji: ![fallback](tg://emoji?id=...)
        if chars[i] == '!' && chars.get(i + 1) == Some(&'[') {
            let close_bracket = find_char(chars, i + 2, end, ']')?;
            if chars.get(close_bracket + 1) != Some(&'(') {
                return None;
            }
            let close_paren = find_char(chars, close_bracket + 2, end, ')')?;
            let url: String = chars[close_bracket + 2..close_paren].iter().collect();
            let id = url.strip_prefix("tg://emoji?id=")?;
            let alternative_text: String = chars[i + 2..close_bracket].iter().collect();
            return Some((
                RichText::CustomEmoji(RichTextCustomEmoji {
                    custom_emoji_id: id.to_owned(),
                    alternative_text,
                }),
                close_paren + 1,
            ));
        }

        // Link: [text](url)
        if chars[i] == '[' {
            let close_bracket = find_char(chars, i + 1, end, ']')?;
            if chars.get(close_bracket + 1) != Some(&'(') {
                return None;
            }
            let close_paren = find_char(chars, close_bracket + 2, end, ')')?;
            let text = collapse(parse_inline_chars(chars, i + 1, close_bracket));
            let url: String = chars[close_bracket + 2..close_paren].iter().collect();
            return Some((
                RichText::Url(RichTextUrl { text: Box::new(text), url }),
                close_paren + 1,
            ));
        }

        for &(delim, wrap) in DOUBLE_DELIMS {
            if starts_with_at(chars, i, end, delim) {
                let delim_len = delim.chars().count();
                let close = find_str_at(chars, i + delim_len, end, delim)?;
                let inner = collapse(parse_inline_chars(chars, i + delim_len, close));
                return Some((wrap(inner), close + delim_len));
            }
        }

        for &(delim, wrap) in SINGLE_DELIMS {
            if chars[i] == delim {
                let close = find_char(chars, i + 1, end, delim)?;
                let inner = collapse(parse_inline_chars(chars, i + 1, close));
                return Some((wrap(inner), close + 1));
            }
        }

        None
    }

    fn starts_with_at(chars: &[char], i: usize, end: usize, s: &str) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        i + s_chars.len() <= end && chars[i..i + s_chars.len()] == s_chars[..]
    }

    fn find_char(chars: &[char], from: usize, end: usize, target: char) -> Option<usize> {
        (from..end).find(|&j| chars[j] == target)
    }

    fn find_str_at(chars: &[char], from: usize, end: usize, s: &str) -> Option<usize> {
        let s_chars: Vec<char> = s.chars().collect();
        let len = s_chars.len();
        if len == 0 || from + len > end {
            return None;
        }
        (from..=end - len).find(|&j| chars[j..j + len] == s_chars[..])
    }

    // ---------------------------------------------------------------------
    // HTML
    // ---------------------------------------------------------------------

    pub(super) fn html(text: &str) -> Vec<RichBlock> {
        let tokens = tokenize(text);
        HtmlParser { tokens: &tokens, pos: 0 }.parse_blocks(None).0
    }

    #[derive(Debug)]
    enum HtmlToken {
        Text(String),
        Open { name: String, attrs: Vec<(String, String)>, self_closing: bool },
        Close { name: String },
    }

    const VOID_ELEMENTS: &[&str] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ];

    fn tokenize(input: &str) -> Vec<HtmlToken> {
        let chars: Vec<char> = input.chars().collect();
        let mut tokens = Vec::new();
        let mut text_buf = String::new();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '<' {
                if let Some(close) = find_char(&chars, i + 1, chars.len(), '>') {
                    if !text_buf.is_empty() {
                        tokens.push(HtmlToken::Text(unescape_html(&std::mem::take(&mut text_buf))));
                    }
                    let raw: String = chars[i + 1..close].iter().collect();
                    let body = raw.trim();
                    if let Some(name) = body.strip_prefix('/') {
                        tokens.push(HtmlToken::Close { name: name.trim().to_lowercase() });
                    } else if !body.starts_with('!') && !body.is_empty() {
                        let self_closing_slash = body.ends_with('/');
                        let body = body.trim_end_matches('/').trim();
                        let mut parts = body.splitn(2, char::is_whitespace);
                        let name = parts.next().unwrap_or("").to_lowercase();
                        let attrs = parse_attrs(parts.next().unwrap_or(""));
                        let self_closing =
                            self_closing_slash || VOID_ELEMENTS.contains(&name.as_str());
                        tokens.push(HtmlToken::Open { name, attrs, self_closing });
                    }
                    i = close + 1;
                    continue;
                }
            }
            text_buf.push(chars[i]);
            i += 1;
        }
        if !text_buf.is_empty() {
            tokens.push(HtmlToken::Text(unescape_html(&text_buf)));
        }
        tokens
    }

    fn parse_attrs(s: &str) -> Vec<(String, String)> {
        let chars: Vec<char> = s.chars().collect();
        let mut attrs = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            let key_start = i;
            while i < chars.len() && chars[i] != '=' && !chars[i].is_whitespace() {
                i += 1;
            }
            if key_start == i {
                break;
            }
            let key: String = chars[key_start..i].iter().collect::<String>().to_lowercase();
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            if chars.get(i) == Some(&'=') {
                i += 1;
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                }
                if matches!(chars.get(i), Some('"') | Some('\'')) {
                    let quote = chars[i];
                    i += 1;
                    let val_start = i;
                    while i < chars.len() && chars[i] != quote {
                        i += 1;
                    }
                    let val: String = chars[val_start..i].iter().collect();
                    if i < chars.len() {
                        i += 1;
                    }
                    attrs.push((key, unescape_html(&val)));
                } else {
                    let val_start = i;
                    while i < chars.len() && !chars[i].is_whitespace() {
                        i += 1;
                    }
                    attrs.push((key, chars[val_start..i].iter().collect()));
                }
            } else {
                attrs.push((key, String::new()));
            }
        }
        attrs
    }

    fn unescape_html(s: &str) -> String {
        s.replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&amp;", "&")
    }

    fn is_block_tag(name: &str) -> bool {
        matches!(
            name,
            "p" | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "h5"
                | "h6"
                | "blockquote"
                | "aside"
                | "cite"
                | "pre"
                | "hr"
                | "ul"
                | "ol"
                | "li"
                | "table"
                | "tr"
                | "th"
                | "td"
                | "caption"
                | "details"
                | "summary"
                | "figure"
                | "figcaption"
                | "footer"
        )
    }

    struct HtmlParser<'a> {
        tokens: &'a [HtmlToken],
        pos: usize,
    }

    impl<'a> HtmlParser<'a> {
        fn peek(&self) -> Option<&HtmlToken> {
            self.tokens.get(self.pos)
        }

        /// Parses blocks until EOF or a closing tag matching `stop`
        /// (consumed). Also returns a trailing `<cite>`'s text, if any — used
        /// by `blockquote` to recover `RichBlockBlockquote::credit`.
        fn parse_blocks(&mut self, stop: Option<&str>) -> (Vec<RichBlock>, Option<String>) {
            let mut blocks = Vec::new();
            let mut credit = None;
            loop {
                match self.tokens.get(self.pos) {
                    None => break,
                    Some(HtmlToken::Close { name }) => {
                        if Some(name.as_str()) == stop {
                            self.pos += 1;
                        }
                        break;
                    }
                    Some(HtmlToken::Text(t)) => {
                        if t.trim().is_empty() {
                            self.pos += 1;
                            continue;
                        }
                        let text = self.parse_inline(None);
                        blocks.push(RichBlock::Paragraph(RichBlockParagraph { text }));
                    }
                    Some(HtmlToken::Open { name, .. }) => {
                        let name = name.clone();
                        match name.as_str() {
                            "p" => {
                                self.pos += 1;
                                let text = self.parse_inline(Some("p"));
                                blocks.push(RichBlock::Paragraph(RichBlockParagraph { text }));
                            }
                            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                                self.pos += 1;
                                let size = name[1..].parse().unwrap_or(1);
                                let text = self.parse_inline(Some(&name));
                                blocks.push(RichBlock::Heading(RichBlockHeading { text, size }));
                            }
                            "hr" => {
                                self.pos += 1;
                                blocks.push(RichBlock::Divider);
                            }
                            "blockquote" => {
                                self.pos += 1;
                                let (inner, credit) = self.parse_blocks(Some("blockquote"));
                                blocks.push(RichBlock::Blockquote(RichBlockBlockquote {
                                    blocks: inner,
                                    credit,
                                }));
                            }
                            "cite" => {
                                self.pos += 1;
                                let mut s = String::new();
                                self.parse_inline(Some("cite")).write_plain(&mut s);
                                credit = Some(s);
                            }
                            "aside" => {
                                // The counterpart of `RichBlock::Pullquote`'s HTML
                                // rendering — distinct from `<blockquote>`.
                                self.pos += 1;
                                let mut text = None;
                                let mut credit = None;
                                loop {
                                    match self.tokens.get(self.pos) {
                                        None => break,
                                        Some(HtmlToken::Close { name }) if name == "aside" => {
                                            self.pos += 1;
                                            break;
                                        }
                                        Some(HtmlToken::Open { name, .. })
                                            if name == "p" && text.is_none() =>
                                        {
                                            self.pos += 1;
                                            text = Some(self.parse_inline(Some("p")));
                                        }
                                        Some(HtmlToken::Open { name, .. }) if name == "cite" => {
                                            self.pos += 1;
                                            let t = self.parse_inline(Some("cite"));
                                            let mut s = String::new();
                                            t.write_plain(&mut s);
                                            credit = Some(s);
                                        }
                                        _ => self.pos += 1,
                                    }
                                }
                                blocks.push(RichBlock::Pullquote(RichBlockPullquote {
                                    text: text.unwrap_or(RichText::Plain(String::new())),
                                    credit,
                                }));
                            }
                            "pre" => {
                                self.pos += 1;
                                if matches!(self.peek(), Some(HtmlToken::Open { name, .. }) if name == "code")
                                {
                                    self.pos += 1;
                                }
                                let mut code_text = String::new();
                                loop {
                                    match self.tokens.get(self.pos) {
                                        None => break,
                                        Some(HtmlToken::Text(t)) => {
                                            code_text.push_str(t);
                                            self.pos += 1;
                                        }
                                        Some(HtmlToken::Close { name }) if name == "code" => {
                                            self.pos += 1;
                                        }
                                        Some(HtmlToken::Close { name }) if name == "pre" => {
                                            self.pos += 1;
                                            break;
                                        }
                                        _ => self.pos += 1,
                                    }
                                }
                                blocks.push(RichBlock::Pre(RichBlockPre {
                                    text: RichText::Plain(code_text),
                                    language: None,
                                }));
                            }
                            "ul" | "ol" => {
                                self.pos += 1;
                                let items = self.parse_list_items(&name);
                                blocks.push(RichBlock::List(RichBlockList { items }));
                            }
                            "table" => {
                                self.pos += 1;
                                let (cells, caption) = self.parse_table();
                                blocks.push(RichBlock::Table(RichBlockTable {
                                    cells,
                                    caption,
                                    is_bordered: true,
                                }));
                            }
                            "details" => {
                                self.pos += 1;
                                let mut summary = String::new();
                                if matches!(self.peek(), Some(HtmlToken::Open { name, .. }) if name == "summary")
                                {
                                    self.pos += 1;
                                    let text = self.parse_inline(Some("summary"));
                                    text.write_plain(&mut summary);
                                }
                                let (inner, _) = self.parse_blocks(Some("details"));
                                blocks.push(RichBlock::Details(RichBlockDetails {
                                    summary,
                                    blocks: inner,
                                }));
                            }
                            "footer" => {
                                self.pos += 1;
                                let text = self.parse_inline(Some("footer"));
                                blocks.push(RichBlock::Footer(super::RichBlockFooter { text }));
                            }
                            "figure" => {
                                self.pos += 1;
                                let mut caption = None;
                                loop {
                                    match self.tokens.get(self.pos) {
                                        None => break,
                                        Some(HtmlToken::Close { name }) if name == "figure" => {
                                            self.pos += 1;
                                            break;
                                        }
                                        Some(HtmlToken::Open { name, .. })
                                            if name == "figcaption" =>
                                        {
                                            self.pos += 1;
                                            caption = Some(self.parse_inline(Some("figcaption")));
                                        }
                                        _ => self.pos += 1,
                                    }
                                }
                                if let Some(text) = caption {
                                    blocks.push(RichBlock::Paragraph(RichBlockParagraph { text }));
                                }
                            }
                            _ => {
                                self.pos += 1;
                                let text = self.parse_inline(Some(&name));
                                blocks.push(RichBlock::Paragraph(RichBlockParagraph { text }));
                            }
                        }
                    }
                }
            }
            (blocks, credit)
        }

        fn parse_list_items(&mut self, list_tag: &str) -> Vec<RichListItem> {
            let mut items = Vec::new();
            loop {
                match self.tokens.get(self.pos) {
                    None => break,
                    Some(HtmlToken::Close { name }) if name == list_tag => {
                        self.pos += 1;
                        break;
                    }
                    Some(HtmlToken::Open { name, .. }) if name == "li" => {
                        self.pos += 1;
                        let mut has_checkbox = false;
                        let mut is_checked = false;
                        if let Some(HtmlToken::Open { name, attrs, .. }) = self.peek() {
                            if name == "input" {
                                has_checkbox =
                                    attrs.iter().any(|(k, v)| k == "type" && v == "checkbox");
                                is_checked = attrs.iter().any(|(k, _)| k == "checked");
                                self.pos += 1;
                            }
                        }
                        let (inner, _) = self.parse_blocks(Some("li"));
                        let ordered = list_tag == "ol";
                        items.push(RichListItem {
                            label: if ordered {
                                format!("{}.", items.len() + 1)
                            } else {
                                "•".to_owned()
                            },
                            blocks: inner,
                            kind: ordered.then(|| "1".to_owned()),
                            value: ordered.then(|| (items.len() + 1) as i64),
                            has_checkbox,
                            is_checked,
                        });
                    }
                    _ => self.pos += 1,
                }
            }
            items
        }

        fn parse_table(&mut self) -> (Vec<Vec<RichTableCell>>, Option<String>) {
            let mut cells = Vec::new();
            let mut caption = None;
            loop {
                match self.tokens.get(self.pos) {
                    None => break,
                    Some(HtmlToken::Close { name }) if name == "table" => {
                        self.pos += 1;
                        break;
                    }
                    Some(HtmlToken::Open { name, .. }) if name == "caption" => {
                        self.pos += 1;
                        let text = self.parse_inline(Some("caption"));
                        let mut s = String::new();
                        text.write_plain(&mut s);
                        caption = Some(s);
                    }
                    Some(HtmlToken::Open { name, .. }) if name == "tr" => {
                        self.pos += 1;
                        let mut row = Vec::new();
                        loop {
                            match self.tokens.get(self.pos) {
                                None => break,
                                Some(HtmlToken::Close { name }) if name == "tr" => {
                                    self.pos += 1;
                                    break;
                                }
                                Some(HtmlToken::Open { name, .. })
                                    if name == "th" || name == "td" =>
                                {
                                    let is_header = name == "th";
                                    let tag = name.clone();
                                    self.pos += 1;
                                    let text = self.parse_inline(Some(&tag));
                                    row.push(RichTableCell {
                                        text,
                                        is_header,
                                        align: None,
                                        valign: None,
                                    });
                                }
                                _ => self.pos += 1,
                            }
                        }
                        cells.push(row);
                    }
                    _ => self.pos += 1,
                }
            }
            (cells, caption)
        }

        /// Parses inline content until EOF, a closing tag matching `stop`
        /// (consumed), or — when `stop` is `None` (an implicit paragraph) — an
        /// unconsumed block-level opening tag.
        fn parse_inline(&mut self, stop: Option<&str>) -> RichText {
            let mut nodes = Vec::new();
            loop {
                match self.tokens.get(self.pos) {
                    None => break,
                    Some(HtmlToken::Close { name }) => {
                        if Some(name.as_str()) == stop {
                            self.pos += 1;
                        }
                        break;
                    }
                    Some(HtmlToken::Text(t)) => {
                        nodes.push(RichText::Plain(t.clone()));
                        self.pos += 1;
                    }
                    Some(HtmlToken::Open { name, attrs, self_closing }) => {
                        let name = name.clone();
                        let attrs = attrs.clone();
                        let self_closing = *self_closing;

                        if stop.is_none() && is_block_tag(&name) {
                            break;
                        }

                        self.pos += 1;
                        if self_closing {
                            continue;
                        }

                        match name.as_str() {
                            "b" | "strong" => {
                                let inner = self.parse_inline(Some(&name));
                                nodes.push(RichText::Bold(RichTextSimple::new(inner)));
                            }
                            "i" | "em" => {
                                let inner = self.parse_inline(Some(&name));
                                nodes.push(RichText::Italic(RichTextSimple::new(inner)));
                            }
                            "u" | "ins" => {
                                let inner = self.parse_inline(Some(&name));
                                nodes.push(RichText::Underline(RichTextSimple::new(inner)));
                            }
                            "s" | "strike" | "del" => {
                                let inner = self.parse_inline(Some(&name));
                                nodes.push(RichText::Strikethrough(RichTextSimple::new(inner)));
                            }
                            "tg-spoiler" => {
                                let inner = self.parse_inline(Some(&name));
                                nodes.push(RichText::Spoiler(RichTextSimple::new(inner)));
                            }
                            "tg-emoji" => {
                                let custom_emoji_id = attrs
                                    .iter()
                                    .find(|(k, _)| k == "emoji-id")
                                    .map(|(_, v)| v.clone())
                                    .unwrap_or_default();
                                let mut alternative_text = String::new();
                                self.parse_inline(Some("tg-emoji"))
                                    .write_plain(&mut alternative_text);
                                nodes.push(RichText::CustomEmoji(RichTextCustomEmoji {
                                    custom_emoji_id,
                                    alternative_text,
                                }));
                            }
                            "tg-math" => {
                                let mut expression = String::new();
                                self.parse_inline(Some("tg-math")).write_plain(&mut expression);
                                nodes.push(RichText::MathematicalExpression(
                                    RichTextMathematicalExpression { expression },
                                ));
                            }
                            "sub" => {
                                let inner = self.parse_inline(Some(&name));
                                nodes.push(RichText::Subscript(RichTextSimple::new(inner)));
                            }
                            "sup" => {
                                let inner = self.parse_inline(Some(&name));
                                nodes.push(RichText::Superscript(RichTextSimple::new(inner)));
                            }
                            "code" => {
                                let mut text = String::new();
                                loop {
                                    match self.tokens.get(self.pos) {
                                        None => break,
                                        Some(HtmlToken::Text(t)) => {
                                            text.push_str(t);
                                            self.pos += 1;
                                        }
                                        Some(HtmlToken::Close { name }) if name == "code" => {
                                            self.pos += 1;
                                            break;
                                        }
                                        _ => self.pos += 1,
                                    }
                                }
                                nodes.push(RichText::Code(RichTextSimple::new(RichText::Plain(
                                    text,
                                ))));
                            }
                            "span" => {
                                let is_spoiler = attrs
                                    .iter()
                                    .any(|(k, v)| k == "class" && v.contains("tg-spoiler"));
                                let inner = self.parse_inline(Some("span"));
                                nodes.push(if is_spoiler {
                                    RichText::Spoiler(RichTextSimple::new(inner))
                                } else {
                                    inner
                                });
                            }
                            "a" => {
                                let url = attrs
                                    .iter()
                                    .find(|(k, _)| k == "href")
                                    .map(|(_, v)| v.clone())
                                    .unwrap_or_default();
                                let inner = self.parse_inline(Some("a"));
                                nodes.push(RichText::Url(RichTextUrl {
                                    text: Box::new(inner),
                                    url,
                                }));
                            }
                            _ => {
                                // Unknown inline tag: keep the content, drop the tag.
                                nodes.push(self.parse_inline(Some(&name)));
                            }
                        }
                    }
                }
            }
            collapse(nodes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockquote_credit_renders_and_roundtrips() {
        let msg = RichMessage::new(vec![RichBlock::Blockquote(RichBlockBlockquote {
            blocks: vec![RichBlock::paragraph("тест")],
            credit: Some("тест автор".to_owned()),
        })]);

        let html = msg.to_html();
        assert!(html.contains("<cite>тест автор</cite>"));
        let from_html = InputRichMessage::parse(&html, RichParseMode::Html).blocks;
        assert!(
            matches!(&from_html[0], RichBlock::Blockquote(b) if b.credit.as_deref() == Some("тест автор"))
        );

        let markdown = msg.to_markdown();
        assert!(markdown.contains("> — тест автор"));
        let from_md = InputRichMessage::parse(&markdown, RichParseMode::Markdown).blocks;
        assert!(
            matches!(&from_md[0], RichBlock::Blockquote(b) if b.credit.as_deref() == Some("тест автор"))
        );
    }

    #[test]
    fn mathematical_expression_renders_and_roundtrips() {
        let msg = RichMessage::new(vec![RichBlock::paragraph(RichText::MathematicalExpression(
            RichTextMathematicalExpression { expression: "х+у=й".to_owned() },
        ))]);

        let html = msg.to_html();
        assert!(html.contains("<tg-math>х+у=й</tg-math>"));
        let from_html = InputRichMessage::parse(&html, RichParseMode::Html).blocks;
        assert!(matches!(
            &from_html[0],
            RichBlock::Paragraph(p) if matches!(&p.text, RichText::MathematicalExpression(e) if e.expression == "х+у=й")
        ));

        let markdown = msg.to_markdown();
        assert!(markdown.contains("$х+у=й$"));
        let from_md = InputRichMessage::parse(&markdown, RichParseMode::Markdown).blocks;
        assert!(matches!(
            &from_md[0],
            RichBlock::Paragraph(p) if matches!(&p.text, RichText::MathematicalExpression(e) if e.expression == "х+у=й")
        ));
    }

    #[test]
    fn custom_emoji_renders_and_roundtrips_with_its_id() {
        let msg = RichMessage::new(vec![RichBlock::paragraph(RichText::CustomEmoji(
            RichTextCustomEmoji {
                custom_emoji_id: "5436040291507247633".to_owned(),
                alternative_text: "🎉".to_owned(),
            },
        ))]);

        let html = msg.to_html();
        assert!(html.contains("<tg-emoji emoji-id=\"5436040291507247633\">🎉</tg-emoji>"));

        let markdown = msg.to_markdown();
        assert!(markdown.contains("![🎉](tg://emoji?id=5436040291507247633)"));

        let from_html = InputRichMessage::parse(&html, RichParseMode::Html).blocks;
        assert!(matches!(
            &from_html[0],
            RichBlock::Paragraph(p) if matches!(&p.text, RichText::CustomEmoji(e)
                if e.custom_emoji_id == "5436040291507247633" && e.alternative_text == "🎉")
        ));

        let from_md = InputRichMessage::parse(&markdown, RichParseMode::Markdown).blocks;
        assert!(matches!(
            &from_md[0],
            RichBlock::Paragraph(p) if matches!(&p.text, RichText::CustomEmoji(e)
                if e.custom_emoji_id == "5436040291507247633" && e.alternative_text == "🎉")
        ));
    }

    #[test]
    fn deserializes_table_with_rich_cell_text() {
        // Real-world payload: table cell `text` can be a formatted RichText
        // (object/array), not just a plain string.
        let json = serde_json::json!({
            "blocks": [{
                "type": "table",
                "cells": [
                    [{
                        "text": {
                            "type": "bold",
                            "text": [
                                {
                                    "type": "custom_emoji",
                                    "custom_emoji_id": "5436040291507247633",
                                    "alternative_text": "🎉"
                                },
                                " Платим 2000₽"
                            ]
                        },
                        "align": "left",
                        "valign": "top"
                    }],
                    [{
                        "text": "plain cell",
                        "align": "left",
                        "valign": "top"
                    }]
                ],
                "is_bordered": true
            }]
        });

        let rich: RichMessage = serde_json::from_value(json).expect("should deserialize");
        let RichBlock::Table(table) = &rich.blocks[0] else {
            panic!("table block must not fall back to Other: {:?}", rich.blocks[0])
        };
        assert!(matches!(&table.cells[0][0].text, RichText::Bold(_)));
        assert!(matches!(&table.cells[1][0].text, RichText::Plain(s) if s == "plain cell"));

        let plain = rich.plain_text();
        assert!(plain.contains("🎉"));
        assert!(plain.contains("Платим 2000₽"));
        assert!(plain.contains("plain cell"));
    }

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

    #[test]
    fn blockquote_and_pullquote_render_and_parse_distinctly() {
        let msg = RichMessage::new(vec![
            RichBlock::Blockquote(RichBlockBlockquote {
                blocks: vec![RichBlock::paragraph("regular quote")],
                credit: None,
            }),
            RichBlock::Pullquote(RichBlockPullquote {
                text: RichText::from("pulled quote"),
                credit: Some("author".to_owned()),
            }),
        ]);

        let html = msg.to_html();
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("<aside>"));
        assert!(!html.contains("<blockquote><p>pulled quote"));

        let markdown = msg.to_markdown();
        assert!(markdown.contains("> regular quote"));
        assert!(markdown.contains(">> pulled quote"));

        let from_html = InputRichMessage::parse(&html, RichParseMode::Html).blocks;
        assert!(matches!(&from_html[0], RichBlock::Blockquote(_)));
        assert!(
            matches!(&from_html[1], RichBlock::Pullquote(p) if p.credit.as_deref() == Some("author"))
        );

        let from_md = InputRichMessage::parse(&markdown, RichParseMode::Markdown).blocks;
        assert!(matches!(&from_md[0], RichBlock::Blockquote(_)));
        assert!(matches!(&from_md[1], RichBlock::Pullquote(_)));
    }

    #[test]
    fn parses_markdown_inline_and_blocks() {
        let src = "\
# Title

Some *bold* and _italic_ and __underline__ and ~strike~ and ||spoiler|| \
                   and `code` and [link](https://example.com) and an escaped \\* asterisk.

> Quote line

```rust
fn main() {}
```

- item1
- item2

1. one
2. two

---

| A | B |
| --- | --- |
| 1 | 2 |
";
        let blocks = InputRichMessage::parse(src, RichParseMode::Markdown).blocks;

        assert!(matches!(&blocks[0], RichBlock::Heading(h) if h.size == 1));
        let RichBlock::Paragraph(p) = &blocks[1] else { panic!("expected paragraph") };
        let RichText::Array(nodes) = &p.text else { panic!("expected formatted paragraph") };
        assert!(nodes.iter().any(|n| matches!(n, RichText::Bold(_))));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Italic(_))));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Underline(_))));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Strikethrough(_))));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Spoiler(_))));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Code(_))));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::Url(u) if u.url == "https://example.com")));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Plain(s) if s.contains('*'))));

        assert!(matches!(&blocks[2], RichBlock::Blockquote(_)));
        assert!(matches!(&blocks[3], RichBlock::Pre(p) if p.language.as_deref() == Some("rust")));

        let RichBlock::List(list) = &blocks[4] else { panic!("expected unordered list") };
        assert_eq!(list.items.len(), 2);
        assert!(list.items.iter().all(|i| i.kind.is_none()));

        let RichBlock::List(list) = &blocks[5] else { panic!("expected ordered list") };
        assert_eq!(list.items.len(), 2);
        assert!(list.items.iter().all(|i| i.kind.is_some()));
        assert_eq!(list.items[0].value, Some(1));
        assert_eq!(list.items[1].value, Some(2));

        assert!(matches!(&blocks[6], RichBlock::Divider));

        let RichBlock::Table(table) = &blocks[7] else { panic!("expected table") };
        assert_eq!(table.cells.len(), 2);
        assert!(table.cells[0].iter().all(|c| c.is_header));
        assert!(table.cells[1].iter().all(|c| !c.is_header));
        assert!(matches!(&table.cells[1][0].text, RichText::Plain(s) if s == "1"));
    }

    #[test]
    fn parses_html_inline_and_blocks() {
        let src = "\
<h2>Title</h2>
<p>Some <b>bold</b> and <i>italic</i> and <u>underline</u> and <s>strike</s> and \
<tg-spoiler>spoiler</tg-spoiler> and <code>code</code> and <a href=\"https://example.com\">link</a>.</p>
<blockquote><p>Quote line</p></blockquote>
<ul><li>item1</li><li><input type=\"checkbox\" checked>item2</li></ul>
<ol><li>one</li><li>two</li></ol>
<hr>
<table><tr><th>A</th><th>B</th></tr><tr><td>1</td><td>2</td></tr></table>
<details><summary>More</summary><p>hidden content</p></details>
";
        let blocks = InputRichMessage::parse(src, RichParseMode::Html).blocks;

        assert!(matches!(&blocks[0], RichBlock::Heading(h) if h.size == 2));

        let RichBlock::Paragraph(p) = &blocks[1] else { panic!("expected paragraph") };
        let RichText::Array(nodes) = &p.text else { panic!("expected formatted paragraph") };
        assert!(nodes.iter().any(|n| matches!(n, RichText::Bold(_))));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Spoiler(_))));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::Url(u) if u.url == "https://example.com")));

        assert!(matches!(&blocks[2], RichBlock::Blockquote(_)));

        let RichBlock::List(list) = &blocks[3] else { panic!("expected unordered list") };
        assert_eq!(list.items.len(), 2);
        assert!(!list.items[0].has_checkbox);
        assert!(list.items[1].has_checkbox && list.items[1].is_checked);

        let RichBlock::List(list) = &blocks[4] else { panic!("expected ordered list") };
        assert_eq!(list.items[0].value, Some(1));

        assert!(matches!(&blocks[5], RichBlock::Divider));

        let RichBlock::Table(table) = &blocks[6] else { panic!("expected table") };
        assert!(matches!(&table.cells[0][0].text, RichText::Plain(s) if s == "A"));
        assert!(matches!(&table.cells[1][1].text, RichText::Plain(s) if s == "2"));

        let RichBlock::Details(details) = &blocks[7] else { panic!("expected details") };
        assert_eq!(details.summary, "More");
        assert!(
            matches!(&details.blocks[0], RichBlock::Paragraph(p) if matches!(&p.text, RichText::Plain(s) if s == "hidden content"))
        );
    }

    fn sample_message() -> RichMessage {
        RichMessage::new(vec![
            RichBlock::heading("Title", 2),
            RichBlock::paragraph(RichText::Array(vec![
                RichText::from("plain "),
                RichText::Bold(RichTextSimple::new("bold")),
                RichText::from(" "),
                RichText::Url(RichTextUrl {
                    text: Box::new(RichText::from("link")),
                    url: "https://example.com/".into(),
                }),
            ])),
            RichBlock::List(RichBlockList {
                items: vec![RichListItem {
                    label: "•".into(),
                    blocks: vec![RichBlock::paragraph("item")],
                    kind: None,
                    value: None,
                    has_checkbox: false,
                    is_checked: false,
                }],
            }),
            RichBlock::divider(),
            RichBlock::Table(RichBlockTable {
                cells: vec![
                    vec![RichTableCell {
                        text: "H".into(),
                        is_header: true,
                        align: None,
                        valign: None,
                    }],
                    vec![RichTableCell {
                        text: "1".into(),
                        is_header: false,
                        align: None,
                        valign: None,
                    }],
                ],
                caption: None,
                is_bordered: true,
            }),
        ])
    }

    #[test]
    fn markdown_roundtrip_preserves_plain_text() {
        let original = sample_message();
        let reparsed = RichMessage::new(
            InputRichMessage::parse(&original.to_markdown(), RichParseMode::Markdown).blocks,
        );
        assert_eq!(original.plain_text(), reparsed.plain_text());
    }

    #[test]
    fn html_roundtrip_preserves_plain_text() {
        let original = sample_message();
        let reparsed = RichMessage::new(
            InputRichMessage::parse(&original.to_html(), RichParseMode::Html).blocks,
        );
        assert_eq!(original.plain_text(), reparsed.plain_text());
    }
}
