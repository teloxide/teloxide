use serde::{de::Error as _, Deserialize, Deserializer, Serialize};

use crate::types::{
    Animation, Audio, InputFile, InputFileLike, InputMediaAnimation, InputMediaAudio,
    InputMediaPhoto, InputMediaVideo, InputMediaVoiceNote, Location, PhotoSize, User, Video, Voice,
};

/// The pseudo-URL a custom (Premium) emoji occupies inside Markdown's image
/// syntax, `![fallback](tg://emoji?id=...)` — Telegram's own MarkdownV2
/// convention.
const CUSTOM_EMOJI_SCHEME: &str = "tg://emoji?id=";

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
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessage {
    /// The blocks that make up the rich message.
    #[serde(default)]
    #[cfg_attr(test, schemars(with = "serde_json::Value"))]
    pub blocks: Vec<RichBlock>,

    /// `true`, if the rich message must be shown right-to-left.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_rtl: bool,
}

impl RichMessage {
    pub fn new<B>(blocks: B) -> Self
    where
        B: IntoIterator<Item = RichBlock>,
    {
        Self { blocks: blocks.into_iter().collect(), is_rtl: false }
    }

    /// Renders the rich message as full HTML markup: inline formatting uses
    /// the same tags as Telegram's [HTML formatted] messages (`<b>`, `<i>`,
    /// `<a>`, ...), while structural blocks use ordinary HTML elements
    /// (`<h1>`-`<h6>`, `<ul>`/`<ol>`/`<li>`, `<table>`, `<hr>`,
    /// `<details>`, ...).
    ///
    /// Note: this is the *rich* message HTML dialect. It is not accepted by
    /// ordinary messages sent with `parse_mode: Html`, whose parser takes
    /// only a small whitelist of inline tags and rejects `<table>`, `<ul>`,
    /// `<hr>` and the like — use [`plain_text`] for those.
    ///
    /// This output is meant for display (e.g. a web page or log viewer), not
    /// for sending back to Telegram. In particular, a [`RichBlock::Photo`]
    /// renders only its caption — the API gives a received photo no link or
    /// short id to put in an `<img src>` that would mean anything to
    /// Telegram, so nothing is rendered for the image itself. Use
    /// [`Self::blocks`] directly to get at the photo (its `file_id`s, sizes,
    /// ...) if you need it.
    ///
    /// [HTML formatted]: https://core.telegram.org/bots/api#html-style
    /// [`plain_text`]: RichMessage::plain_text
    pub fn to_html(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.write_html(&mut out);
        }
        out.trim_end_matches('\n').to_owned()
    }

    /// Renders the rich message as [rich Markdown formatted] text.
    ///
    /// As with [`to_html`], this is for display only — see there for why.
    ///
    /// [rich Markdown formatted]: https://core.telegram.org/bots/api#rich-markdown-style
    /// [`to_html`]: RichMessage::to_html
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
///
/// The outgoing counterpart is [`InputRichBlock`], which the variants here
/// are named to match.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "serde_json::Value")]
pub enum RichBlock {
    Paragraph(RichBlockParagraph),
    SectionHeading(RichBlockSectionHeading),
    Preformatted(RichBlockPreformatted),
    Footer(RichBlockFooter),
    Divider,
    /// A block with a mathematical expression in LaTeX format, corresponding
    /// to the custom HTML tag `<tg-math-block>`. Distinct from
    /// [`RichText::MathematicalExpression`], which is inline.
    MathematicalExpression(RichBlockMathematicalExpression),
    /// A block with an anchor, corresponding to the HTML tag `<a>` with the
    /// attribute `name`.
    Anchor(RichBlockAnchor),
    List(RichBlockList),
    BlockQuotation(RichBlockBlockQuotation),
    PullQuotation(RichBlockPullQuotation),
    /// A collage, corresponding to the custom HTML tag `<tg-collage>`.
    Collage(RichBlockCollage),
    /// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Details(RichBlockDetails),
    /// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
    Map(RichBlockMap),
    /// A block with an animation, corresponding to the HTML tag `<video>`.
    Animation(RichBlockAnimation),
    /// A block with a music file, corresponding to the HTML tag `<audio>`.
    Audio(RichBlockAudio),
    Photo(RichBlockPhoto),
    /// A block with a video, corresponding to the HTML tag `<video>`.
    Video(RichBlockVideo),
    /// A block with a voice note, corresponding to the HTML tag `<audio>`.
    VoiceNote(RichBlockVoiceNote),
    /// A block with a "Thinking..." placeholder, corresponding to the custom
    /// HTML tag `<tg-thinking>`. Only used by `sendRichMessageDraft`.
    Thinking(RichBlockThinking),
    /// A block `type` not (yet) understood by teloxide. The raw JSON fields
    /// (other than `type`) are preserved verbatim.
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

        macro_rules! variant {
            ($Variant:ident) => {
                serde_json::from_value(value.clone()).ok().map(RichBlock::$Variant)
            };
        }

