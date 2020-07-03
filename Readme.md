# teloxide-macros
The teloxide's procedural macros.

# Example
```rust
use teloxide::utils::command::BotCommand;

type UnitOfTime = u8;

#[derive(BotCommand, PartialEq, Debug)]
#[command(rename = "lowercase", parse_with = "split")]
enum AdminCommand {
    Mute(UnitOfTime, char),
    Ban(UnitOfTime, char),
}

let command = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
assert_eq!(command, AdminCommand::Ban(5, 'h'));
```

## Enum attributes
 1. `#[command(rename = "rule")]`
Rename all commands by `rule`. Allowed rules are `lowercase`. If you will
not use this attribute, commands will be parsed by their original names.

 2. `#[command(prefix = "prefix")]`
Change a prefix for all commands (the default is `/`).

 3. `#[command(description = "description")]`
Add a sumary description of commands before all commands.

 4. `#[command(parse_with = "parser")]`
Change the parser of arguments. Possible values:
   - `default` - the same as the unspecified parser. It only puts all text
   after the first space into the first argument, which must implement
   [`FromStr`].

### Example
```rust
use teloxide::utils::command::BotCommand;

#[derive(BotCommand, PartialEq, Debug)]
#[command(rename = "lowercase")]
enum Command {
    Text(String),
}

let command = Command::parse("/text hello my dear friend!", "").unwrap();
assert_eq!(command, Command::Text("hello my dear friend!".to_string()));
```

 - `split` - separates a messsage by a given separator (the default is the
   space character) and parses each part into the corresponding arguments,
   which must implement [`FromStr`].

### Example
```rust
use teloxide::utils::command::BotCommand;

#[derive(BotCommand, PartialEq, Debug)]
#[command(rename = "lowercase", parse_with = "split")]
enum Command {
    Nums(u8, u16, i32),
}

let command = Command::parse("/nums 1 32 -5", "").unwrap();
assert_eq!(command, Command::Nums(1, 32, -5));
```

5. `#[command(separator = "sep")]`
Specify separator used by the `split` parser. It will be ignored when
accompanied by another type of parsers.

### Example
```rust
use teloxide::utils::command::BotCommand;

#[derive(BotCommand, PartialEq, Debug)]
#[command(rename = "lowercase", parse_with = "split", separator = "|")]
enum Command {
    Nums(u8, u16, i32),
}

let command = Command::parse("/nums 1|32|5", "").unwrap();
assert_eq!(command, Command::Nums(1, 32, 5));
```

## Variant attributes
All variant attributes override the corresponding `enum` attributes.

 1. `#[command(rename = "rule")]`
Rename one command by a rule. Allowed rules are `lowercase`, `%some_name%`,
where `%some_name%` is any string, a new name.

 2. `#[command(parse_with = "parser")]`
One more option is available for variants.
   - `custom_parser` - your own parser of the signature `fn(String) ->
   Result<Tuple, ParseError>`, where `Tuple` corresponds to the variant's
arguments.

### Example
```rust
use teloxide::utils::command::{BotCommand, ParseError};

fn accept_two_digits(input: String) -> Result<(u8,), ParseError> {
    match input.len() {
        2 => {
            let num = input
                .parse::<u8>()
                .map_err(|e| ParseError::IncorrectFormat(e.into()))?;
            Ok((num,))
        }
        len => Err(ParseError::Custom(
            format!("Only 2 digits allowed, not {}", len).into(),
        )),
    }
}

#[derive(BotCommand, PartialEq, Debug)]
#[command(rename = "lowercase")]
enum Command {
    #[command(parse_with = "accept_two_digits")]
    Num(u8),
}

let command = Command::parse("/num 12", "").unwrap();
assert_eq!(command, Command::Num(12));
let command = Command::parse("/num 333", "");
assert!(command.is_err());
```

 3. `#[command(prefix = "prefix")]`
 4. `#[command(description = "description")]`
 5. `#[command(separator = "sep")]`

Analogous to the descriptions above.

[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[`BotCommand`]: crate::utils::command::BotCommand
