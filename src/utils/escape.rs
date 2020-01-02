use std::string::String;

// Escapes the string to be presented "as is" with the Telegram HTML message style.
// Does not escape ' and " characters (as should be for usual HTML).
// Because they shoudn't be escaped by the spec: https://core.telegram.org/bots/api#html-style
pub fn escape_html(raw: &str) -> String {
    let mut last = 0;
    let mut out = String::new();

    for (i, ch) in raw.bytes().enumerate() {
        match ch as char {
            '<' | '>' | '&' => {
                out.push_str(&raw[last..i]);
                let s = match ch as char {
                    '>' => "&gt;",
                    '<' => "&lt;",
                    '&' => "&amp;",
                    _ => unreachable!(),
                };
                out.push_str(s);
                last = i + 1;
            }
            _ => {}
        }
    }

    if last < raw.len() {
        out.push_str(&raw[last..])
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_escaped_properly() {
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
}