        let parsed = match kind.as_str() {
            "paragraph" => variant!(Paragraph),
            "heading" => variant!(SectionHeading),
            "pre" => variant!(Preformatted),
            "footer" => variant!(Footer),
            "divider" => Some(RichBlock::Divider),
            "mathematical_expression" => variant!(MathematicalExpression),
            "anchor" => variant!(Anchor),
            "list" => variant!(List),
            "blockquote" => variant!(BlockQuotation),
            "pullquote" => variant!(PullQuotation),
            "collage" => variant!(Collage),
            "slideshow" => variant!(Slideshow),
            "table" => variant!(Table),
            "details" => variant!(Details),
            "map" => variant!(Map),
            "animation" => variant!(Animation),
            "audio" => variant!(Audio),
            "photo" => variant!(Photo),
            "video" => variant!(Video),
            "voice_note" => variant!(VoiceNote),
            "thinking" => variant!(Thinking),
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
            RichBlock::SectionHeading(b) => tagged("heading", &b),
            RichBlock::Preformatted(b) => tagged("pre", &b),
            RichBlock::Footer(b) => tagged("footer", &b),
            RichBlock::Divider => {
                serde_json::json!({ "type": "divider" })
            }
            RichBlock::MathematicalExpression(b) => tagged("mathematical_expression", &b),
            RichBlock::Anchor(b) => tagged("anchor", &b),
            RichBlock::List(b) => tagged("list", &b),
            RichBlock::BlockQuotation(b) => tagged("blockquote", &b),
            RichBlock::PullQuotation(b) => tagged("pullquote", &b),
            RichBlock::Collage(b) => tagged("collage", &b),
            RichBlock::Slideshow(b) => tagged("slideshow", &b),
            RichBlock::Table(b) => tagged("table", &b),
            RichBlock::Details(b) => tagged("details", &b),
            RichBlock::Map(b) => tagged("map", &b),
            RichBlock::Animation(b) => tagged("animation", &b),
            RichBlock::Audio(b) => tagged("audio", &b),
            RichBlock::Photo(b) => tagged("photo", &b),
            RichBlock::Video(b) => tagged("video", &b),
            RichBlock::VoiceNote(b) => tagged("voice_note", &b),
            RichBlock::Thinking(b) => tagged("thinking", &b),
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
        Self::SectionHeading(RichBlockSectionHeading { text: text.into(), size })
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
            RichBlock::SectionHeading(b) => {
                let level = b.size.clamp(1, 6);
                out.push_str(&format!("<h{level}>"));
                b.text.write_html(out);
                out.push_str(&format!("</h{level}>\n"));
            }
            RichBlock::BlockQuotation(b) => {
                out.push_str("<blockquote>\n");
                for inner in &b.blocks {
                    inner.write_html(out);
                }
                if let Some(credit) = &b.credit {
                    out.push_str("<cite>");
                    credit.write_html(out);
                    out.push_str("</cite>\n");
                }
                out.push_str("</blockquote>\n");
            }
            RichBlock::PullQuotation(b) => {
                // `<aside>` (not `<blockquote>`) because a pull quote is a
                // separate, decorative excerpt, not the same thing as a
                // regular quote block.
                out.push_str("<aside>");
                b.text.write_html(out);
                if let Some(credit) = &b.credit {
                    out.push_str("<cite>");
                    credit.write_html(out);
                    out.push_str("</cite>");
                }
                out.push_str("</aside>\n");
            }
            RichBlock::Preformatted(b) => {
                out.push_str("<pre><code");
                if let Some(language) = &b.language {
                    out.push_str(" class=\"language-");
                    out.push_str(&escape_html_attr(language));
                    out.push('"');
                }
                out.push('>');
                b.text.write_html(out);
                out.push_str("</code></pre>\n");
            }
            RichBlock::Footer(b) => {
                out.push_str("<footer>");
                b.text.write_html(out);
                out.push_str("</footer>\n");
            }
            RichBlock::Divider => out.push_str("<hr/>\n"),
            RichBlock::MathematicalExpression(b) => {
                out.push_str("<tg-math-block>");
                out.push_str(&escape_html(&b.expression));
                out.push_str("</tg-math-block>\n");
            }
            RichBlock::Anchor(b) => {
                out.push_str("<a name=\"");
                out.push_str(&escape_html_attr(&b.name));
                out.push_str("\"></a>\n");
            }
            RichBlock::List(b) => {
                let ordered = b.items.first().is_some_and(|item| item.kind.is_some());
                let tag = if ordered { "ol" } else { "ul" };
                out.push_str(&format!("<{tag}>\n"));
                for item in &b.items {
                    out.push_str("<li>");
                    if item.has_checkbox {
                        out.push_str("<input type=\"checkbox\"");
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
            RichBlock::Collage(b) => {
                out.push_str("<tg-collage>");
                for inner in &b.blocks {
                    inner.write_html(out);
                }
                if let Some(caption) = &b.caption {
                    out.push_str("<figcaption>");
                    caption.text.write_html(out);
                    if let Some(credit) = &caption.credit {
                        out.push_str("<cite>");
                        credit.write_html(out);
                        out.push_str("</cite>");
                    }
                    out.push_str("</figcaption>");
                }
                out.push_str("</tg-collage>\n");
            }
            RichBlock::Slideshow(b) => {
                out.push_str("<tg-slideshow>");
                for inner in &b.blocks {
                    inner.write_html(out);
                }
                if let Some(caption) = &b.caption {
                    out.push_str("<figcaption>");
                    caption.text.write_html(out);
                    if let Some(credit) = &caption.credit {
                        out.push_str("<cite>");
                        credit.write_html(out);
                        out.push_str("</cite>");
                    }
                    out.push_str("</figcaption>");
                }
                out.push_str("</tg-slideshow>\n");
            }
            RichBlock::Details(b) => {
                out.push_str("<details><summary>");
                b.summary.write_html(out);
                out.push_str("</summary>\n");
                for inner in &b.blocks {
                    inner.write_html(out);
                }
                out.push_str("</details>\n");
            }
            RichBlock::Map(b) => {
                // `width`/`height` size the static map image the API
                // renders server-side — the docs' own example only puts
                // `lat`/`long`/`zoom` on the tag itself.
                out.push_str(&format!(
                    "<figure><tg-map lat=\"{}\" long=\"{}\" zoom=\"{}\"/>",
                    b.location.latitude, b.location.longitude, b.zoom
                ));
                if let Some(caption) = &b.caption {
                    out.push_str("<figcaption>");
                    caption.text.write_html(out);
                    if let Some(credit) = &caption.credit {
                        out.push_str("<cite>");
                        credit.write_html(out);
                        out.push_str("</cite>");
                    }
                    out.push_str("</figcaption>");
                }
                out.push_str("</figure>\n");
            }
            RichBlock::Animation(b) => write_media_caption_html(out, b.caption.as_ref()),
            RichBlock::Audio(b) => write_media_caption_html(out, b.caption.as_ref()),
            RichBlock::Video(b) => write_media_caption_html(out, b.caption.as_ref()),
            RichBlock::VoiceNote(b) => write_media_caption_html(out, b.caption.as_ref()),
            RichBlock::Thinking(b) => {
                out.push_str("<tg-thinking>");
                b.text.write_html(out);
                out.push_str("</tg-thinking>\n");
            }
            RichBlock::Table(b) => {
                out.push_str("<table");
                if b.is_bordered {
                    out.push_str(" bordered");
                }
                if b.is_striped {
                    out.push_str(" striped");
                }
                out.push_str(">\n");
                if let Some(caption) = &b.caption {
                    out.push_str("<caption>");
                    caption.write_html(out);
                    out.push_str("</caption>\n");
                }
                for row in &b.cells {
                    out.push_str("<tr>");
                    for cell in row {
                        let tag = if cell.is_header { "th" } else { "td" };
                        out.push_str(&format!("<{tag}"));
                        if let Some(colspan) = cell.colspan {
                            out.push_str(&format!(" colspan=\"{colspan}\""));
                        }
                        if let Some(rowspan) = cell.rowspan {
                            out.push_str(&format!(" rowspan=\"{rowspan}\""));
                        }
                        out.push_str(&format!(
                            " align=\"{}\" valign=\"{}\">",
                            cell.align.as_str(),
                            cell.valign.as_str()
                        ));
                        if let Some(text) = &cell.text {
                            text.write_html(out);
                        }
                        out.push_str(&format!("</{tag}>"));
                    }
                    out.push_str("</tr>\n");
                }
                out.push_str("</table>\n");
            }
            RichBlock::Photo(b) => write_media_caption_html(out, b.caption.as_ref()),
            RichBlock::Other { .. } => {}
        }
    }

    fn write_markdown(&self, out: &mut String) {
        match self {
            RichBlock::Paragraph(b) => {
                b.text.write_markdown(out);
                out.push_str("\n\n");
            }
            RichBlock::SectionHeading(b) => {
                out.push_str(&"#".repeat(b.size.clamp(1, 6) as usize));
                out.push(' ');
                b.text.write_markdown(out);
                out.push_str("\n\n");
            }
            RichBlock::BlockQuotation(b) => {
                for inner in &b.blocks {
                    let mut buf = String::new();
                    inner.write_markdown(&mut buf);
                    for line in buf.trim_end().lines() {
                        out.push_str("> ");
                        out.push_str(line);
                        out.push('\n');
                    }
                }
                // Rich Markdown has no attribution syntax, but it does accept
                // the `<cite>` a block quotation's credit maps to.
                if let Some(credit) = &b.credit {
                    out.push_str("> <cite>");
                    credit.write_html(out);
                    out.push_str("</cite>\n");
                }
                out.push('\n');
            }
            RichBlock::PullQuotation(b) => {
                // Rich Markdown has no pull-quote syntax at all — `<aside>` is
                // the documented way to write one.
                out.push_str("<aside>");
                b.text.write_markdown(out);
                if let Some(credit) = &b.credit {
                    out.push_str("<cite>");
                    credit.write_html(out);
                    out.push_str("</cite>");
                }
                out.push_str("</aside>\n\n");
            }
            RichBlock::Preformatted(b) => {
                out.push_str("```");
                out.push_str(b.language.as_deref().unwrap_or(""));
                out.push('\n');
                b.text.write_plain(out);
                out.push_str("\n```\n\n");
            }
            RichBlock::Footer(b) => {
                // Rich Markdown has no footer syntax; `<footer>` is the
                // documented way to write one.
                out.push_str("<footer>");
                b.text.write_markdown(out);
                out.push_str("</footer>\n\n");
            }
            RichBlock::Divider => out.push_str("\n---\n\n"),
            RichBlock::MathematicalExpression(b) => {
                // `$$...$$`: the documented block-formula convention (the
                // other, ` ```math `, needs fence-conflict handling `Pre`
                // doesn't need, since a formula can't itself contain a fence).
                out.push_str("$$");
                out.push_str(&b.expression);
                out.push_str("$$\n\n");
            }
            RichBlock::Anchor(b) => {
                // No Markdown syntax for this either — same HTML fallback as
                // `to_html`.
                out.push_str("<a name=\"");
                out.push_str(&escape_html_attr(&b.name));
                out.push_str("\"></a>\n\n");
            }
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
            // `<tg-collage>`/`<tg-slideshow>`, like `<details>`, keep parsing
            // Markdown inside them.
            RichBlock::Collage(b) => {
                out.push_str("<tg-collage>\n\n");
                for inner in &b.blocks {
                    inner.write_markdown(out);
                }
                if let Some(caption) = &b.caption {
                    out.push_str("<figcaption>");
                    caption.text.write_markdown(out);
                    if let Some(credit) = &caption.credit {
                        out.push_str("<cite>");
                        credit.write_html(out);
                        out.push_str("</cite>");
                    }
                    out.push_str("</figcaption>\n\n");
                }
                out.push_str("</tg-collage>\n\n");
            }
            RichBlock::Slideshow(b) => {
                out.push_str("<tg-slideshow>\n\n");
                for inner in &b.blocks {
                    inner.write_markdown(out);
                }
                if let Some(caption) = &b.caption {
                    out.push_str("<figcaption>");
                    caption.text.write_markdown(out);
                    if let Some(credit) = &caption.credit {
                        out.push_str("<cite>");
                        credit.write_html(out);
                        out.push_str("</cite>");
                    }
                    out.push_str("</figcaption>\n\n");
                }
                out.push_str("</tg-slideshow>\n\n");
            }
            RichBlock::Details(b) => {
                // `<details>` is one of the few block tags rich Markdown keeps
                // parsing Markdown inside of.
                out.push_str("<details><summary>");
                b.summary.write_markdown(out);
                out.push_str("</summary>\n\n");
                for inner in &b.blocks {
                    inner.write_markdown(out);
                }
                out.push_str("</details>\n\n");
            }
            RichBlock::Map(b) => {
                out.push_str(&format!(
                    "<tg-map lat=\"{}\" long=\"{}\" zoom=\"{}\"/>\n\n",
                    b.location.latitude, b.location.longitude, b.zoom
                ));
                if let Some(caption) = &b.caption {
                    caption.text.write_markdown(out);
                    out.push_str("\n\n");
                }
            }
            RichBlock::Animation(b) => write_media_caption_markdown(out, b.caption.as_ref()),
            RichBlock::Audio(b) => write_media_caption_markdown(out, b.caption.as_ref()),
            RichBlock::Video(b) => write_media_caption_markdown(out, b.caption.as_ref()),
            RichBlock::VoiceNote(b) => write_media_caption_markdown(out, b.caption.as_ref()),
            RichBlock::Thinking(b) => {
                out.push_str("<tg-thinking>");
                b.text.write_markdown(out);
                out.push_str("</tg-thinking>\n\n");
            }
            RichBlock::Table(b) => {
                for (i, row) in b.cells.iter().enumerate() {
                    let rendered: Vec<_> = row
                        .iter()
                        .map(|c| {
                            let mut s = String::new();
                            if let Some(text) = &c.text {
                                text.write_markdown(&mut s);
                            }
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
            RichBlock::Photo(b) => write_media_caption_markdown(out, b.caption.as_ref()),
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
            RichBlock::SectionHeading(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::BlockQuotation(b) => {
                for inner in &b.blocks {
                    inner.write_plain(out);
                }
            }
            RichBlock::PullQuotation(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Preformatted(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Divider => out.push('\n'),
            RichBlock::MathematicalExpression(b) => {
                out.push_str(&b.expression);
                out.push_str("\n\n");
            }
            RichBlock::Anchor(_) => {}
            RichBlock::List(b) => {
                for item in &b.items {
                    for inner in &item.blocks {
                        inner.write_plain(out);
                    }
                }
            }
            RichBlock::Collage(b) => {
                for inner in &b.blocks {
                    inner.write_plain(out);
                }
                write_media_caption_plain(out, b.caption.as_ref());
            }
            RichBlock::Slideshow(b) => {
                for inner in &b.blocks {
                    inner.write_plain(out);
                }
                write_media_caption_plain(out, b.caption.as_ref());
            }
            RichBlock::Details(b) => {
                b.summary.write_plain(out);
                out.push('\n');
                for inner in &b.blocks {
                    inner.write_plain(out);
                }
            }
            RichBlock::Map(b) => write_media_caption_plain(out, b.caption.as_ref()),
            RichBlock::Animation(b) => write_media_caption_plain(out, b.caption.as_ref()),
            RichBlock::Audio(b) => write_media_caption_plain(out, b.caption.as_ref()),
            RichBlock::Video(b) => write_media_caption_plain(out, b.caption.as_ref()),
            RichBlock::VoiceNote(b) => write_media_caption_plain(out, b.caption.as_ref()),
            RichBlock::Thinking(b) => {
                b.text.write_plain(out);
                out.push_str("\n\n");
            }
            RichBlock::Table(b) => {
                for row in &b.cells {
                    let rendered: Vec<_> = row
                        .iter()
                        .map(|c| {
                            let mut s = String::new();
                            if let Some(text) = &c.text {
                                text.write_plain(&mut s);
                            }
                            s
                        })
                        .collect();
                    out.push_str(&rendered.join(" "));
                    out.push('\n');
                }
            }
            RichBlock::Photo(b) => write_media_caption_plain(out, b.caption.as_ref()),
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
pub struct RichBlockSectionHeading {
    pub text: RichText,
    pub size: u8,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockBlockQuotation {
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
    pub credit: Option<RichText>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockPullQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockPreformatted {
    pub text: RichText,
    pub language: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockFooter {
    pub text: RichText,
}

/// A block with a mathematical expression in LaTeX format, corresponding to
/// the custom HTML tag `<tg-math-block>`.
///
/// Distinct from [`RichTextMathematicalExpression`], which is inline.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmathematicalexpression).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockMathematicalExpression {
    pub expression: String,
}

/// A block with an anchor, corresponding to the HTML tag `<a>` with the
/// attribute `name`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanchor).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockAnchor {
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockList {
    #[serde(default)]
    pub items: Vec<RichBlockListItem>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockListItem {
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

/// A collage, corresponding to the custom HTML tag `<tg-collage>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcollage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockCollage {
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
    pub caption: Option<RichBlockCaption>,
}

/// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockslideshow).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockSlideshow {
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockDetails {
    pub summary: RichText,
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
    /// `true` if the block is expanded by default.
    #[serde(default)]
    pub is_open: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockTable {
    #[serde(default)]
    pub cells: Vec<Vec<RichBlockTableCell>>,
    pub caption: Option<RichText>,
    #[serde(default)]
    pub is_bordered: bool,
    #[serde(default)]
    pub is_striped: bool,
}

/// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmap).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RichBlockMap {
    pub location: Location,
    pub zoom: u8,
    pub width: u32,
    pub height: u32,
    pub caption: Option<RichBlockCaption>,
}

// `Location` holds `f64`s, so this hashes its serialized form instead of
// deriving (like `RichBlock`'s `Hash` impl does for the same reason).
impl std::hash::Hash for RichBlockMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        serde_json::to_string(self).unwrap_or_default().hash(state);
    }
}

impl Eq for RichBlockMap {}

/// A block with an animation, corresponding to the HTML tag `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanimation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockAnimation {
    pub animation: Animation,
    #[serde(default)]
    pub has_spoiler: bool,
    pub caption: Option<RichBlockCaption>,
}

/// A block with a music file, corresponding to the HTML tag `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockaudio).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockAudio {
    pub audio: Audio,
    pub caption: Option<RichBlockCaption>,
}

/// A block with a video, corresponding to the HTML tag `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockVideo {
    pub video: Video,
    #[serde(default)]
    pub has_spoiler: bool,
    pub caption: Option<RichBlockCaption>,
}

/// A block with a voice note, corresponding to the HTML tag `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvoicenote).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockVoiceNote {
    pub voice_note: Voice,
    pub caption: Option<RichBlockCaption>,
}

/// A block with a "Thinking..." placeholder, corresponding to the custom
/// HTML tag `<tg-thinking>`. Only used by `sendRichMessageDraft`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockthinking).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockThinking {
    pub text: RichText,
}

/// Cell in a [`RichBlockTable`].
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktablecell).
#[serde_with::skip_serializing_none]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockTableCell {
    /// Content of the cell. Table cells can contain only inline formatting.
    pub text: Option<RichText>,

    /// `true` if the cell is a header cell.
    #[serde(default)]
    pub is_header: bool,

    /// The number of columns the cell spans.
    pub colspan: Option<u32>,

    /// The number of rows the cell spans.
    pub rowspan: Option<u32>,

    /// Horizontal cell content alignment.
    #[serde(default)]
    pub align: RichBlockTableCellAlign,

    /// Vertical cell content alignment.
    #[serde(default)]
    pub valign: RichBlockTableCellVerticalAlign,
}

/// Horizontal content alignment of a [`RichBlockTableCell`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum RichBlockTableCellAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl RichBlockTableCellAlign {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

/// Vertical content alignment of a [`RichBlockTableCell`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum RichBlockTableCellVerticalAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

impl RichBlockTableCellVerticalAlign {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Middle => "middle",
            Self::Bottom => "bottom",
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockPhoto {
    /// Available sizes of the photo.
    pub photo: Vec<PhotoSize>,
    /// `true`, if the media preview is covered by a spoiler animation.
    #[serde(default)]
    pub has_spoiler: bool,
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockPhoto {
    /// The largest available [`PhotoSize`].
    pub fn largest(&self) -> Option<&PhotoSize> {
        self.photo.iter().max_by_key(|p| (p.width as u64) * (p.height as u64))
    }
}

/// Caption of a rich formatted block, attached to media blocks such as
/// [`RichBlockPhoto`].
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcaption).
#[serde_with::skip_serializing_none]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichBlockCaption {
    /// Block caption.
    pub text: RichText,

    /// Block credit, corresponding to the HTML tag `<cite>`.
    pub credit: Option<RichText>,
}

/// Formatted inline text used throughout [`RichBlock`]s.
///
/// A node is either a plain string, a sequence of nodes, or a tagged
/// formatting wrapper (bold, italic, link, ...) around another node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "serde_json::Value")]
pub enum RichText {
    // NB. `RichText` is a string, an array or a tagged object depending on the
    // node, so its schema is simply "any JSON value" — see the manual
    // `JsonSchema` impl below.
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
    /// Formatted date and time, corresponding to the custom HTML tag
    /// `<tg-time>`.
    DateTime(RichTextDateTime),
    /// A mention of a Telegram user by their identifier, corresponding to a
    /// `tg://user?id=...` link.
    TextMention(RichTextTextMention),
    /// An automatically detected e-mail address, corresponding to a
    /// `mailto:...` link.
    EmailAddress(RichTextEmailAddress),
    /// An automatically detected phone number, corresponding to a `tel:...`
    /// link.
    PhoneNumber(RichTextPhoneNumber),
    /// An automatically detected bank card number.
    BankCardNumber(RichTextBankCardNumber),
    /// An automatically detected `@username` mention.
    Mention(RichTextMention),
    /// An automatically detected `#hashtag`.
    Hashtag(RichTextHashtag),
    /// An automatically detected `$cashtag`.
    Cashtag(RichTextCashtag),
    /// An automatically detected `/bot_command`.
    BotCommand(RichTextBotCommand),
    /// An anchor, corresponding to the HTML tag `<a name="...">`.
    Anchor(RichTextAnchor),
    /// A link to an anchor, corresponding to `<a href="#...">`.
    AnchorLink(RichTextAnchorLink),
    /// A reference (footnote), corresponding to the custom HTML tag
    /// `<tg-reference>`.
    Reference(RichTextReference),
    /// A link to a [`Reference`](RichText::Reference), corresponding to
    /// `<a href="#...">`.
    ReferenceLink(RichTextReferenceLink),
    /// A `type` not (yet) understood by teloxide (e.g. `mention`,
    /// `hashtag`, `email_address`, ...). The raw JSON fields (other than
    /// `type`) are preserved verbatim.
    Other {
        kind: String,
        raw: serde_json::Value,
    },
}

#[cfg(test)]
impl schemars::JsonSchema for RichText {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RichText".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <serde_json::Value as schemars::JsonSchema>::json_schema(generator)
    }
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
            RichText::DateTime(t) => tagged("date_time", &t),
            RichText::TextMention(t) => tagged("text_mention", &t),
            RichText::EmailAddress(t) => tagged("email_address", &t),
            RichText::PhoneNumber(t) => tagged("phone_number", &t),
            RichText::BankCardNumber(t) => tagged("bank_card_number", &t),
            RichText::Mention(t) => tagged("mention", &t),
            RichText::Hashtag(t) => tagged("hashtag", &t),
            RichText::Cashtag(t) => tagged("cashtag", &t),
            RichText::BotCommand(t) => tagged("bot_command", &t),
            RichText::Anchor(t) => tagged("anchor", &t),
            RichText::AnchorLink(t) => tagged("anchor_link", &t),
            RichText::Reference(t) => tagged("reference", &t),
            RichText::ReferenceLink(t) => tagged("reference_link", &t),
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

                // Every variant's payload is `#[derive(Deserialize)]`, so
                // this covers them all identically regardless of shape.
                macro_rules! variant {
                    ($Variant:ident) => {
                        serde_json::from_value(value.clone()).ok().map(RichText::$Variant)
                    };
                }

                let parsed = match kind.as_str() {
                    "bold" => variant!(Bold),
                    "italic" => variant!(Italic),
                    "underline" => variant!(Underline),
                    "strikethrough" => variant!(Strikethrough),
                    "spoiler" => variant!(Spoiler),
                    "subscript" => variant!(Subscript),
                    "superscript" => variant!(Superscript),
                    "marked" => variant!(Marked),
                    "code" => variant!(Code),
                    "url" => variant!(Url),
                    "mathematical_expression" => variant!(MathematicalExpression),
                    "custom_emoji" => variant!(CustomEmoji),
                    "date_time" => variant!(DateTime),
                    "text_mention" => variant!(TextMention),
                    "email_address" => variant!(EmailAddress),
                    "phone_number" => variant!(PhoneNumber),
                    "bank_card_number" => variant!(BankCardNumber),
                    "mention" => variant!(Mention),
                    "hashtag" => variant!(Hashtag),
                    "cashtag" => variant!(Cashtag),
                    "bot_command" => variant!(BotCommand),
                    "anchor" => variant!(Anchor),
                    "anchor_link" => variant!(AnchorLink),
                    "reference" => variant!(Reference),
                    "reference_link" => variant!(ReferenceLink),
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
            RichText::Subscript(t) => wrap_html(out, "sub", &t.text),
            RichText::Superscript(t) => wrap_html(out, "sup", &t.text),
            RichText::Marked(t) => wrap_html(out, "mark", &t.text),
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
            // "Links `mailto:...`, `tel:...`, and `tg://user?id=...` are
            // rendered as e-mail links, phone links, and inline mentions
            // respectively."
            RichText::TextMention(t) => {
                out.push_str(&format!("<a href=\"tg://user?id={}\">", t.user.id.0));
                t.text.write_html(out);
                out.push_str("</a>");
            }
            RichText::EmailAddress(t) => {
                out.push_str("<a href=\"mailto:");
                out.push_str(&escape_html_attr(&t.email_address));
                out.push_str("\">");
                t.text.write_html(out);
                out.push_str("</a>");
            }
            RichText::PhoneNumber(t) => {
                out.push_str("<a href=\"tel:");
                out.push_str(&escape_html_attr(&t.phone_number));
                out.push_str("\">");
                t.text.write_html(out);
                out.push_str("</a>");
            }
            // No dedicated tag is documented for these — they're products of
            // automatic entity detection, not something a bot author wraps
            // in special markup, so only their (already-formatted) text is
            // rendered.
            RichText::BankCardNumber(t) => t.text.write_html(out),
            RichText::Mention(t) => t.text.write_html(out),
            RichText::Hashtag(t) => t.text.write_html(out),
            RichText::Cashtag(t) => t.text.write_html(out),
            RichText::BotCommand(t) => t.text.write_html(out),
            RichText::DateTime(t) => {
                out.push_str(&format!(
                    "<tg-time unix=\"{}\" format=\"{}\">",
                    t.unix_time,
                    escape_html_attr(&t.date_time_format)
                ));
                t.text.write_html(out);
                out.push_str("</tg-time>");
            }
            RichText::Anchor(t) => {
                out.push_str("<a name=\"");
                out.push_str(&escape_html_attr(&t.name));
                out.push_str("\"></a>");
            }
            RichText::AnchorLink(t) => {
                out.push_str("<a href=\"#");
                out.push_str(&escape_html_attr(&t.anchor_name));
                out.push_str("\">");
                t.text.write_html(out);
                out.push_str("</a>");
            }
            RichText::Reference(t) => {
                out.push_str("<tg-reference name=\"");
                out.push_str(&escape_html_attr(&t.name));
                out.push_str("\">");
                t.text.write_html(out);
                out.push_str("</tg-reference>");
            }
            RichText::ReferenceLink(t) => {
                out.push_str("<a href=\"#");
                out.push_str(&escape_html_attr(&t.reference_name));
                out.push_str("\">");
                t.text.write_html(out);
                out.push_str("</a>");
            }
            RichText::Other { raw, .. } => out.push_str(&escape_html(&other_plain_text(raw))),
        }
    }

    fn write_markdown(&self, out: &mut String) {
        match self {
            RichText::Plain(s) => out.push_str(&escape_markdown(s)),
            RichText::Array(items) => items.iter().for_each(|i| i.write_markdown(out)),
            RichText::Bold(t) => wrap_markdown(out, "**", &t.text),
            RichText::Italic(t) => wrap_markdown(out, "*", &t.text),
            RichText::Strikethrough(t) => wrap_markdown(out, "~~", &t.text),
            RichText::Marked(t) => wrap_markdown(out, "==", &t.text),
            RichText::Spoiler(t) => wrap_markdown(out, "||", &t.text),
            RichText::Code(t) => wrap_markdown(out, "`", &t.text),
            // Rich Markdown has no syntax of its own for these, so it falls
            // back to the HTML tags it also accepts.
            RichText::Underline(t) => wrap_markdown_html(out, "u", &t.text),
            RichText::Subscript(t) => wrap_markdown_html(out, "sub", &t.text),
            RichText::Superscript(t) => wrap_markdown_html(out, "sup", &t.text),
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
                out.push_str("](");
                out.push_str(CUSTOM_EMOJI_SCHEME);
                out.push_str(&t.custom_emoji_id);
                out.push(')');
            }
            // Rich Markdown documents these three explicitly: `[inline
            // e-mail](mailto:user@example.com)`, `[inline phone
            // number](tel:+123456789)`, `[inline mention of a
            // user](tg://user?id=123456789)`.
            RichText::TextMention(t) => {
                out.push('[');
                t.text.write_markdown(out);
                out.push_str(&format!("](tg://user?id={})", t.user.id.0));
            }
            RichText::EmailAddress(t) => {
                out.push('[');
                t.text.write_markdown(out);
                out.push_str("](mailto:");
                out.push_str(&t.email_address);
                out.push(')');
            }
            RichText::PhoneNumber(t) => {
                out.push('[');
                t.text.write_markdown(out);
                out.push_str("](tel:");
                out.push_str(&t.phone_number);
                out.push(')');
            }
            // Same reasoning as in `write_html`: no dedicated syntax, so only
            // the already-formatted text is rendered.
            RichText::BankCardNumber(t) => t.text.write_markdown(out),
            RichText::Mention(t) => t.text.write_markdown(out),
            RichText::Hashtag(t) => t.text.write_markdown(out),
            RichText::Cashtag(t) => t.text.write_markdown(out),
            RichText::BotCommand(t) => t.text.write_markdown(out),
            // `![fallback](tg://time?unix=...&format=...)`, matching the
            // documented example exactly.
            RichText::DateTime(t) => {
                out.push_str("![");
                t.text.write_markdown(out);
                out.push_str(&format!(
                    "](tg://time?unix={}&format={})",
                    t.unix_time, t.date_time_format
                ));
            }
            // No Markdown syntax for anchors — same HTML fallback as
            // `to_html`.
            RichText::Anchor(t) => {
                out.push_str("<a name=\"");
                out.push_str(&escape_html_attr(&t.name));
                out.push_str("\"></a>");
            }
            RichText::AnchorLink(t) => {
                out.push_str("<a href=\"#");
                out.push_str(&escape_html_attr(&t.anchor_name));
                out.push_str("\">");
                t.text.write_html(out);
                out.push_str("</a>");
            }
            // `[^name]: text` / `[^name]`: the GitHub-Flavored-Markdown
            // footnote syntax rich Markdown is explicitly compatible with.
            RichText::Reference(t) => {
                out.push_str(&format!("[^{}]: ", t.name));
                t.text.write_markdown(out);
            }
            RichText::ReferenceLink(t) => out.push_str(&format!("[^{}]", t.reference_name)),
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
            RichText::TextMention(t) => t.text.write_plain(out),
            RichText::EmailAddress(t) => t.text.write_plain(out),
            RichText::PhoneNumber(t) => t.text.write_plain(out),
            RichText::BankCardNumber(t) => t.text.write_plain(out),
            RichText::Mention(t) => t.text.write_plain(out),
            RichText::Hashtag(t) => t.text.write_plain(out),
            RichText::Cashtag(t) => t.text.write_plain(out),
            RichText::BotCommand(t) => t.text.write_plain(out),
            RichText::AnchorLink(t) => t.text.write_plain(out),
            RichText::Reference(t) => t.text.write_plain(out),
            RichText::ReferenceLink(t) => t.text.write_plain(out),
            RichText::MathematicalExpression(t) => out.push_str(&t.expression),
            RichText::CustomEmoji(t) => out.push_str(&t.alternative_text),
            RichText::DateTime(t) => t.text.write_plain(out),
            RichText::Anchor(_) => {}
            RichText::Other { raw, .. } => out.push_str(&other_plain_text(raw)),
        }
    }
}

/// Renders a media block's caption for `to_html` — the media itself is
/// never rendered (see [`RichMessage::to_html`] for why), so this is the
/// entire output for [`RichBlock::Photo`], [`Animation`], [`Audio`],
/// [`Video`] and [`VoiceNote`].
///
/// [`Animation`]: RichBlock::Animation
/// [`Audio`]: RichBlock::Audio
/// [`Video`]: RichBlock::Video
/// [`VoiceNote`]: RichBlock::VoiceNote
fn write_media_caption_html(out: &mut String, caption: Option<&RichBlockCaption>) {
    if let Some(caption) = caption {
        out.push_str("<figure><figcaption>");
        caption.text.write_html(out);
        if let Some(credit) = &caption.credit {
            out.push_str("<cite>");
            credit.write_html(out);
            out.push_str("</cite>");
        }
        out.push_str("</figcaption></figure>\n");
    }
}

/// The Markdown counterpart of [`write_media_caption_html`].
fn write_media_caption_markdown(out: &mut String, caption: Option<&RichBlockCaption>) {
    if let Some(caption) = caption {
        caption.text.write_markdown(out);
        out.push_str("\n\n");
    }
}

/// The plain-text counterpart of [`write_media_caption_html`].
fn write_media_caption_plain(out: &mut String, caption: Option<&RichBlockCaption>) {
    if let Some(caption) = caption {
        caption.text.write_plain(out);
        out.push_str("\n\n");
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

/// Wraps `text` in an inline HTML tag — rich Markdown accepts HTML for the
/// formatting it has no syntax for.
fn wrap_markdown_html(out: &mut String, tag: &str, text: &RichText) {
    out.push('<');
    out.push_str(tag);
    out.push('>');
    text.write_markdown(out);
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

/// Formatted date and time.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextdatetime).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextDateTime {
    pub text: Box<RichText>,
    pub unix_time: i64,
    /// See [date-time entity formatting] for the accepted values.
    ///
    /// [date-time entity formatting]: https://core.telegram.org/bots/api#date-time-entity-formatting
    pub date_time_format: String,
}

/// A mention of a Telegram user by their identifier.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexttextmention).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextTextMention {
    pub text: Box<RichText>,
    pub user: User,
}

/// A text with an e-mail address.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextemailaddress).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
    pub text: Box<RichText>,
    pub email_address: String,
}

/// A text with a phone number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextphonenumber).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
    pub text: Box<RichText>,
    pub phone_number: String,
}

/// A text with a bank card number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbankcardnumber).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextBankCardNumber {
    pub text: Box<RichText>,
    pub bank_card_number: String,
}

/// A mention by a username.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmention).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextMention {
    pub text: Box<RichText>,
    pub username: String,
}

