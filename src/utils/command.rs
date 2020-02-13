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
//! #[command(rename = "lowercase")]
//! enum AdminCommand {
//!     Kick,
//!     Ban,
//! }
//!
//! let (command, args) = AdminCommand::parse("/ban 3 hours").unwrap();
//! assert_eq!(command, AdminCommand::Ban);
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! Using [`parse_command`]:
//! ```
//! use teloxide::utils::command::parse_command;
//!
//! let (command, args) = parse_command("/ban 3 hours").unwrap();
//! assert_eq!(command, "/ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! Using [`parse_command_with_prefix`]:
//! ```
//! use teloxide::utils::command::parse_command_with_prefix;
//!
//! let text = "!ban 3 hours";
//! let (command, args) = parse_command_with_prefix("!", text).unwrap();
//! assert_eq!(command, "ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! See [examples/admin_bot] as a more complicated examples.
//!
//! [`parse_command`]: crate::utils::command::parse_command
//! [`parse_command_with_prefix`]:
//! crate::utils::command::parse_command_with_prefix
//! [examples/admin_bot]: https://github.com/teloxide/teloxide/blob/dev/examples/miltiple_handlers_bot/

pub use teloxide_macros::BotCommand;

/// An enumeration of bot's commands.
///
/// ## Example
/// ```
/// use teloxide::utils::command::BotCommand;
///
/// #[derive(BotCommand, PartialEq, Debug)]
/// #[command(rename = "lowercase")]
/// enum AdminCommand {
///     Mute,
///     Ban,
/// }
///
/// let (command, args) = AdminCommand::parse("/ban 5 h").unwrap();
/// assert_eq!(command, AdminCommand::Ban);
/// assert_eq!(args, vec!["5", "h"]);
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
/// All variant attributes overlap the `enum` attributes.
pub trait BotCommand: Sized {
    fn try_from(s: &str) -> Option<Self>;
    fn descriptions() -> String;
    fn parse(s: &str) -> Option<(Self, Vec<&str>)>;
}

/// Parses a string into a command with args.
///
/// It calls [`parse_command_with_prefix`] with default prefix `/`.
///
/// ## Example
/// ```
/// use teloxide::utils::command::parse_command;
///
/// let text = "/mute 5 hours";
/// let (command, args) = parse_command(text).unwrap();
/// assert_eq!(command, "/mute");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
pub fn parse_command(text: &str) -> Option<(&str, Vec<&str>)> {
    let mut words = text.split_whitespace();
    let command = words.next()?;
    Some((command, words.collect()))
}

/// Parses a string into a command with args (custom prefix).
///
/// `prefix`: start symbols which denote start of a command.
///
/// Example:
/// ```
/// use teloxide::utils::command::parse_command_with_prefix;
///
/// let text = "!mute 5 hours";
/// let (command, args) = parse_command_with_prefix("!", text).unwrap();
/// assert_eq!(command, "mute");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
pub fn parse_command_with_prefix<'a>(
    prefix: &str,
    text: &'a str,
) -> Option<(&'a str, Vec<&'a str>)> {
    if !text.starts_with(prefix) {
        return None;
    }
    let mut words = text.split_whitespace();
    let command = &words.next()?[prefix.len()..];
    Some((command, words.collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_with_args_() {
        let data = "/command arg1 arg2";
        let expected = Some(("/command", vec!["arg1", "arg2"]));
        let actual = parse_command(data);
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_command_with_args_without_args() {
        let data = "/command";
        let expected = Some(("/command", vec![]));
        let actual = parse_command(data);
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_command_with_args() {
        #[command(rename = "lowercase")]
        #[derive(BotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            Start,
            Help,
        }

        let data = "/start arg1 arg2";
        let expected = Some((DefaultCommands::Start, vec!["arg1", "arg2"]));
        let actual = DefaultCommands::parse(data);
        assert_eq!(actual, expected)
    }

    #[test]
    fn attribute_prefix() {
        #[command(rename = "lowercase")]
        #[derive(BotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            #[command(prefix = "!")]
            Start,
            Help,
        }

        let data = "!start arg1 arg2";
        let expected = Some((DefaultCommands::Start, vec!["arg1", "arg2"]));
        let actual = DefaultCommands::parse(data);
        assert_eq!(actual, expected)
    }

    #[test]
    fn many_attributes() {
        #[command(rename = "lowercase")]
        #[derive(BotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            #[command(prefix = "!", description = "desc")]
            Start,
            Help,
        }

        assert_eq!(
            DefaultCommands::Start,
            DefaultCommands::parse("!start").unwrap().0
        );
        assert_eq!(
            DefaultCommands::descriptions(),
            "!start - desc\n/help - \n"
        );
    }

    #[test]
    fn global_attributes() {
        #[command(
            prefix = "!",
            rename = "lowercase",
            description = "Bot commands"
        )]
        #[derive(BotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            #[command(prefix = "/")]
            Start,
            Help,
        }

        assert_eq!(
            DefaultCommands::Start,
            DefaultCommands::parse("/start").unwrap().0
        );
        assert_eq!(
            DefaultCommands::Help,
            DefaultCommands::parse("!help").unwrap().0
        );
        assert_eq!(
            DefaultCommands::descriptions(),
            "Bot commands\n/start - \n!help - \n"
        );
    }
}
