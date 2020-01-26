> WARNING: this library is still in active development under v0.1.0, use it at your own risk!

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

# Getting started

Library requires rustc version more than 1.40.0. You can check your rustc version with command:
```shell script
rustup -V
```
If your rustc version lower than 1.40.0, update it with:
```shell script
rustup update stable
```
Than create Cargo project and write
```toml
teloxide = "0.1.0"
```
in your Cargo.toml file.

### Writing first bot
First, create bot with [@botfather](https://t.me/botfather). After creating, botfather give you
token in format `123456789:somemanyletters`.

Next, open yout `main.rs` file. Let's create a simple echo bot:
```rust
use futures::stream::StreamExt;
use teloxide::{
    dispatching::{
        chat::{ChatUpdate, ChatUpdateKind, Dispatcher},
        update_listeners::polling_default,
        SessionState,
    },
    requests::Request,
    Bot,
};

#[tokio::main]
async fn main() {
    let bot = &Bot::new("1061598315:AAErEDodTsrqD3UxA_EvFyEfXbKA6DT25G0");
    let mut updater = Box::pin(polling_default(bot));
    let handler = |_, upd: ChatUpdate| async move {
        if let ChatUpdateKind::Message(m) = upd.kind {
            let msg = bot.send_message(m.chat.id, m.text);
            msg.send().await.unwrap();
        }
        SessionState::Continue(())
    };
    let mut dp = Dispatcher::<'_, (), _>::new(handler);
    println!("Starting the message handler.");
    loop {
        let u = updater.next().await.unwrap();
        match u {
            Err(e) => eprintln!("Error: {}", e),
            Ok(u) => {
                let _ = dp.dispatch(u).await;
            }
        }
    }
}
```
