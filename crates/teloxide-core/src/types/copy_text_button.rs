use serde::{Deserialize, Serialize};

/// This object represents an inline keyboard button that copies specified text
/// to the clipboard.
///
/// [The official docs](https://core.telegram.org/bots/api#copytextbutton)

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters
    pub text: String,
}
