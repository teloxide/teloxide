# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## unreleased

### Added

- Support for TBA 9.2 ([#1403](https://github.com/teloxide/teloxide/pull/1403))
  - Add `filter_suggested_post_approved`, `filter_suggested_post_approval_failed`, `filter_suggested_post_declined`, `filter_suggested_post_paid`, `filter_suggested_post_refunded` filters to the `MessageFilterExt` trait

### Fixed

- make sure `postgres-storage-rustls` feature actually enables rustls-based postgres storage ([#1400](https://github.com/teloxide/teloxide/pull/1400))

### Changed

- Some dependencies was bumped: `derive_more` to `2.0.1`, `deadpool-redis` to `0.22.0` ([#1408](https://github.com/teloxide/teloxide/pull/1408))

## 0.17.0 - 2025-07-11

### Added

- Support for TBA 8.1 ([#1377](https://github.com/teloxide/teloxide/pull/1377))

- Support for TBA 8.3 ([#1383](https://github.com/teloxide/teloxide/pull/1383))

- Support for TBA 9.0 ([#1385](pr1385) + [#1387](pr1387))
  - Add `filter_paid_message_price_changed`, `filter_gift_info` and `filter_unique_gift_info` filters

- Support for TBA 9.1 ([#1388](https://github.com/teloxide/teloxide/pull/1388))
  - Add `filter_checklist`, `checklist_tasks_done`, `checklist_tasks_added`, and `direct_message_price_changed` filters to the `MessageFilterExt` trait

### Changed

- Support for TBA 8.2 ([#1381](https://github.com/teloxide/teloxide/pull/1381))
  - Removed `hide_url` field from `InlineQueryResultArticle` struct [**BC**]

- Support for TBA 8.3 ([#1383](https://github.com/teloxide/teloxide/pull/1383))
  - `PaidMedia::Video` is now wrapped in a `Box` [**BC**]
  - `InputPaidMedia::Video` is now wrapped in a `Box` [**BC**]

- Support for TBA 9.0 ([#1385](pr1385) + [#1387](pr1387))
  - `TransactionPartnerUser` was reworked to have a `kind` field with `gift_purchase`, `invoice_payment`, `paid_media_payment`, `premium_purchase` getters [**BC**]
  - `can_send_gift` field in `ChatFullInfo` struct was replaced by `accepted_gift_types` [**BC**]
  - `can_reply` field in `BusinessConnection` struct was replaced by `rights` [**BC**]

### Fixed

- Fixed `create_forum_topic` to not require `icon_color` and `icon_custom_emoji_id` ([#1382](https://github.com/teloxide/teloxide/pull/1382)) [**BC**]
- Fixed `send_gift` and `send_gift_chat` with `ParseMode` adaptor ([#1385](pr1385))

[pr1385]: https://github.com/teloxide/teloxide/pull/1385
[pr1387]: https://github.com/teloxide/teloxide/pull/1387

## 0.16.0 - 2025-06-19

### Added

- `dptree` type checking and dead code checking [**BC**].

- New id types ([#1153](https://github.com/teloxide/teloxide/pull/1153)) [**BC**]
  - Add `PollId` struct
  - Add `CallbackQueryId` struct
  - Add `FileId` and `FileUniqueId` structs
  - Add `PreCheckoutQueryId` struct
  - Add `ShippingQueryId` struct
  - Add `InlineQueryId` struct
  - Add `BoostId` struct
  - Add `CustomEmojiId` struct
  - Add `MediaGroupId` struct
  - Add `EffectId` struct

- Support for TBA 7.6 ([#1356](https://github.com/teloxide/teloxide/pull/1356))
  - Add `filter_paid_media` to `MessageFilterExt` trait

- Support for TBA 7.7 ([#1357](https://github.com/teloxide/teloxide/pull/1357)) [**BC**]

- Support for TBA 7.10 ([#1366](https://github.com/teloxide/teloxide/pull/1366))
  - Add `filter_purchased_paid_media` filter

- Support for TBA 7.11 ([#1367](https://github.com/teloxide/teloxide/pull/1367))

- Support for TBA 8.0 ([#1369](pr1369))

[pr1369]: https://github.com/teloxide/teloxide/pull/1369

### Changed

- New id types ([#1153](https://github.com/teloxide/teloxide/pull/1153)) [**BC**]
  - Changed `id` field to `PollId` in `PollAnswer` and in `Poll`
  - Changed `id` field to `CallbackQueryId` in `CallbackQuery` and in `answer_callback_query` method
  - Changed `id` field to `FileId` in `File`, `InputFile::file_id` method and in `get_file` method
  - Changed `unique_id` field to `FileUniqueId` in `File`
  - Changed `small_file_id` and `big_file_id` to `FileId` in `ChatPhoto`
  - Changed `small_file_unique_id` and `big_file_unique_id` to `FileUniqueId` in `ChatPhoto`
  - Changed `inline_query_result_cached_...` structs to use `FileId`
  - Changed `id` field to `PreCheckoutQueryId` in `PreCheckoutQuery` and in `answer_pre_checkout_query` method
  - Changed `id` field to `ShippingQueryId` in `ShippingQuery` and in `answer_shipping_query` method
  - Changed `id` field to `InlineQueryId` in `InlineQuery` and in `answer_inline_query` method
  - Changed `boost_id` field to `BoostId` in `ChatBoost` and in `ChatBoostRemoved`
  - Changed `background_custom_emoji_id`, `profile_background_custom_emoji_id`, `emoji_status_custom_emoji_id` fields to `Option<CustomEmojiId>` in `ChatFullInfo`
  - Changed `icon_custom_emoji_id` field to `Option<CustomEmojiId>` in `ForumTopic`, `ForumTopicEdited` and in `ForumTopicCreated`
  - Changed `icon_custom_emoji_id` field to `CustomEmojiId` in `create_forum_topic` and `edit_forum_topic` methods
  - Changed `custom_emoji_ids` field to `Vec<CustomEmojiId>` in `get_custom_emoji_stickers` method
  - Changed `custom_emoji_id` field to `CustomEmojiId` in `set_custom_emoji_sticker_set_thumbnail` method
  - Changed `custom_emoji_id` field and/or return type to `CustomEmojiId` in `MessageEntityKind::CustomEmoji` and `MessageEntity::custom_emoji` method, `ReactionType::CustomEmoji` and `ReactionType::custom_emoji_id` method, `StickerKind::CustomEmojiId` and `StickerKind::custom_emoji_id` method
  - Changed `media_group_id` field to `MediaGroupId` in `MediaAudio`, `MediaVideo`, `MediaPhoto` and `MediaDocument`
  - Changed return type of `Message::media_group_id` to `Option<&MediaGroupId>`
  - Changed `message_effect_id` field to `EffectId` in `send_animation`, `send_audio`, `send_contact`, `send_dice`, `send_document`, `send_game`, `send_invoice`, `send_location`, `send_media_group`, `send_message`, `send_photo`, `send_sticker`, `send_venue`, `send_video`, `send_video_note` and `send_voice` methods
  - Changed `message_effect_id` field to `EffectId` in `MessageCommon`
  - Changed return type of `MessageCommon::effect_id` method to `Option<&EffectId>`

- MSRV (Minimal Supported Rust Version) was bumped from `1.80` to `1.82` ([#1358](https://github.com/teloxide/teloxide/pull/1358))

- Support for TBA 7.9 ([#1361](https://github.com/teloxide/teloxide/pull/1361))
    - Fixed return type of `revoke_chat_invite_link` and `editChatInviteLink` from `String` to `ChatInviteLink` [**BC**]

## 0.15.0 - 2025-04-04

### Added

- Add two examples to demonstrate how to execute functions _before_ and _after_ some endpoint:
  - [`examples/middlewares.rs`]
  - [`examples/middlewares_fallible.rs`]

[`examples/middlewares.rs`]: crates/teloxide/examples/middlewares.rs
[`examples/middlewares_fallible.rs`]: crates/teloxide/examples/middlewares_fallible.rs

### Changed

- Implement `Clone` for `teloxide::RequestError`
  - `RequestError::Network` now accepts `Arc<reqwest::Error>` [**BC**]
  - `RequestError::InvalidJson` now accepts `source: Arc<serde_json::Error>` [**BC**]
  - `RequestError::Io` now accepts `Arc<io::Error>` [**BC**]
- Implement `Clone` for `teloxide::DownloadError`
  - `DownloadError::Network` now accepts `Arc<reqwest::Error>` [**BC**]
  - `DownloadError::Io` now accepts `Arc<std::io::Error>` [**BC**]
- If `TELOXIDE_DIALOGUE_BEHAVIOUR` environment variable is set to `default`, `.enter_dialogue` will never error, instead setting the dialogue to default ([PR 1187](https://github.com/teloxide/teloxide/pull/1187))
- `D` (state) now has to implement `Clone` for `HandlerExt::enter_dialogue` and `dialogue::enter` [**BC**]

### Deprecated

- `DispatcherBuilder::stack_size` because it is now a no-op (see [PR 1320](https://github.com/teloxide/teloxide/pull/1320) for more details).

### Fixed

- Thread being locked because of thread spawining in dispatcher ([issue 1236](https://github.com/teloxide/teloxide/issues/1236))

## 0.14.1 - 2025-03-30

### Fixed

- Fixed docs build ([PR 1311](https://github.com/teloxide/teloxide/pull/1311))

## 0.14.0 - 2025-03-29

### Added

- `filter_boost_added` and `filter_reply_to_story` filters to the `MessageFilterExt` trait ([PR 1131](https://github.com/teloxide/teloxide/pull/1131))
- `filter_mention_command` filter to the `HandlerExt` trait ([issue 494](https://github.com/teloxide/teloxide/issues/494))
- `filter_business_connection`, `filter_business_message`, `filter_edited_business_message`, and `filter_deleted_business_messages` filters to the `UpdateFilterExt` trait ([PR 1146](https://github.com/teloxide/teloxide/pull/1146))
- Syntax sugar for making requests ([issue 1143](https://github.com/teloxide/teloxide/issues/1143)):
  - `bot.forward`, `bot.edit_live_location`, `bot.stop_live_location`, `bot.set_reaction`, `bot.pin`, `bot.unpin`, `bot.edit_text`, `bot.edit_caption`, `bot.edit_media`, `bot.edit_reply_markup`, `bot.stop_poll_message`, `bot.delete` and `bot.copy` methods to the new `crate::sugar::bot::BotMessagesExt` trait
  - `req.reply_to` method to the new `crate::sugar::request::RequestReplyExt` trait
  - `req.disable_link_preview` method to the new `crate::sugar::request::RequestLinkPreviewExt` trait
- `stack_size` setter to `DispatcherBuilder` ([PR 1185](https://github.com/teloxide/teloxide/pull/1185))
- `utils::render` module to render HTML/Markdown-formatted output ([PR 1152](https://github.com/teloxide/teloxide/pull/1152))
- `Bot::from_env` now can read and use `TELOXIDE_API_URL` environmental variable ([PR 1197](https://github.com/teloxide/teloxide/pull/1197))
- Improved developer experience: ([PR 1255](https://github.com/teloxide/teloxide/pull/1255))
  - Added devcontainer support. It was tested with VS Codium on Fedora/Podman and Ubuntu/Docker, but should work for any platform that supports devcontainers
  - Added Justfile for common tasks. E.g. run `just ci` for a full check, similar to what we do in CI (do it before sending PR!)
- `tracing` feature, that enables trait `UpdateHandlerExt` that instruments `UpdateHandler` with a custom `tracing::Span` ([PR 877](https://github.com/teloxide/teloxide/pull/877))
- Support for TBA 7.3 ([PR 1159](https://github.com/teloxide/teloxide/pull/1159))
  - Add `filter_chat_background_set` to `MessageFilterExt` trait
- Support for TBA 7.4 ([PR 1280](https://github.com/teloxide/teloxide/pull/1280))
- Support for TBA 7.5 ([PR 1281](https://github.com/teloxide/teloxide/pull/1281))

### Changed

- Environment bumps: ([PR 1147](https://github.com/teloxide/teloxide/pull/1147), [PR 1225](https://github.com/teloxide/teloxide/pull/1225))
  - MSRV (Minimal Supported Rust Version) was bumped from `1.70.0` to `1.80.0`
  - Some dependencies was bumped: `axum` to `0.8.0`, `sqlx` to `0.8.1`, `tower` to `0.5.0`, `reqwest` to `0.12.7`, `tower-http` to `0.6.2`, `derive_more` to `1.0.0`, `serde_with` to `3.12.0`, `aquamarine` to `0.6.0`, `deadpool-redis` to `0.20.0`, `thiserror` to `2.0.11`, `syn` to `2.0.96`, `bitflags` to `2`
  - `tokio` version was explicitly specified as `1.39`
- Added new `Send` and `Sync` trait bounds to the `UListener` and `Eh` generic parameters of `try_dispatch_with_listener` and `dispatch_with_listener` ([PR 1185](https://github.com/teloxide/teloxide/pull/1185)) [**BC**]
- Renamed `Limits::messages_per_min_channel` to `messages_per_min_channel_or_supergroup` to reflect its actual behavior ([PR 1214](https://github.com/teloxide/teloxide/pull/1214))
- Added derive `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash` to `ChatPermissions` ([PR 1242](https://github.com/teloxide/teloxide/pull/1242))
- Added derive `Clone`, `Debug` to `Settings` ([PR 1242](https://github.com/teloxide/teloxide/pull/1242))
- The `Throttle` adaptor now also throttles `forward_messages` and `copy_messages` like their non-batch counterparts, as well as `send_game` ([PR 1229](https://github.com/teloxide/teloxide/pull/1229))
- Also parse commands from text captions ([PR 1285](https://github.com/teloxide/teloxide/pull/1285))

### Fixed

- Now Vec<MessageId> in requests serializes into [number] instead of [ {message_id: number} ], `forward_messages`, `copy_messages` and `delete_messages` now work properly
- Now `InlineQueryResultsButton` serializes properly ([issue 1181](https://github.com/teloxide/teloxide/issues/1181))
- Now `ThreadId` is able to serialize in multipart requests ([PR 1179](https://github.com/teloxide/teloxide/pull/1179))
- Now stack does not overflow on dispatch ([issue 1154](https://github.com/teloxide/teloxide/issues/1154))
- Implement `RequestReplyExt` and `RequestLinkPreviewExt` on setters from `teloxide_core::payloads` so syntax sugar can work on bot adaptors too ([PR 1270](https://github.com/teloxide/teloxide/pull/1270))
- Now blockquote handling in the render module works correctly ([PR 1267](https://github.com/teloxide/teloxide/pull/1267))
- Now blockquote generation in the `utils::markdown` module works correctly ([PR 1273](https://github.com/teloxide/teloxide/pull/1273))
- Fixed calculation of per-second limits in the `Throttle` adaptor ([PR 1212](https://github.com/teloxide/teloxide/pull/1212))
- Parsing of `get_chat` responses for channels with paid reactions ([PR 1284](https://github.com/teloxide/teloxide/pull/1284))
- Fix the issue where requests will not consider API URL subpath ([PR 1295](https://github.com/teloxide/teloxide/pull/1295))

## 0.13.0 - 2024-08-16

### Added

- Documentation regarding the way captions work for the official clients on `SendMediaGroup` ([PR 992](https://github.com/teloxide/teloxide/pull/992))
- Add `MessageToCopyNotFound` error to `teloxide::errors::ApiError` ([PR 917](https://github.com/teloxide/teloxide/pull/917))
- `Dispatcher::try_dispatch_with_listener` ([PR 913](https://github.com/teloxide/teloxide/pull/913))
- Missing Message::filter_* functions ([PR 982](https://github.com/teloxide/teloxide/pull/982)):
  - `filter_game`
  - `filter_venue`
  - `filter_video`
  - `filter_video_note`
  - `filter_voice`
  - `filter_migration`
  - `filter_migration_from`
  - `filter_migration_to`
  - `filter_new_chat_title`
  - `filter_new_chat_photo`
  - `filter_delete_chat_photo`
  - `filter_group_chat_created`
  - `filter_supergroup_chat_created`
  - `filter_channel_chat_created`
  - `filter_message_auto_delete_timer_changed`
  - `filter_invoice`
  - `filter_successful_payment`
  - `filter_connected_website`
  - `filter_write_access_allowed`
  - `filter_passport_data`
  - `filter_proximity_alert_triggered`
  - `filter_forum_topic_created`
  - `filter_forum_topic_edited`
  - `filter_forum_topic_closed`
  - `filter_forum_topic_reopened`
  - `filter_general_forum_topic_hidden`
  - `filter_general_forum_topic_unhidden`
  - `filter_video_chat_scheduled`
  - `filter_video_chat_started`
  - `filter_video_chat_ended`
  - `filter_video_chat_participants_invited`
  - `filter_web_app_data`
- Implement `PostgresStorage`, a persistent dialogue storage based on [PostgreSQL](https://www.postgresql.org/)([PR 996](https://github.com/teloxide/teloxide/pull/996))
- Implement `GetChatId` for `teloxide_core::types::{Chat, ChatJoinRequest, ChatMemberUpdated}`
- Use [deadpool-redis](https://crates.io/crates/deadpool-redis) for Redis connection pooling ([PR 1081](https://github.com/teloxide/teloxide/pull/1081))
- Add `MessageExt::filter_story` method for the corresponding `MediaKind::Story` variant ([PR 1087](https://github.com/teloxide/teloxide/pull/1087))
- Add `update_listeners::webhooks::Options::path`, an option to make the webhook server listen on a different path, which can be useful behind a reverse proxy
- Add `filter_giveaway`, `filter_giveaway_completed`, `filter_giveaway_created` and `filter_giveaway_winners` filters to `MessageFilterExt` trait ([PR 1101](https://github.com/teloxide/teloxide/pull/1101))

### Fixed

- Use `UserId` instead of `i64` for `user_id` in `html::user_mention` and `markdown::user_mention` ([PR 896](https://github.com/teloxide/teloxide/pull/896))
- Greatly improved the speed of graceful shutdown (`^C`) ([PR 938](https://github.com/teloxide/teloxide/pull/938))
- Fix typos in documentation ([PR 953](https://github.com/teloxide/teloxide/pull/953))
- Use `Seconds` instead of `String` in `InlineQueryResultAudio` for `audio_duration` ([PR 994](https://github.com/teloxide/teloxide/pull/994))
- High CPU usage on network errors ([PR 1002](https://github.com/teloxide/teloxide/pull/1002), [Issue 780](https://github.com/teloxide/teloxide/issues/780))
- Fix app build errors when using items gated behind sqlite-storage with the feature sqlite-storage-rustls ([PR 1018](https://github.com/teloxide/teloxide/pull/1018))
- Fix typo in `ApiError::ToMuchMessages` variant (rename it to `TooMuchMessages`) ([PR 1046](https://github.com/teloxide/teloxide/pull/1046))
- Fix `ChatPermission` behavior to accurately reflect Telegram's functionality ([PR 1068](https://github.com/teloxide/teloxide/pull/1068))

### Changed

- MSRV (Minimal Supported Rust Version) was bumped from `1.64.0` to `1.68.0` ([PR 950][https://github.com/teloxide/teloxide/pull/950])
- Sqlx version was bumped from `0.6` to `0.7.3`([PR 995](https://github.com/teloxide/teloxide/pull/995))
- Feature `sqlite-storage` was renamed to `sqlite-storage-nativetls`([PR 995](https://github.com/teloxide/teloxide/pull/995))
- MSRV (Minimal Supported Rust Version) was bumped from `1.68.0` to `1.70.0` ([PR 996][https://github.com/teloxide/teloxide/pull/996])
- `axum` was bumped to `0.7`, along with related libraries used for webhooks ([PR 1093][https://github.com/teloxide/teloxide/pull/1093])
- `Polling`'s exponential backoff now results in 64 seconds maximum delay instead of 17 minutes ([PR 1113][https://github.com/teloxide/teloxide/pull/1113])
- `filter_forward_from` was renamed to `filter_forward_origin` and now returns `MessageOrigin` instead of `ForwardFrom` ([PR 1101](https://github.com/teloxide/teloxide/pull/1101))

### Removed

- `UpdateListener::timeout_hint` and related APIs ([PR 938](https://github.com/teloxide/teloxide/pull/938))

## 0.12.2 - 2023-02-15

### Fixed

- `docs.rs` documentation build

## 0.12.1 - 2023-02-15

### Fixed

- Allow `ChatJoinRequest` updates
- Some example links in documentation

### Added

- `Update::filter_chat_join_request`
- `sqlite-storage-rustls` feature, that allows using sqlite storage without `native-tls`

### Changed

- Updated `teloxide-core` to v0.9.1; see its [changelog](https://github.com/teloxide/teloxide/blob/master/crates/teloxide-core/CHANGELOG.md#091---2023-02-15) for more

## 0.12.0 - 2023-01-17

### Changed

- Updated `teloxide-macros` to v0.7.1; see its [changelog](crates/teloxide-macros/CHANGELOG.md#071---2023-01-17) for more
- Updated `teloxide-core` to v0.9.0; see its [changelog](crates/teloxide-core/CHANGELOG.md#090---2023-01-17) for more
- Updated `axum` to v0.6.0
- The module structure
  - `teloxide::dispatching::update_listeners` => `teloxide::update_listeners`
  - `teloxide::dispatching::repls` => `teloxide::repls`
- `CommandDescriptions::new` was made `const`
- The following functions were made `#[must_use]`:
  - `DispatcherBuilder::{enable_ctrlc_handler, distribution_function}`

### Removed

- `rocksdb-storage` feature and associated items (See [PR #761](https://github.com/teloxide/teloxide/pull/761) for reasoning) [**BC**]

### Deprecated

- `teloxide::dispatching::{update_listeners, repls}` (see in the "Changed" section)

## 0.11.3 - 2022-11-28

### Fixed

- Add another missing feature gate for `dispatching::repls` import ([issue #770](https://github.com/teloxide/teloxide/issues/770))

## 0.11.2 - 2022-11-18

### Fixed

- Add missing feature gate for `dispatching::repls` import ([issue #770](https://github.com/teloxide/teloxide/issues/770))

## 0.11.1 - 2022-10-31 [yanked]

This release was yanked because it accidentally [breaks backwards compatibility](https://github.com/teloxide/teloxide/issues/770)

### Added

- The `rocksdb-storage` feature -- enables the RocksDB support ([PR #753](https://github.com/teloxide/teloxide/pull/753))
- `teloxide::dispatching::repls::CommandReplExt`, `teloxide::prelude::CommandReplExt` ([issue #740](https://github.com/teloxide/teloxide/issues/740))

### Deprecated

- `teloxide::dispatching::repls::{commands_repl, commands_repl_with_listener}`, `teloxide::utils::command::BotCommands::ty` (use `CommandReplExt` instead)

## 0.11.0 - 2022-10-07

### Changed

- Updated `teloxide-macros` to v0.7.0; see its [changelog](https://github.com/teloxide/teloxide-macros/blob/master/CHANGELOG.md#070---2022-10-06) for more
- Updated `teloxide-core` to v0.8.0; see its [changelog](https://github.com/teloxide/teloxide-core/blob/master/CHANGELOG.md#080---2022-10-03) for more
- `UpdateListener` now has an associated type `Err` instead of a generic
- `AsUpdateStream` now has an associated type `StreamErr` instead of a generic
- Rename `dispatching::stop_token::{AsyncStopToken, AsyncStopFlag}` => `stop::{StopToken, StopFlag}`
- Replace the generic error type `E` with `RequestError` for REPLs (`repl(_with_listener)`, `commands_repl(_with_listener)`)
- The following functions are now `#[must_use]`:
  - `BotCommands::ty`
  - `CommandDescriptions::{new, global_description, username, username_from_me}`
  - `teloxide::filter_command`
  - `teloxide::dispatching::dialogue::enter`
- `BotCommands::parse` now accept `bot_username` as `&str`

### Added

- `requests::ResponseResult` to `prelude`

### Removed

- `dispatching::stop_token::StopToken` trait (all uses are replaced with `stop::StopToken` structure)
- Some previously deprecated items
  - `enable_logging!`, `enable_logging_with_filter!`
  - `HandlerFactory`, `HandlerExt::dispatch_by`

## 0.10.1 - 2022-07-22

### Fixed

- Mark the following functions with `#[must_use]` ([PR 457](https://github.com/teloxide/teloxide/pull/457)):
  - `TraceStorage::into_inner`
  - `AsyncStopToken::new_pair`
  - `AsyncStopFlag::is_stopped`
  - All from `crate::utils::{html, markdown}`
- Rendering of GIFs in lib.rs and crates.io ([PR 681](https://github.com/teloxide/teloxide/pull/681))

## 0.10.0 - 2022-07-21

### Added

- Security checks based on `secret_token` param of `set_webhook` to built-in webhooks
- `dispatching::update_listeners::{PollingBuilder, Polling, PollingStream}`
- `DispatcherBuilder::enable_ctrlc_handler` method

### Fixed

- `Dispatcher` no longer "leaks" memory for every inactive user ([PR 657](https://github.com/teloxide/teloxide/pull/657))
- Allow specifying a path to a custom command parser in `parse_with` ([issue 668](https://github.com/teloxide/teloxide/issues/668))

### Changed

- Add the `Key: Clone` requirement for `impl Dispatcher` [**BC**]
- `dispatching::update_listeners::{polling_default, polling}` now return a named, `Polling<_>` type
- Update `teloxide-core` to v0.7.0 with Bot API 6.1 support, see [its changelog][core07c] for more information [**BC**]

[core07c]: https://github.com/teloxide/teloxide-core/blob/master/CHANGELOG.md#070---2022-07-19

### Deprecated

- The `dispatching::update_listeners::polling` function
- `Dispatcher::setup_ctrlc_handler` method

## 0.9.2 - 2022-06-07

### Fixed

- Fix Api Unknown error (Can't parse entities) on message created with `utils::markdown::user_mention_or_link` if user full name contains some escapable symbols eg '.'

## 0.9.1 - 2022-05-27

### Fixed

- Fix `#[command(rename = "...")]` for custom command names ([issue 633](https://github.com/teloxide/teloxide/issues/633))

## 0.9.0 - 2022-04-27

### Added

- The `dispatching::filter_command` function (also accessible as `teloxide::filter_command`) as a shortcut for `dptree::entry().filter_command()`
- Re-export `dptree::case!` as `teloxide::handler!` (the former is preferred for new code)

### Changed

- Update `teloxide-core` to v0.6.0 with [Bot API 6.0] support [**BC**]

[Bot API 6.0]: https://core.telegram.org/bots/api#april-16-2022

## 0.8.2 - 2022-04-26

### Fixed

- Fix the broken `#[derive(DialogueState)]` (function return type `dptree::Handler`)

## 0.8.1 - 2022-04-24

### Added

- Implement `GetChatId` for `Update`
- The `dialogue::enter()` function as a shortcut for `dptree::entry().enter_dialogue()`

## 0.8.0 - 2022-04-18

### Removed

- The old dispatching system and related stuff: `dispatching`, `utils::UpState`, `prelude`, `repls2`, `crate::{dialogues_repl, dialogues_repl_with_listener}`, and `#[teloxide(subtransition)]` [**BC**]

### Added

- The new API for dialogue handlers: `teloxide::handler!` ([issue 567](https://github.com/teloxide/teloxide/issues/567))
- Built-in webhooks support via `teloxide::dispatching::update_listeners::webhooks` module
- `Dialogue::chat_id` for retrieving a chat ID from a dialogue

### Changed

- Updated `teloxide-core` from version `0.4.5` to version [`0.5.0`](https://github.com/teloxide/teloxide-core/releases/tag/v0.5.0) [**BC**]
- Rename `dispatching2` => `dispatching` [**BC**]
- Rename `prelude2` => `prelude` [**BC**]
- Move `update_listeners`, `stop_token`, `IdleShutdownError`, and `ShutdownToken` from the old `dispatching` to the new `dispatching` (previously `dispatching2`)
- Replace `crate::{commands_repl, commands_repl_with_listener, repl, repl_with_listener}` with those of the new `dispatching` [**BC**]
- `UpdateListener::StopToken` is now always `Send` [**BC**]
- Rename `BotCommand` trait to `BotCommands` [**BC**]
- `BotCommands::descriptions` now returns `CommandDescriptions` instead of `String` [**BC**]
- Mark `Dialogue::new` as `#[must_use]`

### Fixed

- Concurrent update handling in the new dispatcher ([issue 536](https://github.com/teloxide/teloxide/issues/536))

### Deprecated

- `HandlerFactory` and `HandlerExt::dispatch_by` in favour of `teloxide::handler!`

## 0.7.3 - 2022-04-03

### Fixed

- Update `teloxide-core` to version `0.4.5` to fix a security vulnerability. See more in `teloxide-core` [release notes](https://github.com/teloxide/teloxide-core/releases/tag/v0.4.5)

## 0.7.2 - 2022-03-23

### Added

- The `Storage::erase` default function that returns `Arc<ErasedStorage>`
- `ErasedStorage`, a storage with an erased error type
- Allow the storage generic `S` be `?Sized` in `Dialogue` and `HandlerExt::enter_dialogue`

### Deprecated

- `enable_logging!` and `enable_logging_with_filter!` macros

### Fixed

- Log `UpdateKind::Error` in `teloxide::dispatching2::Dispatcher`
- Don't warn about unhandled updates in `repls2` ([issue 557](https://github.com/teloxide/teloxide/issues/557))
- `parse_command` and `parse_command_with_prefix` now ignores case of the bot username

## 0.7.1 - 2022-03-09

### Fixed

- Compilation with non-default features

## 0.7.0 - 2022-02-09

### Fixed

- `Dispatcher` wasn't `Send`. Make `DispatcherBuilder::{default_handler, error_handler}` accept a handler that implements `Send + Sync` ([PR 517](https://github.com/teloxide/teloxide/pull/517))

## 0.6.1 - 2022-02-06

### Fixed

- docs.rs documentation build

## 0.6.0 - 2022-02-06

### Added

- `BotCommand::bot_commands` to obtain Telegram API commands ([issue 262](https://github.com/teloxide/teloxide/issues/262))
- The `dispatching2` and `prelude2` modules. They present a new dispatching model based on `dptree`

### Changed

- Require that `AsUpdateStream::Stream` is `Send`
- Restrict a user crate by `CARGO_CRATE_NAME` instead of `CARGO_PKG_NAME` in `enable_logging!` and `enable_logging_with_filter!`
- Updated `teloxide-core` to v0.4.0, see [its changelog](https://github.com/teloxide/teloxide-core/blob/master/CHANGELOG.md#040---2022-02-03)

### Deprecated

 - The `dispatching` and `prelude` modules

### Fixed

- Infinite retries while stopping polling listener ([issue 496](https://github.com/teloxide/teloxide/issues/496))
- `polling{,_default}` and it's `Stream` and `StopToken` not being `Send` (and by extension fix the same problem with `repl`s)

## 0.5.3 - 2021-10-25

### Fixed

- Compilation when the `ctrlc_handler` feature is disabled ([issue 462](https://github.com/teloxide/teloxide/issues/462))

## 0.5.2 - 2021-08-25

### Fixed

- Depend on a correct `futures` version (v0.3.15)

## 0.5.1 - 2021-08-05

### Changed

- Improved log messages when `^C` is received with `^C` handler set up

## 0.5.0 - 2021-07-08

### Added

- `Storage::get_dialogue` to obtain a dialogue indexed by a chat ID
- `InMemStorageError` with a single variant `DialogueNotFound` to be returned from `InMemStorage::remove_dialogue`
- `RedisStorageError::DialogueNotFound` and `SqliteStorageError::DialogueNotFound` to be returned from `Storage::remove_dialogue`
- A way to `shutdown` dispatcher
  - `Dispatcher::shutdown_token` function
  - `ShutdownToken` with a `shutdown` function
- `Dispatcher::setup_ctrlc_handler` function ([issue 153](https://github.com/teloxide/teloxide/issues/153))
- `IdleShutdownError`
- Automatic update filtering ([issue 389](https://github.com/teloxide/teloxide/issues/389))
- Added reply shortcut to every kind of messages ([PR 404](https://github.com/teloxide/teloxide/pull/404))

### Changed

- Do not return a dialogue from `Storage::{remove_dialogue, update_dialogue}`
- Return an error from `Storage::remove_dialogue` if a dialogue does not exist
- Require `D: Clone` in `dialogues_repl(_with_listener)` and `InMemStorage`
- Automatically delete a webhook if it was set up in `update_listeners::polling_default` (thereby making it `async`, [issue 319](https://github.com/teloxide/teloxide/issues/319))
- `polling` and `polling_default` now require `R: 'static`
- Refactor `UpdateListener` trait:
  - Add a `StopToken` associated type
    - It must implement a new `StopToken` trait which has the only function `fn stop(self);`
  - Add a `stop_token` function that returns `Self::StopToken` and allows stopping the listener later ([issue 166](https://github.com/teloxide/teloxide/issues/166))
  - Remove blanked implementation
  - Remove `Stream` from super traits
  - Add `AsUpdateStream` to super traits
    - Add an `AsUpdateStream` trait that allows turning implementors into streams of updates (GAT workaround)
  - Add a `timeout_hint` function (with a default implementation)
- `Dispatcher::dispatch` and `Dispatcher::dispatch_with_listener` now require mutable reference to self
- Repls can now be stopped by `^C` signal
- `Noop` and `AsyncStopToken`stop tokens
- `StatefulListener`
- Emit not only errors but also warnings and general information from teloxide, when set up by `enable_logging!`
- Use `i64` instead of `i32` for `user_id` in `html::user_mention` and `markdown::user_mention`
- Updated to `teloxide-core` `v0.3.0` (see it's [changelog](https://github.com/teloxide/teloxide-core/blob/master/CHANGELOG.md#030---2021-07-05) for more)

### Fixed

- Remove the `reqwest` dependency. It's not needed after the [teloxide-core] integration
- A storage persistence bug ([issue 304](https://github.com/teloxide/teloxide/issues/304))
- Log errors from `Storage::{remove_dialogue, update_dialogue}` in `DialogueDispatcher` ([issue 302](https://github.com/teloxide/teloxide/issues/302))
- Mark all the functions of `Storage` as `#[must_use]`

## 0.4.0 - 2021-03-22

### Added

- Integrate [teloxide-core]
- Allow arbitrary error types to be returned from (sub)transitions ([issue 242](https://github.com/teloxide/teloxide/issues/242))
- The `respond` function, a shortcut for `ResponseResult::Ok(())`
- The `sqlite-storage` feature -- enables SQLite support
- `Dispatcher::{my_chat_members_handler, chat_members_handler}`

[teloxide-core]: https://github.com/teloxide/teloxide-core

### Deprecated

- `UpdateWithCx::answer_str`

### Fixed

- Hide `SubtransitionOutputType` from the docs

### Changed

- Export `teloxide_macros::teloxide` in `prelude`
- `dispatching::dialogue::serializer::{JSON -> Json, CBOR -> Cbor}`
- Allow `bot_name` be `N`, where `N: Into<String> + ...` in `commands_repl` & `commands_repl_with_listener`
- 'Edit methods' (namely `edit_message_live_location`, `stop_message_live_location`, `edit_message_text`,
   `edit_message_caption`, `edit_message_media` and `edit_message_reply_markup`) are split into common and inline
   versions (e.g.: `edit_message_text` and `edit_inline_message_text`). Instead of `ChatOrInlineMessage` common versions
   accept `chat_id: impl Into<ChatId>` and `message_id: i32` whereas inline versions accept
   `inline_message_id: impl Into<String>`. Also note that return type of inline versions is `True` ([issue 253], [pr 257])
- `ChatOrInlineMessage` is renamed to `TargetMessage`, it's `::Chat`  variant is renamed to `::Common`,
   `#[non_exhaustive]` annotation is removed from the enum, type of `TargetMessage::Inline::inline_message_id` changed
   `i32` => `String`. `TargetMessage` now implements `From<String>`, `get_game_high_scores` and `set_game_score` use
   `Into<TargetMessage>` to accept `String`s. ([issue 253], [pr 257])
- Remove `ResponseResult` from `prelude`

[issue 253]: https://github.com/teloxide/teloxide/issues/253
[pr 257]: https://github.com/teloxide/teloxide/pull/257

## 0.3.4 - 2020-01-13

### Fixed

- Failing compilation with `serde::export` ([issue 328](https://github.com/teloxide/teloxide/issues/328))

## 0.3.3 - 2020-10-30

### Fixed

- The `dice` field from `MessageDice` is public now ([issue 306](https://github.com/teloxide/teloxide/issues/306))

## 0.3.2 - 2020-10-23

### Added

- `LoginUrl::new` ([issue 298](https://github.com/teloxide/teloxide/issues/298))

## 0.3.1 - 2020-08-25

### Added

- `Bot::builder` method ([PR 269](https://github.com/teloxide/teloxide/pull/269))

## 0.3.0 - 2020-07-31

### Added

- Support for typed bot commands ([issue 152](https://github.com/teloxide/teloxide/issues/152))
- `BotBuilder`, which allows setting a default `ParseMode`
- The `Transition`, `Subtransition`, `SubtransitionOutputType` traits
- A nicer approach to manage dialogues via `#[derive(Transition)]` + `#[teloxide(subtransition)]` (see [`examples/dialogue_bot`](https://github.com/teloxide/teloxide/tree/af2aa218e7bfc442ab4475023a1c661834f576fc/examples/dialogue_bot))
- The `redis-storage` feature -- enables the Redis support
- The `cbor-serializer` feature -- enables the `CBOR` serializer for dialogues
- The `bincode-serializer` feature -- enables the `Bincode` serializer for dialogues
- The `frunk` feature -- enables `teloxide::utils::UpState`, which allows mapping from a structure of `field1, ..., fieldN` to a structure of `field1, ..., fieldN, fieldN+1`
- Upgrade to v4.9 Telegram bots API
- `teloxide::utils::client_from_env` -- constructs a client from the `TELOXIDE_TOKEN` environmental variable
- Import `Transition`, `TransitionIn`, `TransitionOut`, `UpState` to `teloxide::prelude`
- Import `repl`, `commands_repl` to `teloxide`
- Let users inspect an unknown API error using `ApiErrorKind::Unknown(String)`. All the known API errors are placed into `KnownApiErrorKind`
- Setters to all the API types
- `teloxide::dispatching::dialogue::serializer` -- various serializers for memory storages. The `Serializer` trait, `Bincode`, `CBOR`, `JSON`
- `teloxide::{repl, repl_with_listener, commands_repl, commands_repl_with_listener, dialogues_repl, dialogues_repl_with_listener}`
- `InputFile::Memory`
- Option to hide a command from description ([issue 217](https://github.com/teloxide/teloxide/issues/217))
- Respect the `TELOXIDE_PROXY` environment variable in `Bot::from_env`

### Deprecated

- `Bot::{from_env_with_client, new, with_client}`

### Changed

- `DialogueDispatcherHandlerCx` -> `DialogueWithCx`
- `DispatcherHandlerCx` -> `UpdateWithCx`
- Now provided description of unknown telegram error, by splitting ApiErrorKind at `ApiErrorKind` and `ApiErrorKindKnown` enums ([issue 199](https://github.com/teloxide/teloxide/issues/199))
- Extract `Bot` from `Arc` ([issue 216](https://github.com/teloxide/teloxide/issues/230))
- Mark all the API types as `#[non_exhaustive]`
- Replace all `mime_type: String` with `MimeWrapper`

### Fixed

- Now methods which can send file to Telegram return `tokio::io::Result<T>`. Before that it could panic ([issue 216](https://github.com/teloxide/teloxide/issues/216))
- If a bot wasn't triggered for several days, it stops responding ([issue 223](https://github.com/teloxide/teloxide/issues/223))

## 0.2.0 - 2020-02-25

### Added

- The functionality to parse commands only with a correct bot's name (breaks backwards compatibility) ([Issue 168](https://github.com/teloxide/teloxide/issues/168))
- This `CHANGELOG.md`

### Fixed

- Fix parsing a pinned message ([Issue 167](https://github.com/teloxide/teloxide/issues/167))
- Replace `LanguageCode` with `String`, because [the official Telegram documentation](https://core.telegram.org/bots/api#getchat) doesn't specify a concrete version of IETF language tag
- Problems with the `poll_type` field ([Issue 178](https://github.com/teloxide/teloxide/issues/178))
- Make `polling_default` actually a long polling update listener ([PR 182](https://github.com/teloxide/teloxide/pull/182))

### Removed

- [either](https://crates.io/crates/either) from the dependencies in `Cargo.toml`
- `teloxide-macros` migrated into [the separate repository](https://github.com/teloxide/teloxide-macros) to easier releases and testing

## 0.1.0 - 2020-02-19

### Added

- This project
