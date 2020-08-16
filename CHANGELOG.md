# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

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

### Changed

- Changed internal mechanism of sending multipart requests
- Added `RequestError::Io(io::Error)` to wrap I/O error those can happen while sending files to telegram
- Change `StickerType`: instead of newtypes (`Png(InputFile)`) use structs (`Png { png_sticker: InputFile }`), add 
  `StickerType::{png,tgs}` constructors
- Make all fields of all methods `pub`

### Removed

- `RequestWithFile`, now multipart requests use `Request`
- Remove all `#[non_exhaustive]` annotations
  

[`teloxide`]: https://github.com/teloxide/teloxide
[`CHANGELOG.md`]: https://github.com/teloxide/teloxide/blob/master/CHANGELOG.md