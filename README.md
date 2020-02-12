<div align="center">
  <img src="ICON.png" width="250"/>
  <h1>teloxide</h1>
  
  <a href="https://docs.rs/teloxide/">
    <img src="https://img.shields.io/badge/docs.rs-v0.1.0-blue.svg">
  </a>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloxide">
    <img src="https://img.shields.io/badge/crates.io-v0.1.0-orange.svg">
  </a>
  
  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Getting started
 1. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 2. Be sure that you are up to date:
```bash
$ rustup update stable
```

 3. Execute `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = "0.1.0"
```

## The ping-pong bott!
```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // Configure a fancy logger. Let this bot print everything, but restrict
    // teloxide to only log errors.
    std::env::set_var("RUST_LOG", "ping_pong_bot=trace");
    std::env::set_var("RUST_LOG", "teloxide=error");
    pretty_env_logger::init();
    log::info!("Starting the ping-pong bot!");

    // Creates a dispatcher of updates with the specified bot. Don't forget to
    // replace `MyAwesomeToken` with yours.
    Dispatcher::<RequestError>::new(Bot::new("MyAwesomeToken"))
        // Registers a message handler. Inside a body of the closure, answer
        // `"pong"` to an incoming message.
        .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
            ctx.answer("pong").send().await?;
            Ok(())
        })
        .dispatch()
        .await;
}
```
