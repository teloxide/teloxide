use serde::{Deserialize, Serialize};

/// This object represents a bot command.
///
/// [The official docs](https://core.telegram.org/bots/api#botcommand).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotCommand {
    /// Text of the command, 1-32 characters.
    ///
    /// Can contain only lowercase English letters, digits and underscores.
    pub command: String,

    /// Description of the command, 3-256 characters.
    pub description: String,
}

impl BotCommand {
    pub fn new<S1, S2>(command: S1, description: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self { command: command.into(), description: description.into() }
    }

    pub fn command<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.command = val.into();
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = val.into();
        self
    }
}
