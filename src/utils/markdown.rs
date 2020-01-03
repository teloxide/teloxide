use std::string::String;

// Escapes all markdown special characters in the string
// https://core.telegram.org/bots/api#markdownv2-style
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

// Escapes all markdown special characters in the link URL (...)
pub fn escape_link_url(s: &str) -> String {
    s.replace("`", r"\`").replace(")", r"\)")
}

// Escapes all markdown special characters in the code block
pub fn escape_code_block(s: &str) -> String {
    s.replace(r"\", r"\\").replace("`", r"\`")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_escapecode_block() {
        assert_eq!(
            escape_code_block(r"` \code inside the code\ `"),
            r"\` \\code inside the code\\ \`"
        );
        assert_eq!(
            escape_code_block(r"_*[]()~\`#+-=|{}.!\"),
            r"_*[]()~\\\`#+-=|{}.!\\"
        );
    }
}