/// A hashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexthashtag).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextHashtag {
    pub text: Box<RichText>,
    pub hashtag: String,
}

/// A cashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcashtag).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextCashtag {
    pub text: Box<RichText>,
    pub cashtag: String,
}

/// A bot command.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbotcommand).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextBotCommand {
    pub text: Box<RichText>,
    pub bot_command: String,
}

/// An anchor. Unlike the other `RichText` variants, this carries no `text` —
/// it marks a point in the message, corresponding to the HTML tag
/// `<a name="...">`.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchor).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextAnchor {
    pub name: String,
}

/// A link to an anchor.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchorlink).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextAnchorLink {
    pub text: Box<RichText>,
    pub anchor_name: String,
}

/// A reference (footnote).
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreference).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextReference {
    pub text: Box<RichText>,
    pub name: String,
}

/// A link to a [`RichTextReference`].
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreferencelink).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichTextReferenceLink {
    pub text: Box<RichText>,
    pub reference_name: String,
}

/// Media embedded in an outgoing rich message.
///
/// Referenced from the [`html`] or [`markdown`] content by a
/// `tg://photo?id=`, `tg://video?id=` or `tg://audio?id=` link.
///
/// [`html`]: InputRichMessage::html
/// [`markdown`]: InputRichMessage::markdown
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessagemedia).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media used in the link. 1-64 characters,
    /// only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    pub id: String,

    /// The media to be sent. Everything except the media itself and its
    /// properties is ignored.
    pub media: InputRichMedia,
}

