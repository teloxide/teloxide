# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added
 - `Bot::builder` method ([PR 269](https://github.com/teloxide/teloxide/pull/269)).

## [0.3.0] - 2020-07-31
### Added
 - Support for typed bot commands ([issue 152](https://github.com/teloxide/teloxide/issues/152)).
 - `BotBuilder`, which allows setting a default `ParseMode`.
 - The `Transition`, `Subtransition`, `SubtransitionOutputType` traits.
 - A nicer approach to manage dialogues via `#[derive(Transition)]` + `#[teloxide(subtransition)]` (see [`examples/dialogue_bot`](https://github.com/teloxide/teloxide/tree/af2aa218e7bfc442ab4475023a1c661834f576fc/examples/dialogue_bot)).
 - The `redis-storage` feature -- enables the Redis support.
 - The `cbor-serializer` feature -- enables the `CBOR` serializer for dialogues.
 - The `bincode-serializer` feature -- enables the `Bincode` serializer for dialogues.
 - The `frunk` feature -- enables `teloxide::utils::UpState`, which allows mapping from a structure of `field1, ..., fieldN` to a structure of `field1, ..., fieldN, fieldN+1`.
 - Upgrade to v4.9 Telegram bots API.
 - `teloxide::utils::client_from_env` -- constructs a client from the `TELOXIDE_TOKEN` environmental variable.
 - Import `Transition`, `TransitionIn`, `TransitionOut`, `UpState` to `teloxide::prelude`.
 - Import `repl`, `commands_repl` to `teloxide`.
 - Let users inspect an unknown API error using `ApiErrorKind::Unknown(String)`. All the known API errors are placed into `KnownApiErrorKind`.
 - Setters to all the API types.
 - `teloxide::dispatching::dialogue::serializer` -- various serializers for memory storages. The `Serializer` trait, `Bincode`, `CBOR`, `JSON`.
 - `teloxide::{repl, repl_with_listener, commands_repl, commands_repl_with_listener, dialogues_repl, dialogues_repl_with_listener}`
 - `InputFile::Memory`
 - Option to hide a command from description ([issue 217](https://github.com/teloxide/teloxide/issues/217)).
 - Respect the `TELOXIDE_PROXY` environment variable in `Bot::from_env`.

### Deprecated
 - `Bot::{from_env_with_client, new, with_client}`

### Changed
 - `DialogueDispatcherHandlerCx` -> `DialogueWithCx`.
 - `DispatcherHandlerCx` -> `UpdateWithCx`.
 - Now provided description of unknown telegram error, by splitting ApiErrorKind at `ApiErrorKind` and `ApiErrorKindKnown` enums ([issue 199](https://github.com/teloxide/teloxide/issues/199)).
 - Extract `Bot` from `Arc` ([issue 216](https://github.com/teloxide/teloxide/issues/230)).
 - Mark all the API types as `#[non_exhaustive]`.
 - Replace all `mime_type: String` with `MimeWrapper`.

### Fixed
 - Now methods which can send file to Telegram returns `tokio::io::Result<T>`. Early its could panic ([issue 216](https://github.com/teloxide/teloxide/issues/216)).
 - If a bot wasn't triggered for several days, it stops responding ([issue 223](https://github.com/teloxide/teloxide/issues/223)).

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
