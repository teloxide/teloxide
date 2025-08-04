use serde::{Deserialize, Serialize};

use crate::types::WebAppInfo;

/// This object represents a button to be shown above inline query results. You
/// must use exactly one of the optional fields.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultsbutton)
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InlineQueryResultsButton {
    /// Label text on the button
    pub text: String,

    #[serde(flatten)]
    pub kind: InlineQueryResultsButtonKind,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum InlineQueryResultsButtonKind {
    /// Description of the [Web App] that will be launched when the user presses
    /// the button. The Web App will be able to switch back to the inline mode
    /// using the method [switchInlineQuery] inside the Web App.
    ///
    /// [Web App]: https://core.telegram.org/bots/webapps
    /// [switchInlineQuery]: https://core.telegram.org/bots/webapps#initializing-mini-apps
    WebApp(WebAppInfo),

    /// [Deep-linking] parameter for the /start message sent to the bot when a
    /// user presses the button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_`
    /// and `-` are allowed.
    ///
    /// Example: An inline bot that sends YouTube videos can
    /// ask the user to connect the bot to their YouTube account to adapt search
    /// results accordingly. To do this, it displays a 'Connect your YouTube
    /// account' button above the results, or even before showing any. The user
    /// presses the button, switches to a private chat with the bot and, in
    /// doing so, passes a start parameter that instructs the bot to return an
    /// OAuth link. Once done, the bot can offer a [switch_inline] button so
    /// that the user can easily return to the chat where they wanted to use
    /// the bot's inline capabilities.
    ///
    /// [Deep-linking]: https://core.telegram.org/bots/features#deep-linking
    /// [switch_inline]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
    StartParameter(String),
}

#[cfg(test)]
mod tests {
    use crate::types::{InlineQueryResultsButton, InlineQueryResultsButtonKind};

    #[test]
    fn inline_query_results_button() {
        let button = InlineQueryResultsButton {
            text: "test".into(),
            kind: InlineQueryResultsButtonKind::StartParameter("bot".into()),
        };
        let expected = r#"{"text":"test","start_parameter":"bot"}"#;
        let actual = serde_json::to_string(&button).unwrap();
        assert_eq!(expected, actual);
    }
}
