// https://github.com/rust-lang/rust-clippy/issues/7422
#![allow(clippy::nonstandard_macro_braces)]

#[cfg(feature = "macros")]
use teloxide::utils::command::BotCommands;

// We put tests here because macro expand in unit tests in module
// teloxide::utils::command was a failure

#[test]
#[cfg(feature = "macros")]
fn parse_command_with_args() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
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
#[cfg(feature = "macros")]
fn parse_command_with_non_string_arg() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
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
#[cfg(feature = "macros")]
fn attribute_prefix() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
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
#[cfg(feature = "macros")]
fn many_attributes() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(prefix = "!", description = "desc")]
        Start,
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("!start", "").unwrap());
    assert_eq!(DefaultCommands::descriptions().to_string(), "!start — desc\n/help");
}

#[test]
#[cfg(feature = "macros")]
fn global_attributes() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(prefix = "!", rename_rule = "lowercase", description = "Bot commands")]
    enum DefaultCommands {
        #[command(prefix = "/")]
        Start,
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "MyNameBot").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("!help", "MyNameBot").unwrap());
    assert_eq!(DefaultCommands::descriptions().to_string(), "Bot commands\n\n/start\n!help");
}

#[test]
#[cfg(feature = "macros")]
fn parse_command_with_bot_name() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
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
#[cfg(feature = "macros")]
fn parse_with_split() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
    enum DefaultCommands {
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10 hello", "").unwrap(),
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_split2() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split", separator = "|")]
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
#[cfg(feature = "macros")]
fn parse_with_split3() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
    enum DefaultCommands {
        Start(u8),
        Help,
    }

    assert_eq!(DefaultCommands::Start(10), DefaultCommands::parse("/start 10", "").unwrap(),);
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_split4() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
    enum DefaultCommands {
        Start(),
        Help,
    }

    assert_eq!(DefaultCommands::Start(), DefaultCommands::parse("/start", "").unwrap(),);
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_command_separator1() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split", separator = "|", command_separator = "_")]
    enum DefaultCommands {
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start_10|hello", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_command_separator2() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split", separator = "_", command_separator = "_")]
    enum DefaultCommands {
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start_10_hello", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_command_separator3() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split", command_separator = ":")]
    enum DefaultCommands {
        Help,
    }

    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/help", "").unwrap());
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_command_separator4() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split", command_separator = ":")]
    enum DefaultCommands {
        Start(u8),
        Help,
    }

    assert_eq!(DefaultCommands::Start(10), DefaultCommands::parse("/start:10", "").unwrap());
}

#[test]
#[cfg(feature = "macros")]
fn parse_custom_parser() {
    mod parser {
        use teloxide::utils::command::ParseError;

        pub fn custom_parse_function(s: String) -> Result<(u8, String), ParseError> {
            let vec = s.split_whitespace().collect::<Vec<_>>();
            let (left, right) = match vec.as_slice() {
                [l, r] => (l, r),
                _ => return Err(ParseError::IncorrectFormat("might be 2 arguments!".into())),
            };
            left.parse::<u8>().map(|res| (res, (*right).to_string())).map_err(|_| {
                ParseError::Custom("First argument must be a integer!".to_owned().into())
            })
        }
    }

    use parser::custom_parse_function;

    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(parse_with = custom_parse_function)]
        Start(u8, String),

        // Test <https://github.com/teloxide/teloxide/issues/668>.
        #[command(parse_with = parser::custom_parse_function)]
        TestPath(u8, String),

        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10 hello", "").unwrap()
    );
    assert_eq!(
        DefaultCommands::TestPath(10, "hello".to_string()),
        DefaultCommands::parse("/testpath 10 hello", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_named_fields() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
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
#[cfg(feature = "macros")]
#[allow(deprecated)]
fn descriptions_off() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(hide)]
        Start,
        #[command(description = "off")]
        Username,
        /// off
        Help,
    }

    assert_eq!(DefaultCommands::descriptions().to_string(), "/help — off".to_owned());
}

#[test]
#[cfg(feature = "macros")]
fn description_with_doc_attr() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        /// Start command
        Start,
        /// Help command\nwithout replace the `\n`
        Help,
        /// Foo command
        /// with new line
        Foo,
    }

    assert_eq!(
        DefaultCommands::descriptions().to_string(),
        "/start — Start command\n/help — Help command\\nwithout replace the `\\n`\n/foo — Foo \
         command\nwith new line"
    );
}

#[test]
#[cfg(feature = "macros")]
fn description_with_doc_attr_and_command() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        /// Start command
        #[command(description = "Start command")]
        Start,
        #[command(description = "Help command\nwith new line")]
        Help,
        /// Foo command
        /// with new line
        #[command(description = "Foo command\nwith new line")]
        Foo,
    }

    assert_eq!(
        DefaultCommands::descriptions().to_string(),
        "/start — Start command\nStart command\n/help — Help command\nwith new line\n/foo — Foo \
         command\nwith new line\nFoo command\nwith new line"
    );
}

