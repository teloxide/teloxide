# Teloxide by Example

> **Why Examples?**  
> Learning a framework is much easier with real-world code!  
> While the [API docs](https://docs.rs/teloxide/) explain _what_ the functions do, these examples show you _how_ to use them effectively in real Telegram bots.

This folder is a practical, example-driven guide to using [`teloxide`](https://github.com/teloxide/teloxide).

Inside, you'll find a collection of runnable, self-contained examples that demonstrate everything from basic message handling to advanced features like webhooks, dialogues, and middleware.

Whether you're just getting started or exploring more advanced use cases, you'll find examples here to guide you at every step.

> **New to `teloxide`?**  
> Start with the [Suggested Learning Path](#suggested-learning-path) to build your understanding step-by-step.

## Table of Contents

- [Running the Examples](#running-the-examples)
- [Set Up Your Bot Token](#set-up-your-bot-token)
- [Example](#example)
- [Suggested Learning Path](#suggested-learning-path)
- [Available Examples](#available-examples)
- [Categorized Examples](#categorized-examples)


## Running the Examples

To run an example, use the following command:

```bash
RUST_LOG=info cargo run --features "full" --example <example-name>
```

> Replace `<example-name>` with one of the files from the `examples/` directory, like `throw_dice` or `dialogue`.

> Most examples use async features and require the `"full"` feature flag to include macros and runtime support.

## Set Up Your Bot Token

Before running any example, make sure you've set the `TELOXIDE_TOKEN` environment variable. You can get a bot token from [https://t.me/botfather](@BotFather) on Telegram.

```bash
# Unix-like systems
export TELOXIDE_TOKEN=xxxxxxxxx:xxxxxxxxx

# Windows CMD
set TELOXIDE_TOKEN=xxxxxxxxx:xxxxxxxxx

# PowerShell
$env:TELOXIDE_TOKEN = "xxxxxxxxx:xxxxxxxxx"
```

## Example: Run the Dice Bot

```bash
RUST_LOG=info cargo run --features "full" --example throw_dice
```

This will start a bot that replies with a random dice emoji to each incoming message.

## Suggested Learning Path

If you're new to `teloxide`, try these examples in order:

1. [`throw_dice`](throw_dice.rs) – basic message handling
2. [`command`](command.rs) – parsing bot commands
3. [`dialogue`](dialogue.rs) – stateful interactions using dialogues
4. [`db_remember`](db_remember.rs) – persistence with Redis/SQLite
5. [`dispatching_features`](dispatching_features.rs) – advanced update routing

## Available Examples

| Name                 | Description                                                                               |
|----------------------|-------------------------------------------------------------------------------------------|
| throw_dice           | Replies with a random dice emoji for every message. A great starting point.               |
| command              | Parses text commands into enums using BotCommands.                                        |
| admin                | Admin bot with Kick, Ban, and Mute functionality using time units.                        |
| buttons              | Demonstrates inline keyboards with buttons, callback queries, and inline queries.         |
| inline               | Replies to inline queries with custom search result cards (e.g. Google/DuckDuckGo).       |
| chat_member_updates  | Welcomes new users and says goodbye when users leave a group chat.                        |
| dialogue             | A dialogue FSM that collects name, age, and location from the user.                       |
| purchase             | Mixed dialogue with both messages and callback queries for product selection.             |
| deep_linking         | Demonstrates deep-linking behavior (/start <payload>) for Telegram bots.                  |
| db_remember          | Dialogue with persistent storage via Redis or Sqlite.                                     |
| shared_state         | Tracks message count using Arc<AtomicU64> as shared state.                                |
| dispatching_features | Shows advanced dispatching capabilities including multi-command routing.                  |
| middlewares          | Middleware example that inspects updates before and after endpoint execution.             |
| middlewares_fallible | Like middlewares, but demonstrates fallible logic with error handling.                    |
| ngrok_ping_pong      | Webhook-based ping-pong bot using an ngrok tunnel.                                        |
| heroku_ping_pong     | Webhook bot ready to deploy on Heroku with automatic port and host setup.                 |

## Categorized Examples

Here are the examples grouped by topic to help you explore specific teloxide features:

- **Beginner**
  - [`throw_dice`](throw_dice.rs)
  - [`command`](command.rs)
  - [`shared_state`](shared_state.rs)

- **Dialogues**
  - [`dialogue`](dialogue.rs)
  - [`db_remember`](db_remember.rs)
  - [`purchase`](purchase.rs)

- **Advanced Routing**
  - [`dispatching_features`](dispatching_features.rs)
  - [`middlewares`](middlewares.rs)
  - [`middlewares_fallible`](middlewares_fallible.rs)

- **Webhooks**
  - [`ngrok_ping_pong`](ngrok_ping_pong.rs)
  - [`heroku_ping_pong`](heroku_ping_pong.rs)

- **Inline Features**
  - [`buttons`](buttons.rs)
  - [`inline`](inline.rs)

- **Group Interaction**
  - [`admin`](admin.rs)
  - [`chat_member_updates`](chat_member_updates.rs)
  - [`deep_linking`](deep_linking.rs)
