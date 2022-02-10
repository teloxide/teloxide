//! Command parsers.
//!
//! You can either create an `enum` with derived [`BotCommand`], containing
//! commands of your bot, or use functions, which split input text into a string
//! command with its arguments.
//!
//! # Using BotCommand
//! ```
//! # #[cfg(feature = "macros")] {
//! use teloxide::utils::command::BotCommand;
//!
//! type UnitOfTime = u8;
//!
//! #[derive(BotCommand, PartialEq, Debug)]
//! #[command(rename = "lowercase", parse_with = "split")]
//! enum AdminCommand {
//!     Mute(UnitOfTime, char),
//!     Ban(UnitOfTime, char),
//! }
//!
//! let command = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
//! assert_eq!(command, AdminCommand::Ban(5, 'h'));
//! # }
//! ```
//!
//! # Using parse_command
//! ```
//! use teloxide::utils::command::parse_command;
//!
//! let (command, args) = parse_command("/ban@MyBotName 3 hours", "MyBotName").unwrap();
//! assert_eq!(command, "ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! # Using parse_command_with_prefix
//! ```
//! use teloxide::utils::command::parse_command_with_prefix;
//!
//! let text = "!ban 3 hours";
//! let (command, args) = parse_command_with_prefix("!", text, "").unwrap();
//! assert_eq!(command, "ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! See [examples/admin_bot] as a more complicated examples.
//!
//! [examples/admin_bot]: https://github.com/teloxide/teloxide/blob/master/examples/admin_bot/

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use std::marker::PhantomData;
#[cfg(feature = "macros")]
pub use teloxide_macros::BotCommand;

/// An enumeration of bot's commands.
///
/// # Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommand;
///
/// type UnitOfTime = u8;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase", parse_with = "split")]
/// enum AdminCommand {
///     Mute(UnitOfTime, char),
///     Ban(UnitOfTime, char),
/// }
///
/// let command = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
/// assert_eq!(command, AdminCommand::Ban(5, 'h'));
/// # }
/// ```
///
/// # Enum attributes
///  1. `#[command(rename = "rule")]`
/// Rename all commands by `rule`. Allowed rules are `lowercase`. If you will
/// not use this attribute, commands will be parsed by their original names.
///
///  2. `#[command(prefix = "prefix")]`
/// Change a prefix for all commands (the default is `/`).
///
///  3. `#[command(description = "description")]`
/// Add a sumary description of commands before all commands.
///
///  4. `#[command(parse_with = "parser")]`
/// Change the parser of arguments. Possible values:
///    - `default` - the same as the unspecified parser. It only puts all text
///    after the first space into the first argument, which must implement
///    [`FromStr`].
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase")]
/// enum Command {
///     Text(String),
/// }
///
/// let command = Command::parse("/text hello my dear friend!", "").unwrap();
/// assert_eq!(command, Command::Text("hello my dear friend!".to_string()));
/// # }
/// ```
///
///  - `split` - separates a messsage by a given separator (the default is the
///    space character) and parses each part into the corresponding arguments,
///    which must implement [`FromStr`].
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase", parse_with = "split")]
/// enum Command {
///     Nums(u8, u16, i32),
/// }
///
/// let command = Command::parse("/nums 1 32 -5", "").unwrap();
/// assert_eq!(command, Command::Nums(1, 32, -5));
/// # }
/// ```
///
/// 5. `#[command(separator = "sep")]`
/// Specify separator used by the `split` parser. It will be ignored when
/// accompanied by another type of parsers.
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase", parse_with = "split", separator = "|")]
/// enum Command {
///     Nums(u8, u16, i32),
/// }
///
/// let command = Command::parse("/nums 1|32|5", "").unwrap();
/// assert_eq!(command, Command::Nums(1, 32, 5));
/// # }
/// ```
///
/// # Variant attributes
/// All variant attributes override the corresponding `enum` attributes.
///
///  1. `#[command(rename = "rule")]`
/// Rename one command by a rule. Allowed rules are `lowercase`, `%some_name%`,
/// where `%some_name%` is any string, a new name.
///
///  2. `#[command(description = "description")]`
/// Give your command a description. Write `"off"` for `"description"` to hide a
/// command.
///
///  3. `#[command(parse_with = "parser")]`
/// One more option is available for variants.
///    - `custom_parser` - your own parser of the signature `fn(String) ->
///    Result<Tuple, ParseError>`, where `Tuple` corresponds to the variant's
/// arguments.
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::{BotCommand, ParseError};
///
/// fn accept_two_digits(input: String) -> Result<(u8,), ParseError> {
///     match input.len() {
///         2 => {
///             let num = input.parse::<u8>().map_err(|e| ParseError::IncorrectFormat(e.into()))?;
///             Ok((num,))
///         }
///         len => Err(ParseError::Custom(format!("Only 2 digits allowed, not {}", len).into())),
///     }
/// }
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase")]
/// enum Command {
///     #[command(parse_with = "accept_two_digits")]
///     Num(u8),
/// }
///
/// let command = Command::parse("/num 12", "").unwrap();
/// assert_eq!(command, Command::Num(12));
/// let command = Command::parse("/num 333", "");
/// assert!(command.is_err());
/// # }
/// ```
///
///  4. `#[command(prefix = "prefix")]`
///  5. `#[command(separator = "sep")]`
///
/// These attributes just override the corresponding `enum` attributes for a
/// specific variant.
///
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`BotCommand`]: crate::utils::command::BotCommand
pub trait BotCommand: Sized {
    fn descriptions() -> String;
    fn parse<N>(s: &str, bot_name: N) -> Result<Self, ParseError>
    where
        N: Into<String>;
    fn ty() -> PhantomData<Self> {
        PhantomData
    }
    fn bot_commands() -> Vec<crate::types::BotCommand>;
}

