//! Command parsers.
//!
//! You can either create an `enum`, containing commands of your bot, or use
//! functions, which split input text into a string command with its arguments.
//!
//! ## Examples
//! Using `enum`:
//! ```
//! use teloxide::utils::command::BotCommand;
//!
//! #[derive(BotCommand, PartialEq, Debug)]
//! #[command(rename = "lowercase", parser = "split")]
//! enum AdminCommand {
//!     Mute(u8, char),
//!     Ban(u8, char),
//! }
//!
//! let command = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
//! assert_eq!(command, AdminCommand::Ban(5, 'h'));
//! ```
//!
//! Using [`parse_command`]:
//! ```
//! use teloxide::utils::command::parse_command;
//!
//! let (command, args) =
//!     parse_command("/ban@MyBotName 3 hours", "MyBotName").unwrap();
//! assert_eq!(command, "ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! Using [`parse_command_with_prefix`]:
//! ```
//! use teloxide::utils::command::parse_command_with_prefix;
//!
//! let text = "!ban 3 hours";
//! let (command, args) = parse_command_with_prefix("!", text, "").unwrap();
//! assert_eq!(command, "ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! If the name of a bot does not match, it will return `None`:
//! ```
//! use teloxide::utils::command::parse_command;
//!
//! let result = parse_command("/ban@MyNameBot1 3 hours", "MyNameBot2");
//! assert!(result.is_none());
//! ```
//!
//! See [examples/admin_bot] as a more complicated examples.
//!
//! [`parse_command`]: crate::utils::command::parse_command
//! [`parse_command_with_prefix`]:
//! crate::utils::command::parse_command_with_prefix
//! [examples/admin_bot]: https://github.com/teloxide/teloxide/blob/master/examples/miltiple_handlers_bot/

pub use teloxide_macros::BotCommand;

/// An enumeration of bot's commands.
///
/// ## Example
/// ```
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase", parser = "split")]
/// enum AdminCommand {
///     Mute(u8, char),
///     Ban(u8, char),
/// }
///
/// let command = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
/// assert_eq!(command, AdminCommand::Ban(5, 'h'));
/// ```
///
/// ## Enum attributes
///  1. `#[command(rename = "rule")]`
/// Rename all commands by rule. Allowed rules are `lowercase`. If you will not
/// use this attribute, commands will be parsed by their original names.
///
///  2. `#[command(prefix = "prefix")]`
/// Change a prefix for all commands (the default is `/`).
///
///  3. `#[command(description = "description")]`
/// Add a sumary description of commands before all commands.
///
///  4. `#[command(parser = "parser")]`
/// Change the parser of arguments. Possible values:
///  - `default` - it also will be used if `parser` attribute will not be
///    specified.
/// It can only put all text after first space into first argument, which
/// implement FromStr trait.
/// Example:
/// ```
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase")]
/// enum Command {
///     Text(String),
/// }
///
/// let command =
///     AdminCommand::parse("/text hello my dear friend!", "").unwrap();
/// assert_eq!(command, Command::Text("hello my dear friend!".to_string()));
/// ```
///  - `split` - parse args by split incoming text by value specified in
///    `separator`
///   attribute. By default use space seperator. All args must implement FromStr
/// trait. Example:
/// ```
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase", parser = "split")]
/// enum Command {
///     Nums(u8, u16, i32),
/// }
///
/// let command = AdminCommand::parse("/nums 1 32 -5", "").unwrap();
/// assert_eq!(command, Command::Nums(1, 32, -5));
/// ```
///  - `custom_parser` - you can use your own parser, which must used signature
///    `Fn(String) -> Result<Tuple, ParseError>`
/// where `Tuple` - tuple with all fields in type. Allowed only on variant.
/// Example:
/// ```
/// use teloxide::utils::command::{BotCommand, ParseError};
///
/// fn accept_two_digits(input: String) -> Result<(u8), ParseError> {
///     match input.len() {
///         2 => {
///             let num =
///                 input.parse().map_err(|_| ParseError::IncorrectFormat)?;
///             Ok((num))
///         }
///         len => Err(ParseError::Custom(format!(
///             "Only 2 digits allowed, not {}",
///             len
///         ))),
///     }
/// }
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase", parser = "split")]
/// enum Command {
///     Num(u8),
/// }
///
/// let command = Command::parse("/num 12", "").unwrap();
/// assert_eq!(command, Command::Num(12));
/// let command = Command::parse("/num 333", "");
/// assert!(command.is_err());
/// ```
///
/// 5. `#[command(separator = "sep")]`
/// Specify separator used by `split` parser. Will be ignored when used another
/// types of parser.
/// Example:
/// ```
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase", parser = "split", separator = "|")]
/// enum Command {
///     Nums(u8, u16, i32),
/// }
///
/// let command = AdminCommand::parse("/nums 1|32|5", "").unwrap();
/// assert_eq!(command, Command::Nums(1, 32, 5));
/// ```
///
/// ## Variant attributes
///  1. `#[command(rename = "rule")]`
/// Rename one command by a rule. Allowed rules are `lowercase`, `%some_name%`,
/// where `%some_name%` is any string, a new name.
///
///  2. `#[command(prefix = "prefix")]`
/// Change a prefix for one command (the default is `/`).
///
///  3. `#[command(description = "description")]`
/// Add a description of one command.
///
///  4. `#[command(parser = "parser")]`
/// See description above.
///
/// 5. `#[command(separator = "sep")]`
/// See description above.
///
/// All variant attributes overlap the `enum` attributes.
pub trait BotCommand: Sized {
    fn descriptions() -> String;
    fn parse<N>(s: &str, bot_name: N) -> Result<Self, ParseError>
    where
        N: Into<String>;
}

/// Error returned from `BotCommand::parse` method.
#[derive(Debug)]
pub enum ParseError {
    /// This error was returned when count of arguments will be less than
    /// expected count.
    TooFewArguments { expected: usize, found: usize, message: String },
    /// This error was returned when count of arguments will be greater than
    /// expected count.
    TooManyArguments { expected: usize, found: usize, message: String },
    /// This error was returned when error from `FromStr::from_str` was
    /// occured.
    IncorrectFormat,
    /// This error was returned when input command does not represent in list
    /// of commands.
    UnknownCommand(String),
    /// This error was returned when command bot name is different from
    /// expected bot name.
    WrongBotName(String),
    /// Custom error which you can return from custom parser.
    Custom(String),
}

/// Parses a string into a command with args.
///
/// It calls [`parse_command_with_prefix`] with the default prefix `/`.
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
