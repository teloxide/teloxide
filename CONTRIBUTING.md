# Contributing
Before contributing, please read our [code of conduct](https://github.com/teloxide/teloxide/blob/dev/CODE_OF_CONDUCT.md) and [the code style](https://github.com/teloxide/teloxide/blob/dev/CODE_STYLE.md).

To change the source code, fork this repository and work inside your own branch. Then send us a PR and wait for the CI to check everything. However, you'd better check changes first locally:

```
cargo clippy --all --all-features --all-targets
cargo test --all
cargo doc --open
cargo fmt --all -- --check
```
