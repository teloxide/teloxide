use teloxide::utils::command::{BotCommand, ParseError};

// We put tests here because macro expand in unit tests in module
// teloxide::utils::command was a failure

#[test]
fn parse_command_with_args() {
    #[command(rename = "lowercase")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        Start(String),
        Help,
    }

    let data = "/start arg1 arg2";
    let expected = DefaultCommands::Start("arg1 arg2".to_string());
    let actual = DefaultCommands::parse(data, "").unwrap();
    assert_eq!(actual, expected)
}

#[test]
fn parse_command_with_non_string_arg() {
    #[command(rename = "lowercase")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        Start(i32),
        Help,
    }

    let data = "/start -50";
    let expected = DefaultCommands::Start("-50".parse().unwrap());
    let actual = DefaultCommands::parse(data, "").unwrap();
    assert_eq!(actual, expected)
}


#[test]
fn attribute_prefix() {
    #[command(rename = "lowercase")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        #[command(prefix = "!")]
        Start(String),
        Help,
    }

    let data = "!start arg1 arg2";
    let expected = DefaultCommands::Start("arg1 arg2".to_string());
    let actual = DefaultCommands::parse(data, "").unwrap();
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

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("!start", "").unwrap());
    assert_eq!(DefaultCommands::descriptions(), "!start - desc\n/help\n");
}

#[test]
fn global_attributes() {
    #[command(prefix = "!", rename = "lowercase", description = "Bot commands")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        #[command(prefix = "/")]
        Start,
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "MyNameBot").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("!help", "MyNameBot").unwrap());
    assert_eq!(DefaultCommands::descriptions(), "Bot commands\n/start\n!help\n");
}

#[test]
fn parse_command_with_bot_name() {
    #[command(rename = "lowercase")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        #[command(prefix = "/")]
        Start,
        Help,
    }

    assert_eq!(
        DefaultCommands::Start,
        DefaultCommands::parse("/start@MyNameBot", "MyNameBot").unwrap()
    );
}

#[test]
fn parse_with_split() {
    #[command(rename = "lowercase")]
    #[command(parse_with = "split")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10 hello", "").unwrap()
    );
}

#[test]
fn parse_with_split2() {
    #[command(rename = "lowercase")]
    #[command(parse_with = "split", separator = "|")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10|hello", "").unwrap()
    );
}

#[test]
fn parse_custom_parser() {
    fn custom_parse_function(s: String) -> Result<(u8, String), ParseError> {
        let vec = s.split_whitespace().collect::<Vec<_>>();
        let (left, right) = match vec.as_slice() {
            [l, r] => (l, r),
            _ => return Err(ParseError::IncorrectFormat("might be 2 arguments!".into())),
        };
        left.parse::<u8>()
            .map(|res| (res, (*right).to_string()))
            .map_err(|_| ParseError::Custom("First argument must be a integer!".to_owned().into()))
    }

    #[command(rename = "lowercase")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        #[command(parse_with = "custom_parse_function")]
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10 hello", "").unwrap()
    );
}

#[test]
fn parse_named_fields() {
    #[command(rename = "lowercase")]
    #[command(parse_with = "split")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        Start { num: u8, data: String },
        Help,
    }

    assert_eq!(
        DefaultCommands::Start { num: 10, data: "hello".to_string() },
        DefaultCommands::parse("/start 10 hello", "").unwrap()
    );
}

#[test]
fn descriptions_off() {
    #[command(rename = "lowercase")]
    #[derive(BotCommand, Debug, PartialEq)]
    enum DefaultCommands {
        #[command(description = "off")]
        Start,
        Help,
    }

    assert_eq!(DefaultCommands::descriptions(), "/help\n".to_owned());
}
