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
//! let expected = CallbackButtons::Fruit("apple".to_string());
//! let actual = CallbackButtons::parse(data).unwrap();
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

/// An enumeration of inline keyboard buttons with automatic parsing,
/// serialization, and keyboard generation.
///
/// This trait is implemented via the `#[derive(InlineButtons)]` macro.
/// It allows defining inline keyboard buttons using enum variants.
///
/// Each enum variant represents a button. Variants may:
/// - have no fields (unit variants),
/// - contain unnamed fields (tuple variants),
/// - contain named fields (struct variants).
///
/// The macro generates:
/// - parsing from callback data (`parse`)
/// - serialization into callback data (`stringify`)
/// - button construction (`build_button`)
/// - keyboard construction (`build_keyboard`)
///
/// For usage examples look to `examples/inline_buttons_enum.rs` and
/// `examples/inline_keyboard_enum.rs`
///
/// ---
///
/// # Callback data format
///
/// Callback data is serialized as:
///
/// ```text
/// VariantName<sep>arg1<sep>arg2...
/// ```
///
/// where `<sep>` is `;` by default, or a custom separator.
///
/// ## Example
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::button::InlineButtons;
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// enum CallbackButtons {
///     Fruit(String),
///     Other,
/// }
///
/// let data = "Fruit;apple";
/// let parsed = CallbackButtons::parse(data).unwrap();
/// assert_eq!(parsed, CallbackButtons::Fruit("apple".into()));
///
/// assert_eq!(parsed.stringify().unwrap(), data);
/// # }
/// ```
///
/// ---
///
/// # Building buttons
///
/// Each enum variant can be turned into an [`InlineKeyboardButton`].
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::{types::InlineKeyboardButton, utils::button::InlineButtons};
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// enum Buttons {
///     A,
/// }
///
/// let button = Buttons::A.build_button("Click me").unwrap();
/// let expected = InlineKeyboardButton::callback("Click me", "A");
///
/// assert_eq!(button, expected);
/// # }
/// ```
///
/// ---
///
/// # Building keyboards
///
/// The derive macro also generates a `build_keyboard` method on the enum.
///
/// This method:
/// - constructs all buttons
/// - arranges them into rows
/// - returns [`InlineKeyboardMarkup`]
///
/// Arguments required by variants are passed as function parameters.
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::button::InlineButtons;
///
/// #[derive(InlineButtons, Debug)]
/// enum Buttons {
///     #[button(row = 1)]
///     A,
///     #[button(row = 2)]
///     B(i32),
/// }
///
/// let keyboard = Buttons::build_keyboard(42).unwrap();
/// assert_eq!(keyboard.inline_keyboard.len(), 2);
/// # }
/// ```
///
/// ## Row behavior
///
/// - Rows are 1-based (`row = 1` is the first row)
/// - Multiple buttons can share the same row
/// - Rows are grouped automatically
///
/// ---
///
/// # Enum attributes
///
/// ## `fields_separator`
///
/// Changes the separator used between fields.
///
/// Default: `;`
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::button::InlineButtons;
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// #[button(fields_separator = "|")]
/// enum Buttons {
///     A(i32),
/// }
///
/// let data = "A|10";
/// assert_eq!(Buttons::parse(data).unwrap(), Buttons::A(10));
/// # }
/// ```
///
/// ---
///
/// # Variant attributes
///
/// Variant attributes override enum-level settings.
///
/// ## `rename`
///
/// Overrides the serialized name of the variant.
///
/// Useful because Telegram limits callback data to **64 characters**.
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::button::InlineButtons;
///
/// #[derive(InlineButtons, Debug, PartialEq)]
/// enum Buttons {
///     #[button(rename = "f")]
///     Fruit(String),
/// }
///
/// assert_eq!(Buttons::parse("f;apple").unwrap(), Buttons::Fruit("apple".into()));
/// # }
/// ```
///
/// ---
///
/// ## `row`
///
/// Assigns a button to a keyboard row.
///
/// ```
/// #[button(row = 2)]
/// ```
///
/// Constraints:
/// - Must be ≥ 1
/// - Rows must be incremental in order
///
/// ---
///
/// ## `text`
///
/// Sets the button display text.
///
/// ```
/// #[button(text = "Click me")]
/// ```
///
/// If not provided, defaults to variant name (or `rename` if present)
///
/// ---
///
/// # Special button types
///
/// The following attributes change button behavior instead of using callback
/// data.
///
/// These are **mutually exclusive** and only valid on **unit variants**:
///
/// - `url = "..."` Creates a URL button
///
/// - `login_url = "..."` Creates a login button
///
/// - `webapp_url = "..."` Creates a WebApp button
///
/// - `switch_inline_query = "..."`
///
/// - `switch_inline_query_current_chat = "..."`
///
/// - `copy_text = "..."`
///
/// - `game = true`
///
/// - `pay = true`
///
/// ## Example
///
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::button::InlineButtons;
///
/// #[derive(InlineButtons)]
/// enum Buttons {
///     #[button(url = "https://example.com")]
///     Link,
/// }
/// # }
/// ```
///
/// Constraints:
/// - Only one of these may be specified per variant
/// - Cannot be used on variants with fields
///
/// ---
///
/// # Notes
///
/// - Callback data must not exceed Telegram limits (64 characters)
/// - Field values must not contain the separator
/// - Argument order in build_keyboard must match variant definition
///
/// ---
///
/// # Trait methods
///
/// ## `parse`
///
/// Parses callback data into an enum variant.
///
/// ## `stringify`
///
/// Serializes the enum into callback data.
///
/// ## `build_button`
///
/// Builds a single [`InlineKeyboardButton`] using callback data.
///
/// ## `build_keyboard`
///
/// Generated by the derive macro. Builds a full keyboard.
///
/// ---
///
/// # See also
///
/// - [`InlineKeyboardButton`]
/// - [`InlineKeyboardMarkup`]
///
/// [`InlineKeyboardButton`]: crate::types::InlineKeyboardButton
/// [`InlineKeyboardMarkup`]: crate::types::InlineKeyboardMarkup
pub trait InlineButtons: Sized {
    /// Parses the callback data.
    fn parse(s: &str) -> Result<Self, ParseError>;

    /// Stringifies the callback data.
    fn stringify(self) -> Result<String, StringifyError>;

    /// Builds an [`InlineKeyboardButton`] from the enum variant.
    ///
    /// This uses the serialized callback data internally.
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
