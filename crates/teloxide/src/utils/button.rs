//! Docs later
use super::command::ParseError;
#[cfg(feature = "macros")]
pub use teloxide_macros::InlineButtons;

/// Docs later
pub trait InlineButtons: Sized {
    /// Parses the callback data.
    fn parse(s: &str) -> Result<Self, ParseError>;

    /// Stringifies the callback data.
    fn stringify(self) -> Result<String, StringifyError>;
}

/// Errors returned from [`InlineButtons::stringify`].
///
/// [`InlineButtons::stringify`]: InlineButtons::stringify
#[derive(Debug)]
pub enum StringifyError {
    SeparatorInNamedArgument { enum_variant: String, argument: String },
    SeparatorInUnnamedArgument { enum_variant: String, field: usize },
}
