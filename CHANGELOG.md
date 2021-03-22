# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

## [0.2.2] - 2020-03-22

### Fixed

- Typo: `ReplyMarkup::{keyboad => keyboard}` ([#69][pr69])
  - Note: method with the old name was deprecated and hidden from docs

[pr69]: https://github.com/teloxide/teloxide-core/pull/69

## [0.2.1] - 2020-03-19

### Fixed 

- Types fields privacy (make fields of some types public) ([#68][pr68])
  - `Dice::{emoji, value}`
  - `MessageMessageAutoDeleteTimerChanged::message_auto_delete_timer_changed`
  - `PassportElementError::{message, kind}`
  - `StickerSet::thumb`

[pr68]: https://github.com/teloxide/teloxide-core/pull/68

## [0.2.0] - 2020-03-16

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

## [0.1.1] - 2020-02-17

### Fixed

- Remove `dbg!` call from internals ([#53][pr53])

[pr53]: https://github.com/teloxide/teloxide-core/pull/53

## [0.1.0] - 2020-02-17

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
