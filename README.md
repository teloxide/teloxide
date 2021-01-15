<div align="center">
  <img src="media/logo.svg" width="250"/>
</div>

# teloxide-core

[![CI status](https://github.com/teloxide/teloxide-core/workflows/Continuous%20integration/badge.svg)](https://github.com/teloxide/teloxide-core/actions)
[![documentation](https://docs.rs/teloxide_core/badge.svg)](https://docs.rs/teloxide_core/)
[![documentation (master)](https://img.shields.io/badge/docs-master-blue)](https://teloxide-core.netlify.com)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Api Cov](https://img.shields.io/badge/API%20coverage-Up%20to%200.4.9%20(inclusively)-green.svg)](https://core.telegram.org/bots/api)
[![crates.io](https://img.shields.io/crates/v/teloxide_core.svg)](https://crates.io/crates/teloxide_core)
[![Official Chat](https://img.shields.io/badge/official%20chat-t.me%2Fteloxide-blueviolet)](https://t.me/teloxide)



Core part of the [`teloxide`] library.

This library provides tools for making requests to the [Telegram Bot API]
(Currently, version `4.9` is supported) with ease. The library is fully
asynchronouns and built using [`tokio`].

```toml
teloxide_core = "0.1"
```
_Compiler support: requires rustc 1.49+_

[`teloxide`]: https://docs.rs/teloxide
[Telegram Bot API]: https://core.telegram.org/bots/api
[`tokio`]: https://tokio.rs