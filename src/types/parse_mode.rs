use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
/// ## Formatting options
/// The Bot API supports basic formatting for messages.
/// You can use **bold** and *italic* text, as well as [inline links](https://example.com) and `pre-formatted code` in
/// your bots' messages. Telegram clients will render them accordingly. You can
/// use either markdown-style or HTML-style formatting.
///
/// Note that Telegram clients will display an alert to the user before opening
/// an inline link (‘Open this link?’ together with the full URL).
///
/// Links `tg://user?id=<user_id>` can be used to mention a user by their id
/// without using a username. Please note:
///
/// - These links will work only if they are used inside an inline link. For
///   example, they will not work, when used in an inline keyboard button or in
///   a message text.
/// - The mentions are only guaranteed to work if: **A**. the user is a member
///   in the group where he was mentioned or **B**. the user has contacted the
///   bot in the past or has sent a callback query to the bot via inline button
///   and has not restricted linking to their account in `Settings > Privacy &
///   Security > Forwarded Messages`.
///
/// ## Markdown style
/// To use this mode, pass [Markdown] in the `parse_mode` field when using
/// [SendMessage] (or other methods).
///
/// Use the following syntax in your message:
///
/// ```ignore
/// *bold text*
/// _italic text_
/// [inline URL](http://www.example.com/)
/// [inline mention of a user](tg://user?id=123456789)
/// `inline fixed-width code`
/// &#96;&#96;&#96;block_language
/// pre-formatted fixed-width code block
/// &#96;&#96;&#96;
/// ```
///
/// ## HTML style
/// To use this mode, pass [HTML] in the `parse_mode` field when using
/// [SendMessage] (or other methods).
///
/// The following tags are currently supported:
///
/// ```ignore
/// <b>bold</b>, <strong>bold</strong>
/// <i>italic</i>, <em>italic</em>
/// <a href="http://www.example.com/">inline URL</a>
/// <a href="tg://user?id=123456789">inline mention of a user</a>
/// <code>inline fixed-width code</code>
/// <pre>pre-formatted fixed-width code block</pre>
/// ```
///
/// Please note:
///
/// - Only the tags mentioned above are currently supported.
/// - Tags must not be nested.
/// - All `<`, `>` and `&` symbols that are not a part of a tag or an HTML
///   entity must be replaced with the corresponding HTML entities (`<` with
///   `&lt;`, `>` with `&gt;` and `&` with `&amp;`).
/// - All numerical HTML entities are supported.
/// - The API currently supports only the following named HTML entities: `&lt;`,
///   `&gt;`, `&amp;` and `&quot;`.
pub enum ParseMode {
    HTML,
    Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_serialization() {
        let expected_json = String::from(r#""HTML""#);
        let actual_json = serde_json::to_string(&ParseMode::HTML).unwrap();

        assert_eq!(expected_json, actual_json)
    }

    #[test]
    fn markdown_serialization() {
        let expected_json = String::from(r#""Markdown""#);
        let actual_json = serde_json::to_string(&ParseMode::Markdown).unwrap();

        assert_eq!(expected_json, actual_json)
    }
}