impl InputRichMessageMedia {
    pub fn new<I>(id: I, media: impl Into<InputRichMedia>) -> Self
    where
        I: Into<String>,
    {
        Self { id: id.into(), media: media.into() }
    }
}

/// Serializes an `InputMedia*` with the `type` tag the Bot API requires
/// inside the object.
///
/// The crate's `InputMedia*` structs carry no tag of their own — it normally
/// comes from the [`InputMedia`](crate::types::InputMedia) enum wrapping
/// them, which isn't used when a media object is a field in its own right.
fn serialize_tagged_media<T, S>(value: &T, tag: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: serde::Serializer,
{
    let mut value = serde_json::to_value(value).map_err(serde::ser::Error::custom)?;
    if let Some(object) = value.as_object_mut() {
        object.insert("type".to_owned(), serde_json::Value::String(tag.to_owned()));
    }
    value.serialize(serializer)
}

macro_rules! tagged_media_serializers {
    ($($name:ident => $ty:ty, $tag:literal;)*) => {$(
        fn $name<S>(value: &$ty, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_tagged_media(value, $tag, serializer)
        }
    )*};
}

tagged_media_serializers! {
    serialize_animation => InputMediaAnimation, "animation";
    serialize_audio => InputMediaAudio, "audio";
    serialize_photo => InputMediaPhoto, "photo";
    serialize_video => InputMediaVideo, "video";
    serialize_voice_note => InputMediaVoiceNote, "voice_note";
}

/// The kinds of media that can be embedded in an outgoing rich message.
#[derive(Clone, Debug, Serialize, derive_more::From)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichMedia {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
    VoiceNote(InputMediaVoiceNote),
}

