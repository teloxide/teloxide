# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## unreleased

### Added

- Now you can use `#[command(command_separator="sep")]` (default is a whitespace character) to set the separator between command and its arguments ([issue #897](https://github.com/teloxide/teloxide/issues/897))
- Now you can use `/// doc comment` for the command help message ([PR #861](https://github.com/teloxide/teloxide/pull/861)).
- Now you can use `#[command(hide)]` to hide a command from the help message ([PR #862](https://github.com/teloxide/teloxide/pull/862))
- `#[command(alias = "...")]` and `#[command(aliases = "...")]` to specify command aliases ([PR #937](https://github.com/teloxide/teloxide/pull/937))
- `#[command(hide_aliases)]` to hide aliases from the help message ([PR #937](https://github.com/teloxide/teloxide/pull/937))

### Fixed

- Fix `split` parser for tuple variants with len < 2 ([issue #834](https://github.com/teloxide/teloxide/issues/834))

### Changed

- MSRV (Minimal Supported Rust Version) was bumped from `1.64.0` to `1.68.0` ([PR 950][https://github.com/teloxide/teloxide/pull/950])

### Deprecated

- `off` in `#[command(description = "off")]` is deprecated in favour of `#[command(hide)]`

## 0.7.1 - 2023-01-17

### Fixed

- Use fully qualified names in macros

## 0.7.0 - 2022-10-06

### Removed

- `derive(DialogueState)` macro

### Changed

- `#[command(rename = "...")]` now always renames to `"..."`; to rename multiple commands using the same pattern, use `#[command(rename_rule = "snake_case")]` and the like.
- `#[command(parse_with = ...)]` now requires a path, instead of a string, when specifying custom parsers.

### Fixed

- `#[derive(BotCommands)]` even if the trait is not imported ([issue #717](https://github.com/teloxide/teloxide/issues/717)).

## 0.6.3 - 2022-07-19

### Fixed

 - Allow specifying a path to a command handler in `parse_with` ([PR #27](https://github.com/teloxide/teloxide-macros/pull/27)).

## 0.6.2 - 2022-05-27

### Fixed

 - Fix `#[command(rename = "...")]` for custom command names ([issue 633](https://github.com/teloxide/teloxide/issues/633)).

## 0.6.1 - 2022-04-26

### Fixed

 - Fix `#[derive(DialogueState)]` (function return type `dptree::Handler`).

## 0.6.0 - 2022-04-09

### Removed

 - Support for the old dispatching: `#[teloxide(subtransition)]` [**BC**].

### Deprecated

 - `#[derive(DialogueState)]` in favour of `teloxide::handler!`.

## 0.5.1 - 2022-03-23

### Fixed

 - Make bot name check case-insensitive ([PR #16](https://github.com/teloxide/teloxide-macros/pull/16)).

### Added

 - More command rename rules: `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, and `SCREAMING-KEBAB-CASE` ([PR #18](https://github.com/teloxide/teloxide-macros/pull/18)).

## 0.5.0 - 2022-02-05

### Added

- The `BotCommand::bot_commands()` method that returns `Vec<BotCommand>` ([PR #13](https://github.com/teloxide/teloxide-macros/pull/13)).
- `#[derive(DialogueState)]`, `#[handler_out(...)]`, `#[handler(...)]`.

## 0.4.1 - 2021-07-11

### Fixed

 - Fix generics support for a variant's arguments ([PR #8](https://github.com/teloxide/teloxide-macros/issues/8)).

## 0.4.0 - 2021-03-19

### Changed

 - Adjust dialogues with the latest teloxide (v0.4.0).

## 0.3.2 - 2020-07-27

### Added
 - `#[derive(Transition)]` with `#[teloxide(subtransition)]`.

### Removed
 - The `dev` branch.

## 0.3.1 - 2020-07-04

### Added
 - Now you can remove command from showing in descriptions by defining `description` attribute as `"off"`.

## 0.3.0 - 2020-07-03

### Changed
 - The description in `Cargo.toml` was changed to from "The teloxide's macros for internal usage" to "The teloxide's procedural macros".
 - Now parsing of arguments happens using special function. There are 3 possible variants:
   - Using `default` parser, which only put all text in one String field.
   - Using `split` parser, which split all text by `separator` (by default is whitespace) and then use FromStr::from_str to construct value.
   - Using custom separator.
 - Now function `parse` return Result<T, ParseError> instead of Option<T>.

### Added
 - This `CHANGELOG.md`.
 - `.gitignore`.
 - `#[parse_with]` attribute.
 - `#[separator='%sep%']` attribute.

## 0.2.1 - 2020-02-25

### Changed
 - The description in `Cargo.toml` was changed to from "The teloxide's macros for internal usage" to "The teloxide's procedural macros".

### Added
 - This `CHANGELOG.md`.
 - `.gitignore`.
 - The functionality to parse commands only with a correct bot's name (breaks backwards compatibility).

## 0.1.2 - 2020-02-24

### Changed
 - The same as v0.1.1, but fixes [the issue](https://github.com/teloxide/teloxide/issues/176) about backwards compatibility.


## 0.2.0 - YANKED

### Changed
 - Fixes [the issue](https://github.com/teloxide/teloxide/issues/176) about backwards compatibility, but fairly soon I realised that semver recommends to use v0.1.2 instead.


## 0.1.1 - 2020-02-23

### Added
 - The `LICENSE` file.

### Changed
 - Backwards compatibility is broken and was fixed in v0.1.2.


## 0.1.0 - 2020-02-19

### Added
 - This project.
