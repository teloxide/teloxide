use std::string::String;

// Escapes the string to be shown "as is" within the Telegram HTML message style.
// Does not escape ' and " characters (as should be for usual HTML).
// Because they shoudn't be escaped by the spec: https://core.telegram.org/bots/api#html-style
pub fn escape(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
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
