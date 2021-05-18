# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

 - `Storage::get_dialogue` to obtain a dialogue indexed by a chat ID.
 - `InMemStorageError` with a single variant `DialogueNotFound` to be returned from `InMemStorage::remove_dialogue`.
 - `RedisStorageError::DialogueNotFound` and `SqliteStorageError::DialogueNotFound` to be returned from `Storage::remove_dialogue`.
 - `Dispatcher::shutdown` function.
 - `Dispatcher::setup_ctrlc_handler` function ([issue 153](https://github.com/teloxide/teloxide/issues/153)).

### Changed

 - Do not return a dialogue from `Storage::{remove_dialogue, update_dialogue}`.
 - Return an error from `Storage::remove_dialogue` if a dialogue does not exist.
 - Require `D: Clone` in `dialogues_repl(_with_listener)` and `InMemStorage`.
 - Automatically delete a webhook if it was set up in `update_listeners::polling_default` (thereby making it `async`, [issue 319](https://github.com/teloxide/teloxide/issues/319)).
 - `polling` and `polling_default` now require `R: 'static`
 - Refactor `UpdateListener` trait:
   - Add a `stop` function that allows stopping the listener ([issue 166](https://github.com/teloxide/teloxide/issues/166)).
   - Remove blanked implementation.
   - Remove `Stream` from super traits.
   - Add `AsUpdateStream` to super traits.
     - Add an `AsUpdateStream` trait that allows turning implementors into streams of updates (GAT workaround).

### Fixed

 - Remove the `reqwest` dependency. It's not needed after the [teloxide-core] integration.
 - A storage persistency bug ([issue 304](https://github.com/teloxide/teloxide/issues/304)).
 - Log errors from `Storage::{remove_dialogue, update_dialogue}` in `DialogueDispatcher` ([issue 302](https://github.com/teloxide/teloxide/issues/302)).
 - Mark all the functions of `Storage` as `#[must_use]`.

## [0.4.0] - 2021-03-22

### Added
 - Integrate [teloxide-core].
 - Allow arbitrary error types to be returned from (sub)transitions ([issue 242](https://github.com/teloxide/teloxide/issues/242)).
 - The `respond` function, a shortcut for `ResponseResult::Ok(())`.
 - The `sqlite-storage` feature -- enables SQLite support.
 - `Dispatcher::{my_chat_members_handler, chat_members_handler}`

[teloxide-core]: https://github.com/teloxide/teloxide-core

### Deprecated

 - `UpdateWithCx::answer_str`

### Fixed

 - Hide `SubtransitionOutputType` from the docs.

### Changed
 - Export `teloxide_macros::teloxide` in `prelude`.
 - `dispatching::dialogue::serializer::{JSON -> Json, CBOR -> Cbor}`
 - Allow `bot_name` be `N`, where `N: Into<String> + ...` in `commands_repl` & `commands_repl_with_listener`.
 - 'Edit methods' (namely `edit_message_live_location`, `stop_message_live_location`, `edit_message_text`, 
   `edit_message_caption`, `edit_message_media` and `edit_message_reply_markup`) are split into common and inline 
   versions (e.g.: `edit_message_text` and `edit_inline_message_text`). Instead of `ChatOrInlineMessage` common versions
   accept `chat_id: impl Into<ChatId>` and `message_id: i32` whereas inline versions accept 
   `inline_message_id: impl Into<String>`. Also note that return type of inline versions is `True` ([issue 253], [pr 257])
 - `ChatOrInlineMessage` is renamed to `TargetMessage`, it's `::Chat`  variant is renamed to `::Common`, 
   `#[non_exhaustive]` annotation is removed from the enum, type of `TargetMessage::Inline::inline_message_id` changed 
   `i32` => `String`. `TargetMessage` now implements `From<String>`, `get_game_high_scores` and `set_game_score` use 
   `Into<TargetMessage>` to accept `String`s. ([issue 253], [pr 257])
 - Remove `ResponseResult` from `prelude`.

[issue 253]: https://github.com/teloxide/teloxide/issues/253
[pr 257]: https://github.com/teloxide/teloxide/pull/257

## [0.3.4] - 2020-01-13

### Fixed

 - Failing compilation with `serde::export` ([issue 328](https://github.com/teloxide/teloxide/issues/328)).

## [0.3.3] - 2020-10-30

### Fixed
 - The `dice` field from `MessageDice` is public now ([issue 306](https://github.com/teloxide/teloxide/issues/306))

## [0.3.2] - 2020-10-23

### Added
 - `LoginUrl::new` ([issue 298](https://github.com/teloxide/teloxide/issues/298))

## [0.3.1] - 2020-08-25

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
