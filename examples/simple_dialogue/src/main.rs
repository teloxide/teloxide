#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate smart_default;

use std::fmt::{self, Display, Formatter};
use teloxide::{
    prelude::*,
    types::{KeyboardButton, ReplyKeyboardMarkup},
};

// ============================================================================
// [Favourite music kinds]
// ============================================================================

#[derive(Copy, Clone, Display, EnumString)]
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
// [A user's data]
// ============================================================================

#[derive(Default)]
struct User {
    full_name: Option<String>,
    age: Option<u8>,
    favourite_music: Option<FavouriteMusic>,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Your full name: {}, your age: {}, your favourite music: {}",
            self.full_name.as_ref().unwrap(),
            self.age.unwrap(),
            self.favourite_music.unwrap()
        )
    }
}

// ============================================================================
// [States of a dialogue]
// ============================================================================

#[derive(SmartDefault)]
enum State {
    #[default]
    Start,
    FullName,
    Age,
    FavouriteMusic,
}

// ============================================================================
// [Control a dialogue]
// ============================================================================

type Ctx = DialogueHandlerCtx<Message, State, User>;
type Res = Result<DialogueStage<State, User>, RequestError>;

async fn send_favourite_music_types(ctx: &Ctx) -> Result<(), RequestError> {
    ctx.bot
        .send_message(ctx.chat_id(), "Good. Now choose your favourite music:")
        .reply_markup(FavouriteMusic::markup())
        .send()
        .await?;
    Ok(())
}

async fn start(mut ctx: Ctx) -> Res {
    ctx.reply("Let's start! First, what's your full name?")
        .await?;
    ctx.dialogue.state = State::FullName;
    next(ctx.dialogue)
}

async fn full_name(mut ctx: Ctx) -> Res {
    ctx.reply("What a wonderful name! Your age?").await?;
    ctx.dialogue.data.full_name = Some(ctx.update.text().unwrap().to_owned());
    ctx.dialogue.state = State::Age;
    next(ctx.dialogue)
}

async fn age(mut ctx: Ctx) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(ok) => {
            send_favourite_music_types(&ctx).await?;
            ctx.dialogue.data.age = Some(ok);
            ctx.dialogue.state = State::FavouriteMusic;
        }
        Err(_) => ctx.reply("Oh, please, enter a number!").await?,
    }

    next(ctx.dialogue)
}

async fn favourite_music(mut ctx: Ctx) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(ok) => {
            ctx.dialogue.data.favourite_music = Some(ok);
            ctx.reply(format!("Fine. {}", ctx.dialogue.data)).await?;
            exit()
        }
        Err(_) => {
            ctx.reply("Oh, please, enter from the keyboard!").await?;
            next(ctx.dialogue)
        }
    }
}

async fn handle_message(ctx: Ctx) -> Res {
    match ctx.dialogue.state {
        State::Start => start(ctx).await,
        State::FullName => full_name(ctx).await,
        State::Age => age(ctx).await,
        State::FavouriteMusic => favourite_music(ctx).await,
    }
}

// ============================================================================
// [Run!]
// ============================================================================

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "simple_dialogue=trace");
    pretty_env_logger::init();
    log::info!("Starting the simple_dialogue bot!");

    Dispatcher::new(Bot::new("YourAwesomeToken"))
        .message_handler(&DialogueDispatcher::new(|ctx| async move {
            handle_message(ctx)
                .await
                .expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
