#![allow(clippy::trivial_regex)]

#[macro_use]
extern crate smart_default;

use teloxide::{
    prelude::*,
    types::{KeyboardButton, ReplyKeyboardMarkup},
};

use parse_display::{Display, FromStr};

// ============================================================================
// [Favourite music kinds]
// ============================================================================

#[derive(Copy, Clone, Display, FromStr)]
enum FavouriteMusic {
    Rock,
    Metal,
    Pop,
    Other,
}

impl FavouriteMusic {
    fn markup() -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::default().append_row(vec![
            KeyboardButton::new("Rock"),
            KeyboardButton::new("Metal"),
            KeyboardButton::new("Pop"),
            KeyboardButton::new("Other"),
        ])
    }
}

// ============================================================================
// [A type-safe finite automaton]
// ============================================================================

#[derive(Clone)]
struct ReceiveAgeState {
    full_name: String,
}

#[derive(Clone)]
struct ReceiveFavouriteMusicState {
    data: ReceiveAgeState,
    age: u8,
}

#[derive(Display)]
#[display(
    "Your full name: {data.data.full_name}, your age: {data.age}, your \
     favourite music: {favourite_music}"
)]
struct ExitState {
    data: ReceiveFavouriteMusicState,
    favourite_music: FavouriteMusic,
}

#[derive(SmartDefault)]
enum Dialogue {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge(ReceiveAgeState),
    ReceiveFavouriteMusic(ReceiveFavouriteMusicState),
}

// ============================================================================
// [Control a dialogue]
// ============================================================================

type Ctx<State> = DialogueHandlerCtx<Message, State>;
type Res = Result<DialogueStage<Dialogue>, RequestError>;

async fn start(ctx: Ctx<()>) -> Res {
    ctx.answer("Let's start! First, what's your full name?")
        .send()
        .await?;
    next(Dialogue::ReceiveFullName)
}

async fn full_name(ctx: Ctx<()>) -> Res {
    match ctx.update.text() {
        None => {
            ctx.answer("Please, send me a text message!").send().await?;
            next(Dialogue::ReceiveFullName)
        }
        Some(full_name) => {
            ctx.answer("What a wonderful name! Your age?")
                .send()
                .await?;
            next(Dialogue::ReceiveAge(ReceiveAgeState {
                full_name: full_name.to_owned(),
            }))
        }
    }
}

async fn age(ctx: Ctx<ReceiveAgeState>) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(age) => {
            ctx.answer("Good. Now choose your favourite music:")
                .reply_markup(FavouriteMusic::markup())
                .send()
                .await?;
            next(Dialogue::ReceiveFavouriteMusic(
                ReceiveFavouriteMusicState {
                    data: ctx.dialogue,
                    age,
                },
            ))
        }
        Err(_) => {
            ctx.answer("Oh, please, enter a number!").send().await?;
            next(Dialogue::ReceiveAge(ctx.dialogue))
        }
    }
}

async fn favourite_music(ctx: Ctx<ReceiveFavouriteMusicState>) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(favourite_music) => {
            ctx.answer(format!(
                "Fine. {}",
                ExitState {
                    data: ctx.dialogue.clone(),
                    favourite_music
                }
            ))
            .send()
            .await?;
            exit()
        }
        Err(_) => {
            ctx.answer("Oh, please, enter from the keyboard!")
                .send()
                .await?;
            next(Dialogue::ReceiveFavouriteMusic(ctx.dialogue))
        }
    }
}

async fn handle_message(ctx: Ctx<Dialogue>) -> Res {
    match ctx {
        DialogueHandlerCtx {
            bot,
            update,
            dialogue: Dialogue::Start,
        } => start(DialogueHandlerCtx::new(bot, update, ())).await,
        DialogueHandlerCtx {
            bot,
            update,
            dialogue: Dialogue::ReceiveFullName,
        } => full_name(DialogueHandlerCtx::new(bot, update, ())).await,
        DialogueHandlerCtx {
            bot,
            update,
            dialogue: Dialogue::ReceiveAge(s),
        } => age(DialogueHandlerCtx::new(bot, update, s)).await,
        DialogueHandlerCtx {
            bot,
            update,
            dialogue: Dialogue::ReceiveFavouriteMusic(s),
        } => favourite_music(DialogueHandlerCtx::new(bot, update, s)).await,
    }
}

// ============================================================================
// [Run!]
// ============================================================================

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    log::info!("Starting dialogue_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .message_handler(&DialogueDispatcher::new(|ctx| async move {
            handle_message(ctx)
                .await
                .expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