pub type PrefixedBotCommand = String;
pub type BotName = String;

/// Errors returned from [`BotCommand::parse`].
///
/// [`BotCommand::parse`]: crate::utils::command::BotCommand::parse
#[derive(Debug)]
pub enum ParseError {
    TooFewArguments {
        expected: usize,
        found: usize,
        message: String,
    },
    TooManyArguments {
        expected: usize,
        found: usize,
        message: String,
    },

    /// Redirected from [`FromStr::from_str`].
    ///
    /// [`FromStr::from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
    IncorrectFormat(Box<dyn Error + Send + Sync + 'static>),

    UnknownCommand(PrefixedBotCommand),
    WrongBotName(BotName),

    /// A custom error which you can return from your custom parser.
    Custom(Box<dyn Error + Send + Sync + 'static>),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::TooFewArguments { expected, found, message } => write!(
                f,
                "Too few arguments (expected {}, found {}, message = '{}')",
                expected, found, message
            ),
            ParseError::TooManyArguments { expected, found, message } => write!(
                f,
                "Too many arguments (expected {}, found {}, message = '{}')",
                expected, found, message
            ),
            ParseError::IncorrectFormat(e) => write!(f, "Incorrect format of command args: {}", e),
            ParseError::UnknownCommand(e) => write!(f, "Unknown command: {}", e),
            ParseError::WrongBotName(n) => write!(f, "Wrong bot name: {}", n),
            ParseError::Custom(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parses a string into a command with args.
///
/// This function is just a shortcut for calling [`parse_command_with_prefix`]
/// with the default prefix `/`.
///
/// ## Example
/// ```
/// use teloxide::utils::command::parse_command;
///
/// let text = "/mute@my_admin_bot 5 hours";
/// let (command, args) = parse_command(text, "my_admin_bot").unwrap();
/// assert_eq!(command, "mute");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
///
/// If the name of a bot does not match, it will return `None`:
/// ```
/// use teloxide::utils::command::parse_command;
///
/// let result = parse_command("/ban@MyNameBot1 3 hours", "MyNameBot2");
/// assert!(result.is_none());
/// ```
///
/// [`parse_command_with_prefix`]:
/// crate::utils::command::parse_command_with_prefix
pub fn parse_command<N>(text: &str, bot_name: N) -> Option<(&str, Vec<&str>)>
where
    N: AsRef<str>,
{
    parse_command_with_prefix("/", text, bot_name)
}

/// Parses a string into a command with args (custom prefix).
///
/// `prefix`: symbols, which denote start of a command.
///
/// ## Example
/// ```
/// use teloxide::utils::command::parse_command_with_prefix;
///
/// let text = "!mute 5 hours";
/// let (command, args) = parse_command_with_prefix("!", text, "").unwrap();
/// assert_eq!(command, "mute");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
///
/// If the name of a bot does not match, it will return `None`:
/// ```
/// use teloxide::utils::command::parse_command_with_prefix;
///
/// let result = parse_command_with_prefix("!", "!ban@MyNameBot1 3 hours", "MyNameBot2");
/// assert!(result.is_none());
/// ```
pub fn parse_command_with_prefix<'a, N>(
    prefix: &str,
    text: &'a str,
    bot_name: N,
) -> Option<(&'a str, Vec<&'a str>)>
where
    N: AsRef<str>,
{
    if !text.starts_with(prefix) {
        return None;
    }
    let mut words = text.split_whitespace();
    let mut splited = words.next()?[prefix.len()..].split('@');
    let command = splited.next()?;
    let bot = splited.next();
    match bot {
        Some(name) if name == bot_name.as_ref() => {}
        None => {}
        _ => return None,
    }
    Some((command, words.collect()))
}

// The rest of tests are integrational due to problems with macro expansion in
// unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_with_args_() {
        let data = "/command arg1 arg2";
        let expected = Some(("command", vec!["arg1", "arg2"]));
        let actual = parse_command(data, "");
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_command_with_args_without_args() {
        let data = "/command";
        let expected = Some(("command", vec![]));
        let actual = parse_command(data, "");
        assert_eq!(actual, expected)
    }
}