#[test]
#[cfg(feature = "macros")]
fn rename_rules() {
    #[derive(BotCommands, Debug, PartialEq)]
    enum DefaultCommands {
        #[command(rename_rule = "lowercase")]
        AaaAaa,
        #[command(rename_rule = "UPPERCASE")]
        BbbBbb,
        #[command(rename_rule = "PascalCase")]
        CccCcc,
        #[command(rename_rule = "camelCase")]
        DddDdd,
        #[command(rename_rule = "snake_case")]
        EeeEee,
        #[command(rename_rule = "SCREAMING_SNAKE_CASE")]
        FffFff,
        #[command(rename_rule = "kebab-case")]
        GggGgg,
        #[command(rename_rule = "SCREAMING-KEBAB-CASE")]
        HhhHhh,
        #[command(rename = "Bar")]
        Foo,
    }

    assert_eq!(DefaultCommands::AaaAaa, DefaultCommands::parse("/aaaaaa", "").unwrap());
    assert_eq!(DefaultCommands::BbbBbb, DefaultCommands::parse("/BBBBBB", "").unwrap());
    assert_eq!(DefaultCommands::CccCcc, DefaultCommands::parse("/CccCcc", "").unwrap());
    assert_eq!(DefaultCommands::DddDdd, DefaultCommands::parse("/dddDdd", "").unwrap());
    assert_eq!(DefaultCommands::EeeEee, DefaultCommands::parse("/eee_eee", "").unwrap());
    assert_eq!(DefaultCommands::FffFff, DefaultCommands::parse("/FFF_FFF", "").unwrap());
    assert_eq!(DefaultCommands::GggGgg, DefaultCommands::parse("/ggg-ggg", "").unwrap());
    assert_eq!(DefaultCommands::HhhHhh, DefaultCommands::parse("/HHH-HHH", "").unwrap());
    assert_eq!(DefaultCommands::Foo, DefaultCommands::parse("/Bar", "").unwrap());

    assert_eq!(
        "/aaaaaa\n/BBBBBB\n/CccCcc\n/dddDdd\n/eee_eee\n/FFF_FFF\n/ggg-ggg\n/HHH-HHH\n/Bar",
        DefaultCommands::descriptions().to_string()
    );
}

#[test]
#[cfg(feature = "macros")]
fn alias() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        #[command(alias = "s")]
        Start,
        #[command(alias = "h")]
        Help,
        #[command(alias = "привет_мир")]
        HelloWorld(String),
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "").unwrap());
    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/s", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/help", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/h", "").unwrap());
    assert_eq!(
        DefaultCommands::HelloWorld("username".to_owned()),
        DefaultCommands::parse("/hello_world username", "").unwrap()
    );
    assert_eq!(
        DefaultCommands::HelloWorld("username".to_owned()),
        DefaultCommands::parse("/привет_мир username", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn alias_help_message() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        /// Start command
        Start,
        /// Help command
        #[command(alias = "h")]
        Help,
        #[command(alias = "привет_мир")]
        HelloWorld(String),
    }

    assert_eq!(
        "/start — Start command\n/help, /h — Help command\n/hello_world, /привет_мир",
        DefaultCommands::descriptions().to_string()
    );
}

#[test]
#[cfg(feature = "macros")]
fn aliases() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        Start,
        #[command(aliases = ["h", "помощь"])]
        Help,
        #[command(aliases = ["привет_мир"])]
        HelloWorld(String),
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/help", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/h", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/помощь", "").unwrap());
    assert_eq!(
        DefaultCommands::HelloWorld("username".to_owned()),
        DefaultCommands::parse("/hello_world username", "").unwrap()
    );
    assert_eq!(
        DefaultCommands::HelloWorld("username".to_owned()),
        DefaultCommands::parse("/привет_мир username", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn aliases_help_message() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        /// Start command
        Start,
        /// Help command
        #[command(aliases = ["h", "помощь"])]
        Help,
        #[command(aliases = ["привет_мир"])]
        HelloWorld(String),
    }

    assert_eq!(
        "/start — Start command\n/help, /h, /помощь — Help command\n/hello_world, /привет_мир",
        DefaultCommands::descriptions().to_string()
    );
}

#[test]
#[cfg(feature = "macros")]
fn hide_aliases_for_unaliases_command() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        /// Start command.
        Start,
        /// Show help message.
        #[command(hide_aliases)]
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/help", "").unwrap());

    assert_eq!(
        "/start — Start command.\n/help — Show help message.",
        DefaultCommands::descriptions().to_string()
    );
}

#[test]
#[cfg(feature = "macros")]
fn hide_aliases_with_alias() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        /// Start.
        #[command(alias = "s")]
        Start,
        /// Help.
        #[command(alias = "h", hide_aliases)]
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/help", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/h", "").unwrap());

    assert_eq!("/start, /s — Start.\n/help — Help.", DefaultCommands::descriptions().to_string());
}

#[test]
#[cfg(feature = "macros")]
fn hide_command_with_aliases() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        /// Start.
        #[command(alias = "s", hide)]
        Start,
        /// Help.
        #[command(alias = "h")]
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "").unwrap());
    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/s", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/help", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/h", "").unwrap());

    assert_eq!("/help, /h — Help.", DefaultCommands::descriptions().to_string());
}

#[test]
#[cfg(feature = "macros")]
fn hide_aliases_with_aliases() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "snake_case")]
    enum DefaultCommands {
        #[command(aliases = ["s", "старт"])]
        Start,
        #[command(aliases = ["h", "помощь"], hide_aliases)]
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "").unwrap());
    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/s", "").unwrap());
    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/старт", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/help", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/h", "").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("/помощь", "").unwrap());

    assert_eq!("/start, /s, /старт\n/help", DefaultCommands::descriptions().to_string());
}

#[test]
#[cfg(feature = "macros")]
fn custom_result() {
    #[allow(dead_code)]
    type Result = ();

    #[derive(BotCommands, Debug, PartialEq)]
    enum DefaultCommands {}
}
