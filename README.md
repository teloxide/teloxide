
<div align="center">
  <h1>async-telegram-bot</h1>
  
  <a href="https://docs.rs/async-telegram-bot/">
    <img src="https://img.shields.io/badge/docs.rs-link-blue.svg">
  </a>
  <a href="https://travis-ci.com/async-telegram-bot/async-telegram-bot">
    <img src="https://travis-ci.com/async-telegram-bot/async-telegram-bot.svg?branch=dev" />
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg">
  </a>
  <a href="https://crates.io/crates/async-telegram-bot">
    <img src="https://img.shields.io/badge/crates.io-v0.1.0-orange.svg">
  </a>
  
  <br>
  <img src="ICON.png" width="300"/>
  <br>
  
  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## An echo bot
<table>
  <tr valign="top">
    <td>
    <h3>async-telegram-bot (Rust)</h3>
      <pre lang="rust">
const API_TOKEN: &str = "BOT TOKEN HERE";
<br>
fn main() {
  let bot = Bot::new(API_TOKEN).bla().bla();
}
      </pre>
    </td>
    <td>
    <h3>aiogram (Python)</h3>
      <pre lang="python">
import logging
from aiogram import Bot, Dispatcher, executor,
  types
<br>
API_TOKEN = 'BOT TOKEN HERE'
<br>
logging.basicConfig(level=logging.INFO)
<br>
bot = Bot(token=API_TOKEN)
dp = Dispatcher(bot)
<br>
@dp.message_handler(regexp='(^cat[s]?$|puss)')
async def cats(message: types.Message):
    with open('data/cats.jpg', 'rb') as photo:
        await message.reply_photo
          (photo, caption='Cats are here 😺')
<br>
@dp.message_handler()
async def echo(message: types.Message):
    await message.reply
      (message.text, reply=False)
<br>
executor.start_polling(dp, skip_updates=True)
      </pre>
    </td>
  </tr>
</table>
