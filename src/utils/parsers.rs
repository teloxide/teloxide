pub use teloxide_macros::TelegramBotCommand;
/// Enum for telegram commands
///
/// Example:
/// ```
/// use teloxide::utils::{parse_command_into_enum, TelegramBotCommand};
/// #[derive(TelegramBotCommand, PartialEq, Debug)]
/// enum TelegramAdminCommand {
///     Ban,
///     Kick,
/// }
/// let (command, args) =
///     parse_command_into_enum::<TelegramAdminCommand>("/ban 5 h").unwrap();
/// assert_eq!(command, TelegramAdminCommand::Ban);
/// assert_eq!(args, vec!["5", "h"]);
/// ```
pub trait TelegramBotCommand: Sized {
    fn try_from(s: &str) -> Option<Self>;
    fn descriptions() -> String;
}

/// Function to parse message with command into enum. Command must started with
/// `/`
///
/// Example:
/// ```
/// use teloxide::utils::{parse_command_into_enum, TelegramBotCommand};
/// #[derive(TelegramBotCommand, PartialEq, Debug)]
/// enum TelegramAdminCommand {
///     Ban,
///     Kick,
/// }
/// let (command, args) =
///     parse_command_into_enum::<TelegramAdminCommand>("/ban 5 h").unwrap();
/// assert_eq!(command, TelegramAdminCommand::Ban);
/// assert_eq!(args, vec!["5", "h"]);
/// ```
pub fn parse_command_into_enum<T>(text: &str) -> Option<(T, Vec<&str>)>
where
    T: TelegramBotCommand,
{
    let (command, args) = parse_command(text)?;
    match T::try_from(command) {
        Some(command) => Some((command, args)),
        _ => None,
    }
}

/// Function which parse string and return command with args. It calls
/// [`parse_command_with_prefix`] with default prefix `/`
///
/// Example:
/// ```
/// use teloxide::utils::parse_command;
/// let text = "/ban 5 hours";
/// let (command, args) = parse_command(text).unwrap();
/// assert_eq!(command, "ban");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
pub fn parse_command(text: &str) -> Option<(&str, Vec<&str>)> {
    let mut words = text.split_whitespace();
    let command = words.next()?;
    Some((command, words.collect()))
}

/// Function which parse string and return command with args. Prefix - start
/// symbols which denote start of command
///
/// Example:
/// ```
/// use teloxide::utils::parse_command_with_prefix;
/// let text = "!ban 5 hours";
/// let (command, args) = parse_command_with_prefix("!", text).unwrap();
/// assert_eq!(command, "ban");
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
        #[derive(TelegramBotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            Start,
            Help,
        }

        let data = "/start arg1 arg2";
        let expected = Some((DefaultCommands::Start, vec!["arg1", "arg2"]));
        let actual = parse_command_into_enum::<DefaultCommands>(data);
        assert_eq!(actual, expected)
    }

    #[test]
    fn attribute_prefix() {
        #[derive(TelegramBotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            #[command(prefix = "!")]
            Start,
            Help,
        }

        let data = "!start arg1 arg2";
        let expected = Some((DefaultCommands::Start, vec!["arg1", "arg2"]));
        let actual = parse_command_into_enum::<DefaultCommands>(data);
        assert_eq!(actual, expected)
    }
    
    #[test]
    fn many_attributes() {
        #[derive(TelegramBotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            #[command(prefix = "!", description = "desc")]
            Start,
            Help,
        }

        assert_eq!(DefaultCommands::Start, parse_command_into_enum::<DefaultCommands>("!start").unwrap().0);
        assert_eq!(DefaultCommands::descriptions(), "!start - desc\n/help - \n");
    }

    #[test]
    fn global_attributes() {
        #[command(prefix = "!")]
        #[derive(TelegramBotCommand, Debug, PartialEq)]
        enum DefaultCommands {
            #[command(prefix = "/")]
            Start,
            Help,
        }

        assert_eq!(DefaultCommands::Start, parse_command_into_enum::<DefaultCommands>("/start").unwrap().0);
        assert_eq!(DefaultCommands::Help, parse_command_into_enum::<DefaultCommands>("!help").unwrap().0);
        assert_eq!(DefaultCommands::descriptions(), "/start - \n!help - \n");
    }
}
