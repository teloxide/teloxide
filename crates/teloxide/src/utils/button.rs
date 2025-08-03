//! Inline button parser.
//!
//! To use it, you need to create an `enum` with derived [`InlineButtons`],
//! containing buttons of your keyboard.
//!
//! # Using InlineButtons
//!
//! ```
//! # #[cfg(feature = "macros")] {
//! use teloxide::{types::InlineKeyboardButton, utils::button::InlineButtons};
//!
//! #[derive(InlineButtons, Debug, PartialEq)]
//! enum CallbackButtons {
//!     Fruit(String),
//!     Other,
//! }
//!
//! let data = "Fruit;apple";
//! let expected = DefaultData::Fruit("apple".to_string());
//! let actual = DefaultData::parse(data).unwrap();
//! assert_eq!(actual, expected);
//! # }
//! ```
//! See [examples/inline_buttons_enum] as a more complicated examples.
//!
//! [examples/inline_buttons_enum]: https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/inline_buttons_enum.rs

use std::fmt::{Display, Formatter};

use super::command::ParseError;
use teloxide_core::types::InlineKeyboardButton;
#[cfg(feature = "macros")]
pub use teloxide_macros::InlineButtons;

/// An enumeration of keyboards buttons.
///
/// # Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::{types::InlineKeyboardButton, utils::button::InlineButtons};
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// enum CallbackButtons {
///     Fruit(String),
///     Other,
/// }
///
/// let data = "Fruit;apple";
/// let expected = DefaultData::Fruit("apple".to_string());
/// let actual = DefaultData::parse(data).unwrap();
/// assert_eq!(actual, expected);
/// # }
/// ```
///
/// # Building buttons
///
/// Using this macro you can build buttons to achieve the same result you would with regular
/// builders:
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::{types::InlineKeyboardButton, utils::button::InlineButtons};
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// enum CallbackButtons {
///     Button1,
///     Button2,
/// }
///
/// let text = "Text for button 1";
/// let actual = CallbackButtons::Button1.build_button(text).unwrap();
/// let expected = InlineKeyboardButton::callback(text, "Button1");
/// assert_eq!(actual, expected);
/// # }
/// ```
///
/// # Enum attributes
///  1. `#[button(fields_separator = "separator")]` change the separator of the fields (the default
///     is `;`). Useful if the default separator can be in the data.
///
/// ## Example
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::{types::InlineKeyboardButton, utils::button::InlineButtons};
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// #[button(fields_separator = "|")]
/// enum CallbackButtons {
///     Fruit(String),
///     Other,
/// }
///
/// let data = "Fruit|apple";
/// let expected = DefaultData::Fruit("apple".to_string());
/// let actual = DefaultData::parse(data).unwrap();
/// assert_eq!(actual, expected);
/// # }
/// ```
///
/// # Variant attributes
/// All variant attributes override the corresponding `enum` attributes.
///  1. `#[button(fields_separator = "separator")]` change the separator of the field (the default
///     is `;`). Useful if the default separator can be in the data.
///  2. `#[button(rename = "rename")]` change the serialized name of the field. Useful if the
///     enum variants name is long (64 character is the data limit in the TBA).
///
/// ## Example
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::{types::InlineKeyboardButton, utils::button::InlineButtons};
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// enum CallbackButtons {
///     #[button(rename = "f")]
///     Fruit(String),
///     Other,
/// }
///
/// let data = "f;apple";
/// let expected = DefaultData::Fruit("apple".to_string());
/// let actual = DefaultData::parse(data).unwrap();
/// assert_eq!(actual, expected);
/// # }
/// ```
pub trait InlineButtons: Sized {
    /// Parses the callback data.
    fn parse(s: &str) -> Result<Self, ParseError>;

    /// Stringifies the callback data.
    fn stringify(self) -> Result<String, StringifyError>;

    /// Builds an [`InlineKeyboardButton`] from the enum variant
    ///
    /// [`InlineKeyboardButton`]: crate::types::InlineKeyboardButton
    fn build_button<T>(self, text: T) -> Result<InlineKeyboardButton, StringifyError>
    where
        T: Into<String>,
    {
        let callback_data = self.stringify()?;
        Ok(InlineKeyboardButton::callback(text.into(), callback_data))
    }
}

/// Errors returned from [`InlineButtons::stringify`].
///
/// [`InlineButtons::stringify`]: InlineButtons::stringify
#[derive(Debug)]
pub enum StringifyError {
    SeparatorInNamedArgument {
        enum_variant: String,
        stringified_data: String,
        separator: String,
        argument: String,
    },
    SeparatorInUnnamedArgument {
        enum_variant: String,
        stringified_data: String,
        separator: String,
        field: usize,
    },
}

fn make_pointers_string(prefix: String, text: String, point_to: String) -> String {
    let all_indexes: Vec<_> = text.match_indices(&point_to).collect();
    let prefix_len = prefix.chars().count();
    let mut pointers_vec = vec![" "; prefix_len + text.chars().count()];

    for (start, matched) in &all_indexes {
        for (i, _) in matched.chars().enumerate() {
            pointers_vec[prefix_len + start + i] = "^";
        }
    }
    pointers_vec.join("")
}

impl Display for StringifyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SeparatorInNamedArgument {
                enum_variant,
                stringified_data,
                separator,
                argument,
            } => {
                let prefix = format!("self.{argument} == \"");
                // Makes ^^^ pointers to where the separator is
                let separator_pointers_string = make_pointers_string(
                    prefix.clone(),
                    stringified_data.to_string(),
                    separator.to_string(),
                );
                write!(
                    f,
                    "There is a separator \"{separator}\" in `{enum_variant}`: \
                     \n{prefix}{stringified_data}\"\n{separator_pointers_string}\n\nPlease \
                     consider changing the separator with \
                     `#[button(fields_separator=\"NEW_SEPARATOR\")]`"
                )
            }
            Self::SeparatorInUnnamedArgument {
                enum_variant,
                stringified_data,
                separator,
                field,
            } => {
                let prefix = format!("self.{field} == \"");
                let separator_pointers_string = make_pointers_string(
                    prefix.clone(),
                    stringified_data.to_string(),
                    separator.to_string(),
                );
                write!(
                    f,
                    "There is a separator \"{separator}\" in `{enum_variant}`: \
                     \n{prefix}{stringified_data}\"\n{separator_pointers_string}\n\nPlease \
                     consider changing the separator with \
                     `#[button(fields_separator=\"NEW_SEPARATOR\")]`"
                )
            }
        }
    }
}

impl std::error::Error for StringifyError {}
