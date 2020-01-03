use std::string::String;

// Escapes the string to be shown "as is" within the Telegram HTML message style.
// Does not escape ' and " characters (as should be for usual HTML).
// Because they shoudn't be escaped by the spec: https://core.telegram.org/bots/api#html-style
pub fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .to_string()
}

// Escapes all markdown special characters in the string
// https://core.telegram.org/bots/api#markdownv2-style
pub fn escape_markdown(s: &str) -> String {
    s.replace("_", r"\_")
        .replace("*", r"\*")
        .replace("[", r"\[")
        .replace("]", r"\]")
        .replace("(", r"\(")
        .replace(")", r"\)")
        .replace("~", r"\~")
        .replace("`", r"\`")
        .replace("#", r"\#")
        .replace("+", r"\+")
        .replace("-", r"\-")
        .replace("=", r"\=")
        .replace("|", r"\|")
        .replace("{", r"\{")
        .replace("}", r"\}")
        .replace(".", r"\.")
        .replace("!", r"\!")
}

// Escapes all markdown special characters in the link URL (...)
pub fn escape_markdown_link_url(s: &str) -> String {
    s.replace("`", r"\`").replace(")", r"\)")
}

// Escapes all markdown special characters in the code block
pub fn escape_markdown_code_block(s: &str) -> String {
    s.replace(r"\", r"\\").replace("`", r"\`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(
            escape_html("  <title>Foo & Bar</title>   "),
            "  &lt;title&gt;Foo &amp; Bar&lt;/title&gt;   "
        );
        assert_eq!(
            escape_html("<p>你好 & 再見</p>"),
            "&lt;p&gt;你好 &amp; 再見&lt;/p&gt;"
        );
        assert_eq!(escape_html("'foo\""), "'foo\"");
    }

    #[test]
    fn test_escape_markdown_link_url() {
        assert_eq!(
            escape_markdown_link_url(
                r"https://en.wikipedia.org/wiki/Development+(Software)"
            ),
            r"https://en.wikipedia.org/wiki/Development+(Software\)"
        );
        assert_eq!(
            escape_markdown_link_url(r"https://en.wikipedia.org/wiki/`"),
            r"https://en.wikipedia.org/wiki/\`"
        );
        assert_eq!(
            escape_markdown_link_url(r"_*[]()~`#+-=|{}.!\"),
            r"_*[](\)~\`#+-=|{}.!\"
        );
    }

    #[test]
    fn test_escape_markdown_code_block() {
        assert_eq!(
            escape_markdown_code_block(r"` \code inside the code\ `"),
            r"\` \\code inside the code\\ \`"
        );
        assert_eq!(
            escape_markdown_code_block(r"_*[]()~\`#+-=|{}.!\"),
            r"_*[]()~\\\`#+-=|{}.!\\"
        );
    }
}
