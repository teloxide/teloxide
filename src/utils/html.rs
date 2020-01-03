//! Utils for working with the [HTML message style][spec]
//!
//! [spec]: https://core.telegram.org/bots/api#html-style
use std::string::String;

/// Escapes the string to be shown "as is" within the Telegram HTML message style.
///
/// Does not escape ' and " characters (as should be for usual HTML), because they shoudn't
/// be escaped by the [spec].
///
/// [spec]: https://core.telegram.org/bots/api#html-style
pub fn escape(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub fn bold(s: &str) -> String {
    format!("<b>{}</b>", s)
}

pub fn italic(s: &str) -> String {
    format!("<i>{}</i>", s)
}

pub fn underline(s: &str) -> String {
    format!("<u>{}</u>", s)
}

pub fn strike(s: &str) -> String {
    format!("<s>{}</s>", s)
}

pub fn link(url: &str, text: &str) -> String {
    format!("<a href=\"{}\">{}</a>", escape(url), text)
}

pub fn user_mention(user_id: i32, text: &str) -> String {
    link(format!("tg://user?id={}", user_id).as_str(), text)
}

pub fn code_block(code: &str) -> String {
    format!("<pre>\n{}\n</pre>", escape(code))
}

pub fn code_block_with_lang(code: &str, lang: &str) -> String {
    format!(
        "<pre><code class=\"language-{}\">\n{}\n</code></pre>",
        escape(lang),
        escape(code)
    )
}

pub fn code_inline(s: &str) -> String {
    format!("<code>{}</code>", escape(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(
            escape("  <title>Foo & Bar</title>   "),
            "  &lt;title&gt;Foo &amp; Bar&lt;/title&gt;   "
        );
        assert_eq!(
            escape("<p>你好 & 再見</p>"),
            "&lt;p&gt;你好 &amp; 再見&lt;/p&gt;"
        );
        assert_eq!(escape("'foo\""), "'foo\"");
    }
}
