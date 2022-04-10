# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## unreleased

### Added

- `errors` module and `errors::AsResponseParameters` trait
- `EditedMessageIsTooLong` error [#109][pr109]
- `UntilDate` enum and use it for `{Restricted, Banned}::until_date` ([#116][pr116])
- `Limits::messages_per_min_channel` ([#121][pr121])

[pr109]: https://github.com/teloxide/teloxide-core/pull/109
[pr116]: https://github.com/teloxide/teloxide-core/pull/116
[pr121]: https://github.com/teloxide/teloxide-core/pull/121

### Changed

- Improve `Throttling` adoptor
  - Freeze when getting `RetryAfter(_)` error
  - Retry requests that previously returned `RetryAfter(_)` error
- `RequestError::RetryAfter` now has a `Duration` field instead of `i32`

### Added

- `UserId::{url, is_anonymous, is_channel, is_telegram}` convenience functions ([#197][pr197]) 
- `User::{tme_url, preferably_tme_url}` convenience functions ([#197][pr197])
- `Me::username` and `Deref<Target = User>` implementation for `Me` ([#197][pr197])
- `Me::{mention, tme_url}` ([#197][pr197])

[pr197]: https://github.com/teloxide/teloxide-core/pull/197

### Changed

- `user.id` now uses `UserId` type, `ChatId` now represents only _chat id_, not channel username, all `chat_id` function parameters now accept `Recipient` [**BC**]

## 0.4.5 - 2022-04-03

### Fixed

- Hide bot token in errors ([#200][200])

[200]: https://github.com/teloxide/teloxide-core/pull/200

## 0.4.4 - 2022-04-21

### Added

- `WrongFileIdOrUrl` and `FailedToGetUrlContent` errors ([#188][pr188])
- `NotFound` error ([#190][pr190])
- `HasPayload::with_payload_mut` function ([#189][pr189])

[pr188]: https://github.com/teloxide/teloxide-core/pull/188
[pr189]: https://github.com/teloxide/teloxide-core/pull/189
[pr190]: https://github.com/teloxide/teloxide-core/pull/190

## 0.4.3 - 2022-03-08

### Added

- `User::is_telegram` function ([#186][pr186])

[pr186]: https://github.com/teloxide/teloxide-core/pull/186

### Fixed

- `Update::chat()` now returns `Some(&Chat)` for `UpdateKind::ChatMember`, `UpdateKind::MyChatMember`,
  `UpdateKind::ChatJoinRequest` ([#184][pr184])
- `get_updates` timeouts (partially revert buggy [#180][pr180]) ([#185][pr185])

[pr184]: https://github.com/teloxide/teloxide-core/pull/184
[pr185]: https://github.com/teloxide/teloxide-core/pull/185

## 0.4.2 - 2022-02-17 [yanked]

### Deprecated

- `Message::chat_id` use `.chat.id` field instead ([#182][pr182])

[pr182]: https://github.com/teloxide/teloxide-core/pull/182

### Fixed

- Serialization of `SendPoll::type_` (it's now possible to send quiz polls) ([#181][pr181])

[pr181]: https://github.com/teloxide/teloxide-core/pull/181

### Added

- `Payload::timeout_hint` method to properly handle long running requests like `GetUpdates` ([#180][pr180])

[pr180]: https://github.com/teloxide/teloxide-core/pull/180

## 0.4.1 - 2022-02-13

### Fixed

- Deserialization of `UntilDate` ([#178][pr178])

[pr178]: https://github.com/teloxide/teloxide-core/pull/178

## 0.4.0 - 2022-02-03

### Added

- `ApiError::TooMuchInlineQueryResults` ([#135][pr135])
- `ApiError::NotEnoughRightsToChangeChatPermissions` ([#155][pr155])
- Support for 5.4 telegram bot API ([#133][pr133])
- Support for 5.5 telegram bot API ([#143][pr143], [#164][pr164])
- Support for 5.6 telegram bot API ([#162][pr162])
- Support for 5.7 telegram bot API ([#175][pr175])
- `EditedMessageIsTooLong` error ([#109][pr109])
- `UntilDate` enum and use it for `{Restricted, Banned}::until_date` ([#117][pr117])
- `Limits::messages_per_min_channel` ([#121][pr121])
- `media_group_id` field to `MediaDocument` and `MediaAudio` ([#139][pr139])
- `caption_entities` method to `InputMediaPhoto` ([#140][pr140])
- `User::is_anonymous` and `User::is_channel` functions ([#151][pr151])
- `UpdateKind::Error` ([#156][pr156])

[pr109]: https://github.com/teloxide/teloxide-core/pull/109
[pr117]: https://github.com/teloxide/teloxide-core/pull/117
[pr121]: https://github.com/teloxide/teloxide-core/pull/121
[pr135]: https://github.com/teloxide/teloxide-core/pull/135
[pr139]: https://github.com/teloxide/teloxide-core/pull/139
[pr140]: https://github.com/teloxide/teloxide-core/pull/140
[pr143]: https://github.com/teloxide/teloxide-core/pull/143
[pr151]: https://github.com/teloxide/teloxide-core/pull/151
[pr155]: https://github.com/teloxide/teloxide-core/pull/155
[pr156]: https://github.com/teloxide/teloxide-core/pull/156
[pr162]: https://github.com/teloxide/teloxide-core/pull/162
[pr164]: https://github.com/teloxide/teloxide-core/pull/164
[pr175]: https://github.com/teloxide/teloxide-core/pull/175

### Changed

- Refactor `InputFile` ([#167][pr167])
  - Make it an opaque structure, instead of enum
  - Add `read` constructor, that allows creating `InputFile` from `impl AsyncRead`
  - Internal changes
- Refactor errors ([#134][pr134])
  - Rename `DownloadError::NetworkError` to `Network`
  - Rename `RequestError::ApiError` to `Api`
  - Remove `RequestError::Api::status_code` and rename `RequestError::Api::kind` to `0` (struct to tuple struct)
  - Rename `RequestError::NetworkError` to `Network`
  - Implement `Error` for `ApiError`
- Use `url::Url` for urls, use `chrono::DateTime<Utc>` for dates in types ([#115][pr115])
- Mark `ApiError` as `non_exhaustive` ([#125][pr125])
- `InputFile` and related structures now do **not** implement `PartialEq`, `Eq` and `Hash` ([#133][pr133])
- How forwarded messages are represented ([#151][pr151])
- `RequestError::InvalidJson` now has a `raw` field with raw json for easier debugability ([#150][pr150])
- `ChatPermissions` is now bitflags ([#157][pr157])
- Type of `WebhookInfo::ip_address` from `Option<String>` to `Option<std::net::IpAddr>` ([#172][pr172])
- Type of `WebhookInfo::allowed_updates` from `Option<Vec<String>>` to `Option<Vec<AllowedUpdate>>` ([#174][pr174])

[pr115]: https://github.com/teloxide/teloxide-core/pull/115
[pr125]: https://github.com/teloxide/teloxide-core/pull/125
[pr134]: https://github.com/teloxide/teloxide-core/pull/134
[pr150]: https://github.com/teloxide/teloxide-core/pull/150
[pr157]: https://github.com/teloxide/teloxide-core/pull/157
[pr167]: https://github.com/teloxide/teloxide-core/pull/167
[pr172]: https://github.com/teloxide/teloxide-core/pull/172
[pr174]: https://github.com/teloxide/teloxide-core/pull/174

### Fixed

- Deserialization of chat migrations, see issue [#427][issue427] ([#143][pr143])
- Type of `BanChatMember::until_date`: `u64` -> `chrono::DateTime<Utc>` ([#117][pr117])
- Type of `Poll::correct_option_id`: `i32` -> `u8` ([#119][pr119])
- Type of `Poll::open_period`: `i32` -> `u16` ([#119][pr119])
- `Throttle` adaptor not honouring chat/min limits ([#121][pr121])
- Make `SendPoll::type_` optional ([#133][pr133])
- Bug with `caption_entities`, see issue [#473][issue473]
- Type of response for `CopyMessage` method ([#141](pr141), [#142](pr142))
- Bad request serialization when the `language` field of `MessageEntityKind::Pre` is `None` ([#145](pr145))
- Deserialization of `MediaKind::Venue` ([#147][pr147])
- Deserialization of `VoiceChat{Started,Ended}` messages ([#153][pr153])
- Serialization of `BotCommandScope::Chat{,Administrators}` ([#154][pr154])

[pr119]: https://github.com/teloxide/teloxide-core/pull/119
[pr133]: https://github.com/teloxide/teloxide-core/pull/133
[pr141]: https://github.com/teloxide/teloxide-core/pull/141
[pr142]: https://github.com/teloxide/teloxide-core/pull/142
[pr143]: https://github.com/teloxide/teloxide-core/pull/143
[pr145]: https://github.com/teloxide/teloxide-core/pull/145
[pr147]: https://github.com/teloxide/teloxide-core/pull/147
[pr153]: https://github.com/teloxide/teloxide-core/pull/153
[pr154]: https://github.com/teloxide/teloxide-core/pull/154
[issue473]: https://github.com/teloxide/teloxide/issues/473
[issue427]: https://github.com/teloxide/teloxide/issues/427

### Removed

- `get_updates_fault_tolerant` method and `SemiparsedVec` ([#156][pr156])

## 0.3.3 - 2021-08-03

### Fixed

- Compilation with `nightly` feature (use `type_alias_impl_trait` instead of `min_type_alias_impl_trait`) ([#108][pr108])

[pr108]: https://github.com/teloxide/teloxide-core/pull/108

## 0.3.2 - 2021-07-27

### Added

- `ErasedRequester` bot adaptor, `ErasedRequest` struct, `{Request, RequesterExt}::erase` functions ([#105][pr105])
- `Trace` bot adaptor ([#104][pr104])
- `HasPayload`, `Request` and `Requester` implementations for `either::Either` ([#103][pr103])

[pr103]: https://github.com/teloxide/teloxide-core/pull/103
[pr104]: https://github.com/teloxide/teloxide-core/pull/104
[pr105]: https://github.com/teloxide/teloxide-core/pull/105

## 0.3.1 - 2021-07-07

- Minor documentation tweaks ([#102][pr102])
- Remove `Self: 'static` bound on `RequesterExt::throttle` ([#102][pr102])

[pr102]: https://github.com/teloxide/teloxide-core/pull/102

## 0.3.0 - 2021-07-05

### Added

- `impl Clone` for {`CacheMe`, `DefaultParseMode`, `Throttle`} ([#76][pr76])
- `DefaultParseMode::parse_mode` which allows to get currently used default parse mode ([#77][pr77])
- `Thrrotle::{limits,set_limits}` functions ([#77][pr77])
- `Throttle::{with_settings,spawn_with_settings}` and `throttle::Settings` ([#96][pr96])
- Getters for fields nested in `Chat` ([#80][pr80]) 
- API errors: `ApiError::NotEnoughRightsToManagePins`, `ApiError::BotKickedFromSupergroup` ([#84][pr84])
- Telegram bot API 5.2 support ([#86][pr86])
- Telegram bot API 5.3 support ([#99][pr99])
- `net::default_reqwest_settings` function ([#90][pr90])

[pr75]: https://github.com/teloxide/teloxide-core/pull/75
[pr77]: https://github.com/teloxide/teloxide-core/pull/77
[pr76]: https://github.com/teloxide/teloxide-core/pull/76
[pr80]: https://github.com/teloxide/teloxide-core/pull/80
[pr84]: https://github.com/teloxide/teloxide-core/pull/84
[pr86]: https://github.com/teloxide/teloxide-core/pull/86
[pr90]: https://github.com/teloxide/teloxide-core/pull/90
[pr96]: https://github.com/teloxide/teloxide-core/pull/96
[pr99]: https://github.com/teloxide/teloxide-core/pull/99

### Changed

- `Message::url` now returns links to messages in private groups too ([#80][pr80]) 
- Refactor `ChatMember` methods ([#74][pr74])
  - impl `Deref<Target = ChatMemberKind>` to make `ChatMemberKind`'s methods callable directly on `ChatMember`
  - Add `ChatMemberKind::is_{creator,administrator,member,restricted,left,kicked}` which check `kind` along with `is_privileged` and `is_in_chat` which combine some of the above.
  - Refactor privilege getters
- Rename `ChatAction::{RecordAudio => RecordVoice, UploadAudio => UploadVoice}` ([#86][pr86])
- Use `url::Url` for urls, use `chrono::DateTime<Utc>` for dates ([#97][pr97])

[pr74]: https://github.com/teloxide/teloxide-core/pull/74
[pr97]: https://github.com/teloxide/teloxide-core/pull/97

### Fixed

- telegram_response: fix issue `retry_after` and `migrate_to_chat_id` handling ([#94][pr94])
- Type of `PublicChatSupergroup::slow_mode_delay` field: `Option<i32>`=> `Option<u32>` ([#80][pr80]) 
- Add missing `Chat::message_auto_delete_time` field ([#80][pr80]) 
- Output types of `LeaveChat` `PinChatMessage`, `SetChatDescription`, `SetChatPhoto` `SetChatTitle`, `UnpinAllChatMessages` and `UnpinChatMessage`: `String` => `True` ([#79][pr79])
- `SendChatAction` output type `Message` => `True` ([#75][pr75])
- `GetChatAdministrators` output type `ChatMember` => `Vec<ChatMember>` ([#73][pr73])
- `reqwest` dependency bringing `native-tls` in even when `rustls` was selected ([#71][pr71])
- Type of `{Restricted,Kicked}::until_date` fields: `i32` => `i64` ([#74][pr74])
- Type of `PhotoSize::{width,height}` fields: `i32` => `u32` ([#100][pr100])

[pr71]: https://github.com/teloxide/teloxide-core/pull/71
[pr73]: https://github.com/teloxide/teloxide-core/pull/73
[pr75]: https://github.com/teloxide/teloxide-core/pull/75
[pr79]: https://github.com/teloxide/teloxide-core/pull/79
[pr94]: https://github.com/teloxide/teloxide-core/pull/94
[pr100]: https://github.com/teloxide/teloxide-core/pull/100

## 0.2.2 - 2020-03-22

### Fixed

- Typo: `ReplyMarkup::{keyboad => keyboard}` ([#69][pr69])
  - Note: method with the old name was deprecated and hidden from docs

[pr69]: https://github.com/teloxide/teloxide-core/pull/69

## 0.2.1 - 2020-03-19

### Fixed 

- Types fields privacy (make fields of some types public) ([#68][pr68])
  - `Dice::{emoji, value}`
  - `MessageMessageAutoDeleteTimerChanged::message_auto_delete_timer_changed`
  - `PassportElementError::{message, kind}`
  - `StickerSet::thumb`

[pr68]: https://github.com/teloxide/teloxide-core/pull/68

## 0.2.0 - 2020-03-16

### Changed

- Refactor `ReplyMarkup` ([#pr65][pr65]) (**BC**)
  - Rename `ReplyMarkup::{InlineKeyboardMarkup => InlineKeyboard, ReplyKeyboardMarkup => Keyboard, ReplyKeyboardRemove => KeyboardRemove}`
  - Add `inline_kb`, `keyboad`, `kb_remove` and `force_reply` `ReplyMarkup` consructors
  - Rename `ReplyKeyboardMarkup` => `KeyboardMarkup`
  - Rename `ReplyKeyboardRemove` => `KeyboardRemove`
  - Remove useless generic param from `ReplyKeyboardMarkup::new` and `InlineKeyboardMarkup::new`
  - Change parameters order in `ReplyKeyboardMarkup::append_to_row` and `InlineKeyboardMarkup::append_to_row`
- Support telegram bot API version 5.1 (see it's [changelog](https://core.telegram.org/bots/api#march-9-2021)) ([#pr63][pr63]) (**BC**)
- Support telegram bot API version 5.0 (see it's [changelog](https://core.telegram.org/bots/api#november-4-2020)) ([#pr62][pr62]) (**BC**)

[pr62]: https://github.com/teloxide/teloxide-core/pull/62
[pr63]: https://github.com/teloxide/teloxide-core/pull/63
[pr65]: https://github.com/teloxide/teloxide-core/pull/65

### Added

- `GetUpdatesFaultTolerant` - fault toletant version of `GetUpdates` ([#58][pr58]) (**BC**)
- Derive `Clone` for `AutoSend`.

[pr58]: https://github.com/teloxide/teloxide-core/pull/58

### Fixed

- Make `MediaContact::contact` public ([#pr64][pr64])
- `set_webhook` signature (make `allowed_updates` optional) ([#59][pr59])
- Fix typos in payloads ([#57][pr57]):
  - `get_updates`: `offset` `i64` -> `i32`
  - `send_location`: make `live_period` optional
- `send_contact` signature (`phone_number` and `first_name` `f64` => `String`) ([#56][pr56])

[pr56]: https://github.com/teloxide/teloxide-core/pull/56
[pr57]: https://github.com/teloxide/teloxide-core/pull/57
[pr59]: https://github.com/teloxide/teloxide-core/pull/59
[pr64]: https://github.com/teloxide/teloxide-core/pull/64

### Removed

- `Message::text_owned` ([#pr62][pr62]) (**BC**)

### Changed

 - `NonStrictVec` -> `SemiparsedVec`.

## 0.1.1 - 2020-02-17

### Fixed

- Remove `dbg!` call from internals ([#53][pr53])

[pr53]: https://github.com/teloxide/teloxide-core/pull/53

## 0.1.0 - 2020-02-17

### Added

- `#[non_exhaustive]` on `InputFile` since we may want to add new ways to send files in the future ([#49][pr49])
- `MultipartPayload` for future proofing ([#49][pr49])
- Support for `rustls` ([#24][pr24])
- `#[must_use]` attr to payloads implemented by macro ([#22][pr22])
- forward-to-deref `Requester` impls ([#39][pr39])
- `Bot::{set_,}api_url` methods ([#26][pr26], [#35][pr35])
- `payloads` module
- `RequesterExt` trait which is implemented for all `Requester`s and allows easily wrapping them in adaptors
- `adaptors` module ([#14][pr14])
  - `throttle`, `cache_me`, `auto_send` and `full` crate features
  - Request throttling - opt-in feature represented by `Throttle` bot adapter which allows automatically checking telegram limits ([#10][pr10], [#46][pr46], [#50][pr50])
  - Request auto sending - ability to `.await` requests without need to call `.send()` (opt-in feature represented by `AutoSend` bot adapter, [#8][pr8])
  - `get_me` caching (opt-in feature represented by `CacheMe` bot adapter)
- `Requester` trait which represents bot-clients ([#7][pr7], [#12][pr12], [#27][pr27])
- `{Json,Multipart}Request` the `Bot` requests types ([#6][pr6])
- `Output<T>` alias to `<<T as HasPayload>::Payload as Payload>::Output`
- `Payload`, `HasPayload` and `Request` traits which represent different parts of the request ([#5][pr5])
- `GetUpdatesNonStrict` 'telegram' method, that behaves just like `GetUpdates` but doesn't [#2][pr2]
  fail if one of updates fails to be deserialized 
- Move core code here from the [`teloxide`] main repo, for older changes see it's [`CHANGELOG.md`].
  - Following modules were moved:
    - `bot`
    - `requests` [except `requests::respond` function]
    - `types`
    - `errors`
    - `net` [private] 
  - `client_from_env` was moved from `teloxide::utils` to crate root of `teloxide-core`
  - To simplify `GetUpdates` request it was changed to simply return `Vec<Update>` 
    (instead of `Vec<Result<Update, (Value, serde_json::Error)>>`)

[pr2]: https://github.com/teloxide/teloxide-core/pull/2
[pr5]: https://github.com/teloxide/teloxide-core/pull/5
[pr6]: https://github.com/teloxide/teloxide-core/pull/6
[pr7]: https://github.com/teloxide/teloxide-core/pull/7
[pr8]: https://github.com/teloxide/teloxide-core/pull/8
[pr10]: https://github.com/teloxide/teloxide-core/pull/10
[pr12]: https://github.com/teloxide/teloxide-core/pull/12
[pr14]: https://github.com/teloxide/teloxide-core/pull/14
[pr22]: https://github.com/teloxide/teloxide-core/pull/22
[pr24]: https://github.com/teloxide/teloxide-core/pull/24
[pr26]: https://github.com/teloxide/teloxide-core/pull/26
[pr27]: https://github.com/teloxide/teloxide-core/pull/27
[pr35]: https://github.com/teloxide/teloxide-core/pull/35
[pr39]: https://github.com/teloxide/teloxide-core/pull/39
[pr46]: https://github.com/teloxide/teloxide-core/pull/46
[pr49]: https://github.com/teloxide/teloxide-core/pull/49
[pr50]: https://github.com/teloxide/teloxide-core/pull/50

### Changed

- Cleanup setters in `types::*` (remove most of them) ([#44][pr44])
- Refactor `KeyboardButtonPollType` ([#44][pr44])
- Replace `Into<Vec<_>>` by `IntoIterator<Item = _>` in function arguments ([#44][pr44])
- Update dependencies (including tokio 1.0) ([#37][pr37])
- Refactor file downloading ([#30][pr30]):
  - Make `net` module public
  - Move `Bot::download_file{,_stream}` methods to a new `Download` trait
    - Impl `Download` for all bot adaptors & the `Bot` itself
  - Change return type of `download_file_stream` — return `Stream<Result<Bytes>>``,
    instead of `Future<Result<Stream<Result<Bytes>>>>``
  - Add `api_url` param to standalone versions of `download_file{,_stream}`
  - Make `net::{TELEGRAM_API_URL, download_file{,_stream}}` pub
- Refactor `Bot` ([#29][pr29]):
  - Move default parse mode to an adaptor (`DefaultParseMode`)
  - Remove bot builder (it's not usefull anymore, since parse_mode is moved away)
  - Undeprecate bot constructors (`Bot::{new, with_client, from_env_with_client}`)
- Rename `StickerType` => `InputSticker`, `{CreateNewStickerSet,AddStickerToSet}::sticker_type}` => `sticker` ([#23][pr23], [#43][pr43])
- Use `_: IntoIterator<Item = T>` bound instead of `_: Into<Vec<T>>` in telegram methods which accept collections ([#21][pr21])
- Make `MessageDice::dice` pub ([#20][pr20])
- Merge `ApiErrorKind` and `KnownApiErrorKind` into `ApiError` ([#13][pr13])
- Refactor ChatMember ([#9][pr9])
  - Replace a bunch of `Option<_>` fields with `ChatMemberKind`
  - Remove setters (users are not expected to create this struct)
  - Add getters
- Changed internal mechanism of sending multipart requests ([#1][pr1])
- Added `RequestError::Io(io::Error)` to wrap I/O error those can happen while sending files to telegram
- Make all fields of all methods `pub` ([#3][pr3])

[pr1]: https://github.com/teloxide/teloxide-core/pull/1
[pr3]: https://github.com/teloxide/teloxide-core/pull/3
[pr9]: https://github.com/teloxide/teloxide-core/pull/9
[pr13]: https://github.com/teloxide/teloxide-core/pull/13
[pr20]: https://github.com/teloxide/teloxide-core/pull/20
[pr21]: https://github.com/teloxide/teloxide-core/pull/21
[pr23]: https://github.com/teloxide/teloxide-core/pull/23
[pr29]: https://github.com/teloxide/teloxide-core/pull/29
[pr30]: https://github.com/teloxide/teloxide-core/pull/30
[pr37]: https://github.com/teloxide/teloxide-core/pull/37
[pr43]: https://github.com/teloxide/teloxide-core/pull/43

### Removed

- `unstable-stream` feature (now `Bot::download_file_stream` is accesable by default)
- old `Request` trait
- `RequestWithFile`, now multipart requests use `Request`
- Remove all `#[non_exhaustive]` annotations ([#4][pr4])
- Remove `MessageEntity::text_from` because it's wrong ([#44][pr44])

[pr4]: https://github.com/teloxide/teloxide-core/pull/4
[pr44]: https://github.com/teloxide/teloxide-core/pull/44
  

[`teloxide`]: https://github.com/teloxide/teloxide
[`CHANGELOG.md`]: https://github.com/teloxide/teloxide/blob/master/CHANGELOG.md
