# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2020-02-25
### Added
 - The functionality to parse commands only with a correct bot's name (breaks backwards compatibility) ([Issue 168](https://github.com/teloxide/teloxide/issues/168)).
 - This `CHANGELOG.md`.

### Fixed
 - Fix parsing a pinned message ([Issue 167](https://github.com/teloxide/teloxide/issues/167)).
 - Replace `LanguageCode` with `String`, because [the official Telegram documentation](https://core.telegram.org/bots/api#getchat) doesn't specify a concrete version of IETF language tag.
 - Problems with the `poll_type` field ([Issue 178](https://github.com/teloxide/teloxide/issues/178)).
 - Make `polling_default` actually a long polling update listener ([PR 182](https://github.com/teloxide/teloxide/pull/182)).

### Removed
 - [either](https://crates.io/crates/either) from the dependencies in `Cargo.toml`.
 - `teloxide-macros` migrated into [the separate repository](https://github.com/teloxide/teloxide-macros) to easier releases and testing.

## [0.1.0] - 2020-02-19
### Added
 - This project.