/// Describes a rich message to be sent.
///
/// Exactly one of `html`, `markdown` or `blocks` carries the content — use
/// the constructor of the same name to build one. Telegram parses the
/// `html`/`markdown` forms itself; see [rich message formatting options] for
/// the accepted syntax.
///
/// [rich message formatting options]: https://core.telegram.org/bots/api#rich-message-formatting-options
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessage {
    /// Content of the rich message described as a list of blocks.
    pub blocks: Option<Vec<InputRichBlock>>,

    /// Content of the rich message described using HTML formatting. Use
    /// [`media`](Self::media) to specify the media used in the message.
    pub html: Option<String>,

    /// Content of the rich message described using Markdown formatting. Use
    /// [`media`](Self::media) to specify the media used in the message.
    pub markdown: Option<String>,

    /// Media specified in the `markdown` or `html` fields using
    /// `tg://photo?id=`, `tg://video?id=` and `tg://audio?id=` links.
    pub media: Option<Vec<InputRichMessageMedia>>,

    /// Pass `true` if the rich message must be shown right-to-left.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_rtl: bool,

    /// Pass `true` to skip automatic detection of entities (e.g. URLs, email
    /// addresses, username mentions, hashtags, cashtags, bot commands or
    /// phone numbers) in the text.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub skip_entity_detection: bool,
}

impl InputRichMessage {
    fn empty() -> Self {
        Self {
            blocks: None,
            html: None,
            markdown: None,
            media: None,
            is_rtl: false,
            skip_entity_detection: false,
        }
    }

    /// Content described using [rich HTML formatting], parsed by Telegram.
    ///
    /// [rich HTML formatting]: https://core.telegram.org/bots/api#rich-html-style
    pub fn html<S>(html: S) -> Self
    where
        S: Into<String>,
    {
        Self { html: Some(html.into()), ..Self::empty() }
    }

    /// Content described using [rich Markdown formatting], parsed by
    /// Telegram.
    ///
    /// [rich Markdown formatting]: https://core.telegram.org/bots/api#rich-markdown-style
    pub fn markdown<S>(markdown: S) -> Self
    where
        S: Into<String>,
    {
        Self { markdown: Some(markdown.into()), ..Self::empty() }
    }

    /// Content described as an explicit list of blocks.
    pub fn blocks<B>(blocks: B) -> Self
    where
        B: IntoIterator<Item = InputRichBlock>,
    {
        Self { blocks: Some(blocks.into_iter().collect()), ..Self::empty() }
    }

    /// Attaches the media referenced from `html`/`markdown` content by
    /// `tg://photo?id=`, `tg://video?id=` and `tg://audio?id=` links.
    #[must_use]
    pub fn media<M>(mut self, media: M) -> Self
    where
        M: IntoIterator<Item = InputRichMessageMedia>,
    {
        self.media = Some(media.into_iter().collect());
        self
    }

    #[must_use]
    pub const fn is_rtl(mut self, val: bool) -> Self {
        self.is_rtl = val;
        self
    }

    #[must_use]
    pub const fn skip_entity_detection(mut self, val: bool) -> Self {
        self.skip_entity_detection = val;
        self
    }
}

impl From<Vec<InputRichBlock>> for InputRichMessage {
    fn from(blocks: Vec<InputRichBlock>) -> Self {
        Self::blocks(blocks)
    }
}

/// A block of an outgoing rich message.
///
/// The receiving counterpart is [`RichBlock`].
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblock).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichBlock {
    /// A text paragraph, corresponding to the HTML tag `<p>`.
    Paragraph { text: RichText },

    /// A section heading, corresponding to the HTML tags `<h1>`-`<h6>`.
    #[serde(rename = "heading")]
    SectionHeading { text: RichText, size: u8 },

    /// A preformatted text block, corresponding to the nested HTML tags
    /// `<pre>` and `<code>`.
    #[serde(rename = "pre")]
    Preformatted { text: RichText, language: Option<String> },

    /// A footer, corresponding to the HTML tag `<footer>`.
    Footer { text: RichText },

    /// A divider, corresponding to the HTML tag `<hr/>`.
    Divider,

    /// A block with a mathematical expression in LaTeX format, corresponding
    /// to the custom HTML tag `<tg-math-block>`.
    MathematicalExpression { expression: String },

    /// A block with an anchor, corresponding to the HTML tag `<a>` with the
    /// attribute `name`.
    Anchor { name: String },

    /// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>`.
    List { items: Vec<InputRichBlockListItem> },

    /// A block quotation, corresponding to the HTML tag `<blockquote>`.
    #[serde(rename = "blockquote")]
    BlockQuotation { blocks: Vec<InputRichBlock>, credit: Option<RichText> },

    /// A quotation with centered text, loosely corresponding to the HTML tag
    /// `<aside>`.
    #[serde(rename = "pullquote")]
    PullQuotation { text: RichText, credit: Option<RichText> },

    /// A collage, corresponding to the custom HTML tag `<tg-collage>`.
    Collage { blocks: Vec<InputRichBlock>, caption: Option<RichBlockCaption> },

    /// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
    Slideshow { blocks: Vec<InputRichBlock>, caption: Option<RichBlockCaption> },

    /// A table, corresponding to the HTML tag `<table>`.
    Table {
        cells: Vec<Vec<RichBlockTableCell>>,
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        is_bordered: bool,
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        is_striped: bool,
        caption: Option<RichText>,
    },

    /// An expandable block for details disclosure, corresponding to the HTML
    /// tag `<details>`.
    Details {
        summary: RichText,
        blocks: Vec<InputRichBlock>,
        /// If `true`, the block is expanded by default.
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        is_open: bool,
    },

    /// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
    ///
    /// The map's `width` and `height` must not exceed 10000 in total.
    Map { location: Location, zoom: u8, width: u32, height: u32, caption: Option<RichBlockCaption> },

    /// A block with an animation, corresponding to the HTML tag `<video>`.
    Animation {
        #[serde(serialize_with = "serialize_animation")]
        animation: InputMediaAnimation,
        caption: Option<RichBlockCaption>,
    },

    /// A block with a music file, corresponding to the HTML tag `<audio>`.
    Audio {
        #[serde(serialize_with = "serialize_audio")]
        audio: InputMediaAudio,
        caption: Option<RichBlockCaption>,
    },

    /// A block with a photo, corresponding to the HTML tag `<img>`.
    Photo {
        #[serde(serialize_with = "serialize_photo")]
        photo: InputMediaPhoto,
        caption: Option<RichBlockCaption>,
    },

    /// A block with a video, corresponding to the HTML tag `<video>`.
    Video {
        #[serde(serialize_with = "serialize_video")]
        video: InputMediaVideo,
        caption: Option<RichBlockCaption>,
    },

    /// A block with a voice note, corresponding to the HTML tag `<audio>`.
    VoiceNote {
        #[serde(serialize_with = "serialize_voice_note")]
        voice_note: InputMediaVoiceNote,
        caption: Option<RichBlockCaption>,
    },

    /// A block with a "Thinking..." placeholder, corresponding to the custom
    /// HTML tag `<tg-thinking>`. May be used only in `sendRichMessageDraft`.
    Thinking { text: RichText },
}

/// An item of an outgoing [`InputRichBlock::List`].
///
/// Unlike [`RichBlockListItem`] it carries no `label` — Telegram renders one
/// from the list's position and [`kind`](Self::kind).
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocklistitem).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockListItem {
    /// Content of the item.
    pub blocks: Vec<InputRichBlock>,

    /// `true` if the item is rendered with a checkbox.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub has_checkbox: bool,

    /// `true` if the item's checkbox is checked.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_checked: bool,

    /// The item's number, for ordered lists.
    pub value: Option<i64>,

    /// For ordered lists, the type of the item label; one of `"a"`, `"A"`,
    /// `"i"`, `"I"` or `"1"`.
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

impl InputRichBlockListItem {
    pub fn new<B>(blocks: B) -> Self
    where
        B: IntoIterator<Item = InputRichBlock>,
    {
        Self {
            blocks: blocks.into_iter().collect(),
            has_checkbox: false,
            is_checked: false,
            value: None,
            kind: None,
        }
    }
}

/// Collecting the files that have to be uploaded with a rich message.
///
/// Unlike [`InputMedia::files`] these are visitors rather than iterators:
/// blocks nest into each other (lists, collages, quotations, ...), and a
/// recursive iterator would have to be boxed at every level.
///
/// [`InputMedia::files`]: crate::types::InputMedia
impl InputRichMessage {
    pub(crate) fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        if let Some(blocks) = &self.blocks {
            blocks.iter().for_each(|block| block.copy_files(into));
        }

        if let Some(media) = &self.media {
            media.iter().for_each(|media| media.media.copy_files(into));
        }
    }

    pub(crate) fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        if let Some(blocks) = &mut self.blocks {
            blocks.iter_mut().for_each(|block| block.move_files(into));
        }

        if let Some(media) = &mut self.media {
            media.iter_mut().for_each(|media| media.media.move_files(into));
        }
    }
}

