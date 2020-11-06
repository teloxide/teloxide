# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- `throttle`, `cache_me`, `auto_send` and `full` crate features
- `payloads` module
- `RequesterExt` trait which is implemented for all `Requester`s and allows easily wrapping them in adaptors
- `adaptors` module
  - Request throttling - opt-in feature represented by `Throttle` bot adapter which allows automatically checking telegram limits ([#10][pr10])
  - Request auto sending - ability to `.await` requests without need to call `.send()` (opt-in feature represented by `AutoSend` bot adapter, [#8][pr8])
  - `get_me` caching (opt-in feature represented by `CacheMe` bot adapter)
- `Requester` trait which represents bot-clients ([#7][pr7])
- `{Json,Multipart}Request` the `Bot` requests types ([#6][pr6])
- `Output<T>` alias to `<<T as HasPayload>::Payload as Payload>::Output`
- `Payload`, `HasPayload` and `Request` traits which represent different parts of the request ([#5][pr5])
- `GetUpdatesNonStrict` - fail proof version of `GetUpdates`
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
- `GetUpdatesNonStrict` 'telegram' method, that behaves just like `GetUpdates` but doesn't 
  fail if one of updates fails to be deserialized 

[pr5]: https://github.com/teloxide/teloxide-core/pull/5
[pr6]: https://github.com/teloxide/teloxide-core/pull/6
[pr7]: https://github.com/teloxide/teloxide-core/pull/7
[pr8]: https://github.com/teloxide/teloxide-core/pull/8
[pr10]: https://github.com/teloxide/teloxide-core/pull/10

### Changed

- Rename `StickerType` => `InputSticker`, `{CreateNewStickerSet,AddStickerToSet}::sticker_type}` => `sticker` ([#23][pr23])
- Use `_: IntoIterator<Item = T>` bound instead of `_: Into<Vec<T>>` in telegram methods which accept collections ([#21][pr21])
- Make `MessageDice::dice` pub ([#20][pr20])
- Merge `ApiErrorKind` and `KnownApiErrorKind` into `ApiError` ([#13][pr13])
- Refactor ChatMember ([#9][pr9])
  - Replace a bunch of `Option<_>` fields with `ChatMemberKind`
  - Remove setters (users are not expected to create this struct)
  - Add getters
- Changed internal mechanism of sending multipart requests
- Added `RequestError::Io(io::Error)` to wrap I/O error those can happen while sending files to telegram
- Change `StickerType`: instead of newtypes (`Png(InputFile)`) use structs (`Png { png_sticker: InputFile }`), add 
  `StickerType::{png,tgs}` constructors
- Make all fields of all methods `pub`

[pr9]: https://github.com/teloxide/teloxide-core/pull/9
[pr13]: https://github.com/teloxide/teloxide-core/pull/13
[pr20]: https://github.com/teloxide/teloxide-core/pull/20
[pr21]: https://github.com/teloxide/teloxide-core/pull/21
[pr23]: https://github.com/teloxide/teloxide-core/pull/23

### Removed

- `unstable-stream` feature (now `Bot::download_file_stream` is accesable by default)
- old `Request` trait
- `RequestWithFile`, now multipart requests use `Request`
- Remove all `#[non_exhaustive]` annotations
  

[`teloxide`]: https://github.com/teloxide/teloxide
[`CHANGELOG.md`]: https://github.com/teloxide/teloxide/blob/master/CHANGELOG.md