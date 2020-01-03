//! Utils for working with the [MarkdownV2 message style.](https://core.telegram.org/bots/api#markdownv2-style)

use std::string::String;

/// Applies the bold font style to the string.
/// Passed string will not be automatically escaped
/// because it can contain nested markup.
pub fn bold(s: &str) -> String {
    wrap(s, "*", "*")
}

/// Applies the italic font style to the string.
/// Can be safely used with `utils::markdown::underline()`.
/// Passed string will not be automatically escaped
/// because it can contain nested markup.
pub fn italic(s: &str) -> String {
    if s.starts_with("__") && s.ends_with("__") {
        return wrap(&s[..s.len() - 1], "_", r"\r__");
    }
    wrap(s, "_", "_")
}

/// Applies the underline font style to the string.
/// Can be safely used with `utils::markdown::italic()`.
/// Passed string will not be automatically escaped
/// because it can contain nested markup.
pub fn underline(s: &str) -> String {
    // In case of ambiguity between italic and underline entities
    // ‘__’ is always greadily treated from left to right as beginning or end of underline entity,
    // so instead of ___italic underline___ we should use ___italic underline_\r__,
    // where \r is a character with code 13, which will be ignored.
    if s.starts_with("_") && s.ends_with("_") {
        return wrap(s, "__", r"\r__");
    }
    wrap(s, "__", "__")
}

/// Applies the strikethrough font style to the string.
/// Passed string will not be automatically escaped
/// because it can contain nested markup.
pub fn strike(s: &str) -> String {
    wrap(s, "~", "~")
}

/// Builds an inline link with an anchor.
/// Escapes `)` and ``` characters inside the link url.
pub fn link(url: &str, text: &str) -> String {
    let mut out = String::with_capacity(url.len() + text.len() + 4);
    out.push_str(wrap(text, "[", "]").as_str());
    out.push_str(wrap(escape_link_url(url).as_str(), "(", ")").as_str());
    out
}

/// Builds an inline user mention link with an anchor.
pub fn user_mention(user_id: i32, text: &str) -> String {
    link(format!("tg://user?id={}", user_id).as_str(), text)
}

/// Formats the code block. Escapes ``` and `\` characters inside the block.
pub fn code_block(code: &str) -> String {
    code_block_with_lang(code, "")
}

/// Formats the code block with a specific language syntax.
/// Escapes ``` and `\` characters inside the block.
pub fn code_block_with_lang(code: &str, lang: &str) -> String {
    wrap(
        escape_code(code).as_str(),
        format!("```{}\n", lang).as_str(),
        "\n```",
    )
}

/// Formats the string as an inline code.
/// Escapes ``` and `\` characters inside the block.
pub fn code_inline(s: &str) -> String {
    wrap(escape_code(s).as_str(), "`", "`")
}

/// Escapes all markdown special characters in the passed string.
pub fn escape(s: &str) -> String {
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

/// Escapes all markdown special characters specific for the inline link URL (``` and `)`)
pub fn escape_link_url(s: &str) -> String {
    s.replace("`", r"\`").replace(")", r"\)")
}

/// Escapes all markdown special characters specific for the code block (``` and `\`)
pub fn escape_code(s: &str) -> String {
    s.replace(r"\", r"\\").replace("`", r"\`")
}

fn wrap(s: &str, left: &str, right: &str) -> String {
    let mut out = String::with_capacity(left.len() + s.len() + right.len());
    out.push_str(left);
    out.push_str(s);
    out.push_str(right);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!(bold(" foobar "), "* foobar *");
        assert_eq!(bold(" _foobar_ "), "* _foobar_ *");
        assert_eq!(bold("~(`foobar`)~"), "*~(`foobar`)~*");
    }

    #[test]
    fn test_italic() {
        assert_eq!(italic(" foobar "), "_ foobar _");
        assert_eq!(italic("*foobar*"), "_*foobar*_");
        assert_eq!(italic("~(foobar)~"), "_~(foobar)~_");
    }

    #[test]
    fn test_underline() {
        assert_eq!(underline(" foobar "), "__ foobar __");
        assert_eq!(underline("*foobar*"), "__*foobar*__");
        assert_eq!(underline("~(foobar)~"), "__~(foobar)~__");
    }

    #[test]
    fn test_strike() {
        assert_eq!(strike(" foobar "), "~ foobar ~");
        assert_eq!(strike("*foobar*"), "~*foobar*~");
        assert_eq!(strike("*(foobar)*"), "~*(foobar)*~");
    }

    #[test]
    fn test_italic_with_underline() {
        assert_eq!(underline(italic("foobar").as_str()), r"___foobar_\r__");
        assert_eq!(italic(underline("foobar").as_str()), r"___foobar_\r__");
    }

    #[test]
    fn test_link() {
        assert_eq!(
            link("https://www.google.com/(`foobar`)", "google"),
            r"[google](https://www.google.com/(\`foobar\`\))",
        );
    }

    #[test]
    fn test_user_mention() {
        assert_eq!(
            user_mention(123456789, "pwner666"),
            "[pwner666](tg://user?id=123456789)"
        );
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            code_block("pre-'formatted'\nfixed-width \\code `block`"),
            "```\npre-'formatted'\nfixed-width \\\\code \\`block\\`\n```"
        );
    }
    
    #[test]
    fn test_code_block_with_language() {
        assert_eq!(
            code_block_with_lang(
                "pre-'formatted'\nfixed-width \\code `block`",
                "python"
            ),
            "```python\npre-'formatted'\nfixed-width \\\\code \\`block\\`\n```"
        );
    }

    #[test]
    fn test_code_inline() {
        assert_eq!(
            code_inline(" let x = (1, 2, 3); "),
            "` let x = (1, 2, 3); `"
        );
        assert_eq!(code_inline("<html>foo</html>"), "`<html>foo</html>`");
        assert_eq!(
            code_inline(r" `(code inside code \ )` "),
            r"` \`(code inside code \\ )\` `"
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape("* foobar *"), r"\* foobar \*");
        assert_eq!(
            escape(r"_ * [ ] ( ) ~ \ ` # + - = | { } . !"),
            r"\_ \* \[ \] \( \) \~ \ \` \# \+ \- \= \| \{ \} \. \!",
        );
    }

    #[test]
    fn test_escape_link_url() {
        assert_eq!(
            escape_link_url(
                r"https://en.wikipedia.org/wiki/Development+(Software)"
            ),
            r"https://en.wikipedia.org/wiki/Development+(Software\)"
        );
        assert_eq!(
            escape_link_url(r"https://en.wikipedia.org/wiki/`"),
            r"https://en.wikipedia.org/wiki/\`"
        );
        assert_eq!(
            escape_link_url(r"_*[]()~`#+-=|{}.!\"),
            r"_*[](\)~\`#+-=|{}.!\"
        );
    }

    #[test]
    fn test_escape_code() {
        assert_eq!(
            escape_code(r"` \code inside the code\ `"),
            r"\` \\code inside the code\\ \`"
        );
        assert_eq!(
            escape_code(r"_*[]()~`#+-=|{}.!\"),
            r"_*[]()~\`#+-=|{}.!\\"
        );
    }
}
