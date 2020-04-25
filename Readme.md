# teloxide-macros
The teloxide's procedural macros.

## Example
```rust
use teloxide::utils::command::BotCommand;
#[derive(BotCommand, PartialEq, Debug)]
#[command(rename = "lowercase")]
enum AdminCommand {
    Mute,
    Ban,
}
let (command, args) = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
assert_eq!(command, AdminCommand::Ban);
assert_eq!(args, vec!["5", "h"]);
```
## Enum attributes
 1. `#[command(rename = "rule")]`
Rename all commands by rule. Allowed rules are `lowercase`. If you will not
use this attribute, commands will be parsed by their original names.

 2. `#[command(prefix = "prefix")]`
Change a prefix for all commands (the default is `/`).

 3. `#[command(description = "description")]`
Add a sumary description of commands before all commands.

## Variant attributes
 1. `#[command(rename = "rule")]`
Rename one command by a rule. Allowed rules are `lowercase`, `%some_name%`,
where `%some_name%` is any string, a new name.

 2. `#[command(prefix = "prefix")]`
Change a prefix for one command (the default is `/`).

 3. `#[command(description = "description")]`
Add a description of one command.

All variant attributes overlap the `enum` attributes.