# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1]
### Added
 - Now you can remove command from showing in descriptions by defining `description` attribute as `"off"`.

## [0.3.0] - 2020-07-03
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

## [0.2.1] - 2020-02-25
### Changed
 - The description in `Cargo.toml` was changed to from "The teloxide's macros for internal usage" to "The teloxide's procedural macros".

### Added
 - This `CHANGELOG.md`.
 - `.gitignore`.
 - The functionality to parse commands only with a correct bot's name (breaks backwards compatibility).

## [0.1.2] - 2020-02-24
### Changed
 - The same as v0.1.1, but fixes [the issue](https://github.com/teloxide/teloxide/issues/176) about backwards compatibility.


## [0.2.0] - [YANKED]
### Changed
 - Fixes [the issue](https://github.com/teloxide/teloxide/issues/176) about backwards compatibility, but fairly soon I realised that semver recommends to use v0.1.2 instead.


## [0.1.1] - 2020-02-23
### Added
 - The `LICENSE` file.
### Changed
 - Backwards compatibility is broken and was fixed in v0.1.2.


## [0.1.0] - 2020-02-19
### Added
 - This project.
