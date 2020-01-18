pub use teloxide_macros::TelegramCommandEnum;
/// Enum for telegram commands
///
/// Example:
/// ```
/// use teloxide::utils::TelegramCommandEnum;
/// use teloxide::utils::parse_command_into_enum;
/// #[derive(TelegramCommandEnum, PartialEq, Debug)]
/// enum TelegramCommand {
///     Start,
///     Help,
/// }
/// let (command, args) = parse_command_into_enum::<TelegramCommand>("/", "/start arg1 arg2").unwrap();
/// assert_eq!(command, TelegramCommand::Start);
/// assert_eq!(args, vec!["arg1", "arg2"]);
/// ```
pub trait TelegramCommandEnum: Sized {
    fn try_from(s: &str) -> Option<Self>;
}

pub fn parse_command_into_enum<'a, T>(
    prefix: &str,
    text: &'a str,
) -> Option<(T, Vec<&'a str>)>
where
    T: TelegramCommandEnum,
{
    let (command, args) = parse_command(prefix,text)?;
    return match T::try_from(command) {
        Some(command) => Some((command, args)),
        _ => None,
    };
}

/// Function which parse string and return command with args
///
/// Example:
/// ```
/// use teloxide::utils::parse_command;
/// let text = "/ban 5 hours";
/// let (command, args) = parse_command("/", text).unwrap();
/// assert_eq!(command, "ban");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
pub fn parse_command<'a>(prefix: &str, text: &'a str) -> Option<(&'a str, Vec<&'a str>)> {
    if !text.starts_with(prefix) {
        return None;
    }
    let mut words = text.split_whitespace();
    let command = &words.next()?[1..];
    Some((command, words.collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_with_args_() {
        let data = "/command arg1 arg2";
        let expected = Some(("command", vec!["arg1", "arg2"]));
        let actual = parse_command("/", data);
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_command_with_args_without_args() {
        let data = "/command";
        let expected = Some(("command", vec![]));
        let actual = parse_command("/", data);
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_command__with_args() {
        #[derive(TelegramCommandEnum, Debug, PartialEq)]
        enum DefaultCommands {
            Start,
            Help,
        }

        let data = "/start arg1 arg2";
        let expected = Some((DefaultCommands::Start, vec!["arg1", "arg2"]));
        let actual = parse_command_into_enum::<DefaultCommands>("/", data);
        assert_eq!(actual, expected)
    }
}
