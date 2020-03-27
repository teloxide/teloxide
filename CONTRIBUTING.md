# Contributing
Before contributing, please read [our code style](https://github.com/teloxide/teloxide/blob/master/CODE_STYLE.md) and [the license](https://github.com/teloxide/teloxide/blob/master/LICENSE).

To change the source code, fork the `master` branch of this repository and work inside your own branch. Then send us a PR into `master` branch and wait for the CI to check everything. However, you'd better check changes first locally:

```
cargo clippy --all --all-features --all-targets
cargo test --all
cargo doc --open
cargo fmt --all -- --check
```

To report a bug, suggest new functionality, or ask a question, go to [Issues](https://github.com/teloxide/teloxide/issues). Try to make MRE (**M**inimal **R**eproducible **E**xample) and specify your teloxide version to let others help you.


And don't forget to switch to the nightly channel in order to be able to run `cargo fmt` (because our [rustfmt.toml](https://github.com/teloxide/teloxide/blob/master/rustfmt.toml) requires some nightly-only functionality):

```bash
$ rustup override set nightly
```