impl InputRichMedia {
    pub(crate) fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Photo(InputMediaPhoto { media, .. })
            | Self::VoiceNote(InputMediaVoiceNote { media, .. }) => media.copy_into(into),
            Self::Animation(InputMediaAnimation { media, thumbnail, .. })
            | Self::Audio(InputMediaAudio { media, thumbnail, .. }) => {
                media.copy_into(into);
                thumbnail.copy_into(into);
            }
            Self::Video(InputMediaVideo { media, thumbnail, cover, .. }) => {
                media.copy_into(into);
                thumbnail.copy_into(into);
                cover.copy_into(into);
            }
        }
    }

    pub(crate) fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Photo(InputMediaPhoto { media, .. })
            | Self::VoiceNote(InputMediaVoiceNote { media, .. }) => media.move_into(into),
            Self::Animation(InputMediaAnimation { media, thumbnail, .. })
            | Self::Audio(InputMediaAudio { media, thumbnail, .. }) => {
                media.move_into(into);
                thumbnail.move_into(into);
            }
            Self::Video(InputMediaVideo { media, thumbnail, cover, .. }) => {
                media.move_into(into);
                thumbnail.move_into(into);
                cover.move_into(into);
            }
        }
    }
}

impl InputRichBlock {
    // NB. The file-less variants are listed one by one instead of using a `_`
    // arm, so that a newly added block with media in it fails to compile here
    // instead of silently losing its file.
    pub(crate) fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation { animation: InputMediaAnimation { media, thumbnail, .. }, .. }
            | Self::Audio { audio: InputMediaAudio { media, thumbnail, .. }, .. } => {
                media.copy_into(into);
                thumbnail.copy_into(into);
            }
            Self::Video { video: InputMediaVideo { media, thumbnail, cover, .. }, .. } => {
                media.copy_into(into);
                thumbnail.copy_into(into);
                cover.copy_into(into);
            }
            Self::Photo { photo: InputMediaPhoto { media, .. }, .. }
            | Self::VoiceNote { voice_note: InputMediaVoiceNote { media, .. }, .. } => {
                media.copy_into(into)
            }
            Self::List { items } => {
                items.iter().flat_map(|item| &item.blocks).for_each(|block| block.copy_files(into))
            }
            Self::BlockQuotation { blocks, .. }
            | Self::Collage { blocks, .. }
            | Self::Slideshow { blocks, .. }
            | Self::Details { blocks, .. } => {
                blocks.iter().for_each(|block| block.copy_files(into));
            }
            Self::Paragraph { .. }
            | Self::SectionHeading { .. }
            | Self::Preformatted { .. }
            | Self::Footer { .. }
            | Self::Divider
            | Self::MathematicalExpression { .. }
            | Self::Anchor { .. }
            | Self::PullQuotation { .. }
            | Self::Table { .. }
            | Self::Map { .. }
            | Self::Thinking { .. } => {}
        }
    }

    pub(crate) fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::Animation { animation: InputMediaAnimation { media, thumbnail, .. }, .. }
            | Self::Audio { audio: InputMediaAudio { media, thumbnail, .. }, .. } => {
                media.move_into(into);
                thumbnail.move_into(into);
            }
            Self::Video { video: InputMediaVideo { media, thumbnail, cover, .. }, .. } => {
                media.move_into(into);
                thumbnail.move_into(into);
                cover.move_into(into);
            }
            Self::Photo { photo: InputMediaPhoto { media, .. }, .. }
            | Self::VoiceNote { voice_note: InputMediaVoiceNote { media, .. }, .. } => {
                media.move_into(into)
            }
            Self::List { items } => items
                .iter_mut()
                .flat_map(|item| &mut item.blocks)
                .for_each(|block| block.move_files(into)),
            Self::BlockQuotation { blocks, .. }
            | Self::Collage { blocks, .. }
            | Self::Slideshow { blocks, .. }
            | Self::Details { blocks, .. } => {
                blocks.iter_mut().for_each(|block| block.move_files(into));
            }
            Self::Paragraph { .. }
            | Self::SectionHeading { .. }
            | Self::Preformatted { .. }
            | Self::Footer { .. }
            | Self::Divider
            | Self::MathematicalExpression { .. }
            | Self::Anchor { .. }
            | Self::PullQuotation { .. }
            | Self::Table { .. }
            | Self::Map { .. }
            | Self::Thinking { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FileId, FileMeta, FileUniqueId, InputFile, Seconds, UserId};

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
        assert!(matches!(&table.cells[0][0].text, Some(RichText::Bold(_))));
        assert!(matches!(&table.cells[1][0].text, Some(RichText::Plain(s)) if s == "plain cell"));

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
                    "cells": [[{ "text": "1", "is_header": true, "align": "left", "valign": "top" }]],
                    "caption": "test (table",
                    "is_bordered": true,
                    "is_striped": true
                },
                { "type": "mathematical_expression", "expression": "test (block expression)" },
                { "type": "anchor", "name": "test-anchor" },
                {
                    "type": "map",
                    "location": { "latitude": 41.9, "longitude": 12.5 },
                    "zoom": 14,
                    "width": 400,
                    "height": 300
                },
                {
                    "type": "paragraph",
                    "text": [
                        {
                            "type": "text_mention",
                            "text": "test (mention)",
                            "user": { "id": 123456789, "is_bot": false, "first_name": "Test" }
                        },
                        { "type": "email_address", "text": "test@example.com", "email_address": "test@example.com" },
                        { "type": "phone_number", "text": "+123456789", "phone_number": "+123456789" },
                        { "type": "mention", "text": "@username", "username": "username" },
                        { "type": "hashtag", "text": "#hashtag", "hashtag": "hashtag" },
                        { "type": "cashtag", "text": "$USD", "cashtag": "USD" },
                        { "type": "bot_command", "text": "/command", "bot_command": "command" },
                        { "type": "bank_card_number", "text": "4242 4242 4242 4242", "bank_card_number": "4242424242424242" },
                        { "type": "date_time", "text": "tomorrow", "unix_time": 1647531900, "date_time_format": "wDT" },
                        { "type": "anchor_link", "text": "test (anchor link)", "anchor_name": "test-anchor" },
                        { "type": "reference", "text": "test (reference)", "name": "note-1" },
                        { "type": "reference_link", "text": "test (reference link)", "reference_name": "note-1" }
                    ]
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
                { "type": "thinking", "text": "..." },
                // A block type teloxide doesn't know about yet must not break parsing.
                { "type": "some_future_block", "surprise": true }
            ]
        });

        let rich: RichMessage = serde_json::from_value(json).expect("should deserialize");
        assert_eq!(rich.blocks.len(), 20);
        assert!(matches!(&rich.blocks[11], RichBlock::MathematicalExpression(_)));
        assert!(matches!(&rich.blocks[12], RichBlock::Anchor(b) if b.name == "test-anchor"));
        assert!(matches!(&rich.blocks[13], RichBlock::Map(b) if b.zoom == 14));
        let RichBlock::Paragraph(new_texts) = &rich.blocks[14] else {
            panic!("expected a paragraph, got {:?}", rich.blocks[14])
        };
        let RichText::Array(nodes) = &new_texts.text else { panic!("expected an array") };
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::TextMention(t) if t.user.id.0 == 123456789)));
        assert!(nodes.iter().any(
            |n| matches!(n, RichText::EmailAddress(t) if t.email_address == "test@example.com")
        ));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::PhoneNumber(t) if t.phone_number == "+123456789")));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::Mention(t) if t.username == "username")));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Hashtag(t) if t.hashtag == "hashtag")));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Cashtag(t) if t.cashtag == "USD")));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::BotCommand(t) if t.bot_command == "command")));
        assert!(nodes.iter().any(|n| matches!(n, RichText::BankCardNumber(_))));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::DateTime(t) if t.unix_time == 1647531900)));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::AnchorLink(t) if t.anchor_name == "test-anchor")));
        assert!(nodes.iter().any(|n| matches!(n, RichText::Reference(t) if t.name == "note-1")));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, RichText::ReferenceLink(t) if t.reference_name == "note-1")));

        assert!(
            matches!(&rich.blocks[18], RichBlock::Thinking(t) if matches!(&t.text, RichText::Plain(s) if s == "..."))
        );
        assert!(
            matches!(rich.blocks.last(), Some(RichBlock::Other { kind, .. }) if kind == "some_future_block")
        );

        let html = rich.to_html();
        assert!(html.contains("<b>test (bold)</b>"));
        assert!(html.contains("<a href=\"https://example.com/\">test (link)</a>"));
        assert!(html.contains("<tg-math-block>test (block expression)</tg-math-block>"));
        assert!(html.contains("<a name=\"test-anchor\"></a>"));
        assert!(html.contains("<tg-map lat=\"41.9\" long=\"12.5\" zoom=\"14\"/>"));
        assert!(html.contains("<a href=\"tg://user?id=123456789\">"));
        assert!(html.contains("<a href=\"mailto:test@example.com\">"));
        assert!(html.contains("<a href=\"tel:+123456789\">"));
        assert!(html.contains("<tg-time unix=\"1647531900\" format=\"wDT\">"));
        assert!(html.contains("<a href=\"#test-anchor\">test (anchor link)</a>"));
        assert!(html.contains("<tg-reference name=\"note-1\">test (reference)</tg-reference>"));

        let plain = rich.plain_text();
        assert!(plain.contains("test (plain)"));
        assert!(plain.contains("test (expression)"));
        assert!(plain.contains("test (block expression)"));
    }

    #[test]
    fn renders_blockquote_credit() {
        let msg = RichMessage::new(vec![RichBlock::BlockQuotation(RichBlockBlockQuotation {
            blocks: vec![RichBlock::paragraph("цитата")],
            credit: Some("тест автор".into()),
        })]);

        assert!(msg.to_html().contains("<cite>тест автор</cite>"));
        assert!(msg.to_markdown().contains("> <cite>тест автор</cite>"));
    }

    /// Every tag and attribute the HTML renderer emits must appear in the
    /// [rich HTML style] list — teloxide-specific markup would simply be
    /// rejected by Telegram.
    ///
    /// [rich HTML style]: https://core.telegram.org/bots/api#rich-html-style
    #[test]
    fn rendered_html_uses_only_documented_markup() {
        let msg = RichMessage::new(vec![
            RichBlock::heading("h", 2),
            RichBlock::paragraph(RichText::Array(vec![
                RichText::Bold(RichTextSimple::new("b")),
                RichText::Italic(RichTextSimple::new("i")),
                RichText::Underline(RichTextSimple::new("u")),
                RichText::Strikethrough(RichTextSimple::new("s")),
                RichText::Marked(RichTextSimple::new("m")),
                RichText::Subscript(RichTextSimple::new("sub")),
                RichText::Superscript(RichTextSimple::new("sup")),
                RichText::Spoiler(RichTextSimple::new("sp")),
                RichText::Code(RichTextSimple::new("c")),
            ])),
            RichBlock::Preformatted(RichBlockPreformatted {
                text: "code".into(),
                language: Some("python".to_owned()),
            }),
            RichBlock::Footer(RichBlockFooter { text: "f".into() }),
            RichBlock::divider(),
            RichBlock::PullQuotation(RichBlockPullQuotation {
                text: "q".into(),
                credit: Some("author".into()),
            }),
            RichBlock::List(RichBlockList {
                items: vec![RichBlockListItem {
                    label: "•".into(),
                    blocks: vec![RichBlock::paragraph("i")],
                    kind: None,
                    value: None,
                    has_checkbox: true,
                    is_checked: true,
                }],
            }),
            RichBlock::MathematicalExpression(RichBlockMathematicalExpression {
                expression: "x".into(),
            }),
            RichBlock::Anchor(RichBlockAnchor { name: "a".into() }),
            RichBlock::Collage(RichBlockCollage {
                blocks: vec![RichBlock::Photo(RichBlockPhoto {
                    photo: vec![],
                    has_spoiler: false,
                    caption: None,
                })],
                caption: Some(RichBlockCaption { text: "cap".into(), credit: Some("cr".into()) }),
            }),
            RichBlock::Map(RichBlockMap {
                location: Location {
                    longitude: 1.0,
                    latitude: 2.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None,
                },
                zoom: 10,
                width: 100,
                height: 100,
                caption: Some(RichBlockCaption { text: "map".into(), credit: None }),
            }),
            RichBlock::Thinking(RichBlockThinking { text: "t".into() }),
            RichBlock::paragraph(RichText::Array(vec![
                RichText::TextMention(RichTextTextMention {
                    text: Box::new("u".into()),
                    user: User {
                        id: crate::types::UserId(1),
                        is_bot: false,
                        first_name: "T".into(),
                        last_name: None,
                        username: None,
                        language_code: None,
                        is_premium: false,
                        added_to_attachment_menu: false,
                    },
                }),
                RichText::EmailAddress(RichTextEmailAddress {
                    text: Box::new("e".into()),
                    email_address: "e@example.com".into(),
                }),
                RichText::PhoneNumber(RichTextPhoneNumber {
                    text: Box::new("p".into()),
                    phone_number: "+1".into(),
                }),
                RichText::DateTime(RichTextDateTime {
                    text: Box::new("d".into()),
                    unix_time: 0,
                    date_time_format: "wDT".into(),
                }),
                RichText::Anchor(RichTextAnchor { name: "a".into() }),
                RichText::AnchorLink(RichTextAnchorLink {
                    text: Box::new("al".into()),
                    anchor_name: "a".into(),
                }),
                RichText::Reference(RichTextReference {
                    text: Box::new("r".into()),
                    name: "n".into(),
                }),
                RichText::ReferenceLink(RichTextReferenceLink {
                    text: Box::new("rl".into()),
                    reference_name: "n".into(),
                }),
            ])),
        ]);

        let html = msg.to_html();
        let documented = [
            "b",
            "strong",
            "i",
            "em",
            "u",
            "ins",
            "s",
            "strike",
            "del",
            "code",
            "mark",
            "sub",
            "sup",
            "tg-spoiler",
            "a",
            "tg-reference",
            "tg-emoji",
            "img",
            "tg-time",
            "tg-math",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "p",
            "pre",
            "footer",
            "hr",
            "ul",
            "ol",
            "li",
            "input",
            "blockquote",
            "cite",
            "aside",
            "video",
            "audio",
            "figure",
            "figcaption",
            "tg-map",
            "tg-collage",
            "tg-slideshow",
            "table",
            "caption",
            "tr",
            "th",
            "td",
            "details",
            "summary",
            "tg-math-block",
            "tg-thinking",
        ];

        for tag in html.split('<').skip(1) {
            let name = tag
                .trim_start_matches('/')
                .split([' ', '>', '/'])
                .next()
                .expect("a tag name follows `<`");
            assert!(documented.contains(&name), "undocumented tag `<{name}>` in {html}");
        }

        // Attributes teloxide is allowed to emit, per the same list.
        let documented_attrs = [
            "href",
            "name",
            "emoji-id",
            "src",
            "class",
            "type",
            "checked",
            "value",
            "start",
            "reversed",
            "colspan",
            "rowspan",
            "align",
            "valign",
            "bordered",
            "striped",
            "open",
            "lat",
            "long",
            "zoom",
            "unix",
            "format",
            "tg-spoiler",
        ];

        for tag in html.split('<').skip(1) {
            let Some((head, _)) = tag.split_once('>') else { continue };
            for attr in head.split_whitespace().skip(1) {
                let name = attr.split('=').next().unwrap().trim_end_matches('/');
                if name.is_empty() {
                    continue;
                }
                assert!(
                    documented_attrs.contains(&name),
                    "undocumented attribute `{name}` in `<{head}>`"
                );
            }
        }
    }

    #[test]
    fn renders_custom_emoji_with_its_id() {
        let msg = RichMessage::new(vec![RichBlock::paragraph(RichText::CustomEmoji(
            RichTextCustomEmoji {
                custom_emoji_id: "5436040291507247633".to_owned(),
                alternative_text: "🎉".to_owned(),
            },
        ))]);

        assert!(msg.to_html().contains("<tg-emoji emoji-id=\"5436040291507247633\">🎉</tg-emoji>"));
        assert!(msg.to_markdown().contains("![🎉](tg://emoji?id=5436040291507247633)"));
        assert_eq!(msg.plain_text(), "🎉");
    }

    fn photo(width: u32, height: u32) -> PhotoSize {
        PhotoSize {
            file: FileMeta {
                id: FileId("AgACAgIAAx".to_owned()),
                unique_id: FileUniqueId(String::new()),
                size: 0,
            },
            width,
            height,
        }
    }

    #[test]
    fn renders_photo_block_with_its_caption() {
        let msg = RichMessage::new(vec![RichBlock::Photo(RichBlockPhoto {
            photo: vec![photo(320, 240), photo(800, 600)],
            has_spoiler: false,
            caption: Some(RichBlockCaption {
                text: RichText::Array(vec![
                    RichText::from("a "),
                    RichText::Bold(RichTextSimple::new("cat")),
                ]),
                credit: Some("Photographer".into()),
            }),
        })]);

        // Only the caption is rendered — the photo itself has no
        // representation in either dialect.
        assert_eq!(
            msg.to_html(),
            "<figure><figcaption>a <b>cat</b><cite>Photographer</cite></figcaption></figure>"
        );
        assert_eq!(msg.to_markdown(), "a **cat**");
        assert_eq!(msg.plain_text(), "a cat");
    }

    #[test]
    fn photo_without_caption_renders_nothing() {
        let msg = RichMessage::new(vec![RichBlock::Photo(RichBlockPhoto {
            photo: vec![photo(800, 600)],
            has_spoiler: false,
            caption: None,
        })]);

        assert_eq!(msg.to_html(), "");
        assert_eq!(msg.to_markdown(), "");
        assert_eq!(msg.plain_text(), "");
    }

    #[test]
    fn table_cell_without_text_renders_empty() {
        let msg = RichMessage::new(vec![RichBlock::Table(RichBlockTable {
            cells: vec![vec![RichBlockTableCell::default()]],
            caption: None,
            is_bordered: true,
            is_striped: false,
        })]);

        // `align`/`valign` are required by the API, so they are always
        // rendered.
        assert!(msg.to_html().contains("<td align=\"left\" valign=\"top\"></td>"));
    }

    /// `RichBlock`/`RichText` deserialization falls back to `Other` on any
    /// error, which is what keeps unknown future kinds from breaking a whole
    /// `Message` — but it also means a *known* kind that fails to parse
    /// degrades silently. This pins that every kind teloxide models actually
    /// survives a round trip.
    #[test]
    fn every_known_kind_survives_a_round_trip() {
        fn media_file() -> FileMeta {
            FileMeta { id: FileId("f".into()), unique_id: FileUniqueId("u".into()), size: 1 }
        }
        fn caption() -> Option<RichBlockCaption> {
            Some(RichBlockCaption { text: "c".into(), credit: Some("cr".into()) })
        }

        let blocks = vec![
            RichBlock::paragraph("x"),
            RichBlock::heading("x", 1),
            RichBlock::Preformatted(RichBlockPreformatted { text: "x".into(), language: None }),
            RichBlock::Footer(RichBlockFooter { text: "x".into() }),
            RichBlock::divider(),
            RichBlock::MathematicalExpression(RichBlockMathematicalExpression {
                expression: "e".into(),
            }),
            RichBlock::Anchor(RichBlockAnchor { name: "n".into() }),
            RichBlock::List(RichBlockList {
                items: vec![RichBlockListItem {
                    label: "1.".into(),
                    blocks: vec![],
                    kind: Some("1".into()),
                    value: Some(1),
                    has_checkbox: true,
                    is_checked: true,
                }],
            }),
            RichBlock::BlockQuotation(RichBlockBlockQuotation {
                blocks: vec![],
                credit: Some("cr".into()),
            }),
            RichBlock::PullQuotation(RichBlockPullQuotation {
                text: "x".into(),
                credit: Some("cr".into()),
            }),
            RichBlock::Collage(RichBlockCollage { blocks: vec![], caption: caption() }),
            RichBlock::Slideshow(RichBlockSlideshow { blocks: vec![], caption: caption() }),
            RichBlock::Table(RichBlockTable {
                cells: vec![vec![RichBlockTableCell::default()]],
                caption: Some("c".into()),
                is_bordered: true,
                is_striped: true,
            }),
            RichBlock::Details(RichBlockDetails {
                summary: "s".into(),
                blocks: vec![],
                is_open: true,
            }),
            RichBlock::Map(RichBlockMap {
                location: Location {
                    longitude: 1.0,
                    latitude: 2.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None,
                },
                zoom: 1,
                width: 1,
                height: 1,
                caption: caption(),
            }),
            // Media blocks: their `mime_type` is optional on the wire, so
            // these also pin that an absent one doesn't sink the block.
            RichBlock::Animation(RichBlockAnimation {
                animation: Animation {
                    file: media_file(),
                    width: 1,
                    height: 1,
                    duration: Seconds::from_seconds(1),
                    thumbnail: None,
                    file_name: None,
                    mime_type: None,
                },
                has_spoiler: true,
                caption: caption(),
            }),
            RichBlock::Audio(RichBlockAudio {
                audio: Audio {
                    file: media_file(),
                    duration: Seconds::from_seconds(1),
                    performer: None,
                    title: None,
                    file_name: None,
                    mime_type: None,
                    thumbnail: None,
                },
                caption: caption(),
            }),
            RichBlock::Photo(RichBlockPhoto {
                photo: vec![],
                has_spoiler: true,
                caption: caption(),
            }),
            RichBlock::Video(RichBlockVideo {
                video: Video {
                    file: media_file(),
                    width: 1,
                    height: 1,
                    duration: Seconds::from_seconds(1),
                    thumbnail: None,
                    file_name: None,
                    mime_type: None,
                    cover: None,
                    start_timestamp: None,
                },
                has_spoiler: true,
                caption: caption(),
            }),
            RichBlock::VoiceNote(RichBlockVoiceNote {
                voice_note: Voice {
                    file: media_file(),
                    duration: Seconds::from_seconds(1),
                    mime_type: None,
                },
                caption: caption(),
            }),
            RichBlock::Thinking(RichBlockThinking { text: "x".into() }),
        ];

        let json = serde_json::to_string(&blocks).unwrap();
        let back: Vec<RichBlock> = serde_json::from_str(&json).unwrap();
        for (original, parsed) in blocks.iter().zip(&back) {
            assert_eq!(original, parsed, "round trip changed a block");
        }

        let texts = vec![
            RichText::Bold(RichTextSimple::new("x")),
            RichText::Italic(RichTextSimple::new("x")),
            RichText::Underline(RichTextSimple::new("x")),
            RichText::Strikethrough(RichTextSimple::new("x")),
            RichText::Spoiler(RichTextSimple::new("x")),
            RichText::Subscript(RichTextSimple::new("x")),
            RichText::Superscript(RichTextSimple::new("x")),
            RichText::Marked(RichTextSimple::new("x")),
            RichText::Code(RichTextSimple::new("x")),
            RichText::Url(RichTextUrl { text: Box::new("x".into()), url: "u".into() }),
            RichText::MathematicalExpression(RichTextMathematicalExpression {
                expression: "e".into(),
            }),
            RichText::CustomEmoji(RichTextCustomEmoji {
                custom_emoji_id: "c".into(),
                alternative_text: "a".into(),
            }),
            RichText::DateTime(RichTextDateTime {
                text: Box::new("x".into()),
                unix_time: 1,
                date_time_format: "wDT".into(),
            }),
            RichText::TextMention(RichTextTextMention {
                text: Box::new("x".into()),
                user: User {
                    id: UserId(1),
                    is_bot: false,
                    first_name: "T".into(),
                    last_name: None,
                    username: None,
                    language_code: None,
                    is_premium: false,
                    added_to_attachment_menu: false,
                },
            }),
            RichText::EmailAddress(RichTextEmailAddress {
                text: Box::new("x".into()),
                email_address: "e@example.com".into(),
            }),
            RichText::PhoneNumber(RichTextPhoneNumber {
                text: Box::new("x".into()),
                phone_number: "+1".into(),
            }),
            RichText::BankCardNumber(RichTextBankCardNumber {
                text: Box::new("x".into()),
                bank_card_number: "4242".into(),
            }),
            RichText::Mention(RichTextMention { text: Box::new("x".into()), username: "u".into() }),
            RichText::Hashtag(RichTextHashtag { text: Box::new("x".into()), hashtag: "h".into() }),
            RichText::Cashtag(RichTextCashtag { text: Box::new("x".into()), cashtag: "c".into() }),
            RichText::BotCommand(RichTextBotCommand {
                text: Box::new("x".into()),
                bot_command: "c".into(),
            }),
            RichText::Anchor(RichTextAnchor { name: "n".into() }),
            RichText::AnchorLink(RichTextAnchorLink {
                text: Box::new("x".into()),
                anchor_name: "n".into(),
            }),
            RichText::Reference(RichTextReference { text: Box::new("x".into()), name: "n".into() }),
            RichText::ReferenceLink(RichTextReferenceLink {
                text: Box::new("x".into()),
                reference_name: "n".into(),
            }),
        ];

        let json = serde_json::to_string(&texts).unwrap();
        let back: Vec<RichText> = serde_json::from_str(&json).unwrap();
        for (original, parsed) in texts.iter().zip(&back) {
            assert_eq!(original, parsed, "round trip changed a text node");
        }
    }

    #[test]
    fn rich_message_is_rtl_roundtrips() {
        let json = serde_json::json!({ "blocks": [], "is_rtl": true });
        let rich: RichMessage = serde_json::from_value(json.clone()).unwrap();
        assert!(rich.is_rtl);
        assert_eq!(serde_json::to_value(&rich).unwrap(), json);

        // Absent on the wire means `false`, and `false` is omitted when
        // serializing back out (matching how every other flag in this file
        // behaves).
        let rich: RichMessage =
            serde_json::from_value(serde_json::json!({ "blocks": [] })).unwrap();
        assert!(!rich.is_rtl);
        assert_eq!(serde_json::to_value(&rich).unwrap(), serde_json::json!({ "blocks": [] }));
    }

    #[test]
    fn input_rich_message_uses_exactly_one_content_field() {
        let html = serde_json::to_value(InputRichMessage::html("<p>hi</p>")).unwrap();
        assert_eq!(html, serde_json::json!({ "html": "<p>hi</p>" }));

        let markdown = serde_json::to_value(InputRichMessage::markdown("**hi**")).unwrap();
        assert_eq!(markdown, serde_json::json!({ "markdown": "**hi**" }));

        let blocks = serde_json::to_value(InputRichMessage::blocks([
            InputRichBlock::Paragraph { text: "hi".into() },
            InputRichBlock::Divider,
        ]))
        .unwrap();
        assert_eq!(
            blocks,
            serde_json::json!({
                "blocks": [
                    { "type": "paragraph", "text": "hi" },
                    { "type": "divider" }
                ]
            })
        );
    }

    #[test]
    fn input_rich_block_photo_serializes_as_an_object() {
        // Regression: `photo` is an `InputMediaPhoto` when sending, but an
        // array of `PhotoSize` when receiving.
        let block = InputRichBlock::Photo {
            photo: InputMediaPhoto::new(InputFile::file_id("AgACAgIAAx".into())),
            caption: Some(RichBlockCaption {
                text: "a cat".into(),
                credit: Some("The Author".into()),
            }),
        };

        assert_eq!(
            serde_json::to_value(block).unwrap(),
            serde_json::json!({
                "type": "photo",
                "photo": { "type": "photo", "media": "AgACAgIAAx" },
                "caption": { "text": "a cat", "credit": "The Author" }
            })
        );
    }

    #[test]
    fn input_rich_message_media_is_referenced_by_id() {
        let message = InputRichMessage::html("<img src=\"tg://photo?id=cat\"/>")
            .media([InputRichMessageMedia::new(
                "cat",
                InputMediaPhoto::new(InputFile::url(
                    "https://example.com/cat.png".parse().unwrap(),
                )),
            )])
            .skip_entity_detection(true);

        assert_eq!(
            serde_json::to_value(message).unwrap(),
            serde_json::json!({
                "html": "<img src=\"tg://photo?id=cat\"/>",
                "media": [{
                    "id": "cat",
                    "media": { "type": "photo", "media": "https://example.com/cat.png" }
                }],
                "skip_entity_detection": true
            })
        );
    }

    /// The files `copy_files` hands to the multipart layer, in order, as they
    /// are referenced from the serialized payload.
    fn copied_files(message: &InputRichMessage) -> Vec<String> {
        let mut files = Vec::new();
        message.copy_files(&mut |file| {
            files.push(serde_json::to_value(file).unwrap().as_str().unwrap().to_owned());
        });
        files
    }

    #[test]
    fn input_rich_message_media_files_are_collected() {
        let message = InputRichMessage::html("<img src=\"tg://photo?id=cat\"/>").media([
            InputRichMessageMedia::new("cat", InputMediaPhoto::new(InputFile::file_id("c".into()))),
            InputRichMessageMedia::new(
                "clip",
                InputMediaVideo::new(InputFile::file_id("v".into()))
                    .thumbnail(InputFile::file_id("t".into())),
            ),
        ]);

        assert_eq!(copied_files(&message), ["c", "v", "t"]);
    }

    #[test]
    fn input_rich_block_files_are_collected_recursively() {
        let message = InputRichMessage::blocks([
            InputRichBlock::Paragraph { text: "no files here".into() },
            InputRichBlock::List {
                items: vec![InputRichBlockListItem::new([InputRichBlock::Collage {
                    blocks: vec![InputRichBlock::Photo {
                        photo: InputMediaPhoto::new(InputFile::file_id("nested".into())),
                        caption: None,
                    }],
                    caption: None,
                }])],
            },
            InputRichBlock::VoiceNote {
                voice_note: InputMediaVoiceNote::new(InputFile::file_id("voice".into())),
                caption: None,
            },
        ]);

        assert_eq!(copied_files(&message), ["nested", "voice"]);

        let mut message = message;
        let mut moved = Vec::new();
        message.move_files(&mut |file| {
            moved.push(serde_json::to_value(file).unwrap().as_str().unwrap().to_owned());
        });

        assert_eq!(moved, ["nested", "voice"]);
    }

    #[test]
    fn input_rich_block_renames_match_the_api() {
        let cases = [
            (InputRichBlock::SectionHeading { text: "h".into(), size: 2 }, "heading"),
            (InputRichBlock::Preformatted { text: "c".into(), language: None }, "pre"),
            (InputRichBlock::BlockQuotation { blocks: vec![], credit: None }, "blockquote"),
            (InputRichBlock::PullQuotation { text: "q".into(), credit: None }, "pullquote"),
            (
                InputRichBlock::VoiceNote {
                    voice_note: InputMediaVoiceNote::new(InputFile::file_id("v".into())),
                    caption: None,
                },
                "voice_note",
            ),
        ];

        for (block, expected) in cases {
            let value = serde_json::to_value(block).unwrap();
            assert_eq!(value["type"], expected);
        }
    }
}
