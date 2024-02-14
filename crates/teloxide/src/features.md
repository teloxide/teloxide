## Cargo features

| Feature              | Description |
|----------------------|-------------|
| `webhooks`           | Enables general webhook utilities (almost useless on its own). |
| `webhooks-axum`      | Enables webhook implementation based on axum framework. |
| `macros`             | Re-exports macros from [`teloxide-macros`]. |
| `ctrlc_handler`      | Enables the [`DispatcherBuilder::enable_ctrlc_handler`] function (**enabled by default**). |
| `throttle`           | Enables the [`Throttle`](adaptors::Throttle) bot adaptor. |
| `cache-me`           | Enables the [`CacheMe`](adaptors::CacheMe) bot adaptor. |
| `trace-adaptor`      | Enables the [`Trace`](adaptors::Trace) bot adaptor. |
| `erased`             | Enables the [`ErasedRequester`](adaptors::ErasedRequester) bot adaptor. |
| `full`               | Enables all the features except `nightly`. |
| `nightly`            | Enables nightly-only features (see the [`teloxide-core` features]). |
| `native-tls`         | Enables the [`native-tls`] TLS implementation (**enabled by default**). |
| `rustls`             | Enables the [`rustls`] TLS implementation. |
| `redis-storage`      | Enables the [Redis] storage support for dialogues. |
| `sqlite-storage-nativetls`     | Enables the [Sqlite] storage support for dialogues (depends on `native-tls`). |
| `sqlite-storage-rustls`     | Enables the [Sqlite] storage support for dialogues (depends on `rustls`, conflicts with `sqlite-storage-nativetls`). |
| `cbor-serializer`    | Enables the [CBOR] serializer for dialogues. |
| `bincode-serializer` | Enables the [Bincode] serializer for dialogues. |

[Redis]: https://redis.io/
[Sqlite]: https://www.sqlite.org/
[CBOR]: https://en.wikipedia.org/wiki/CBOR
[Bincode]: https://github.com/servo/bincode
[`teloxide-macros`]: https://github.com/teloxide/teloxide-macros
[`native-tls`]: https://docs.rs/native-tls
[`rustls`]: https://docs.rs/rustls
[`teloxide::utils::UpState`]: utils::UpState
[`teloxide-core` features]: https://docs.rs/teloxide-core/latest/teloxide_core/#cargo-features

[`DispatcherBuilder::enable_ctrlc_handler`]: dispatching::DispatcherBuilder::enable_ctrlc_handler
