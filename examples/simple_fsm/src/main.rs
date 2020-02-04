#[macro_use]
extern crate strum_macros;

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
// [FSM - Finite-State Machine]
// ============================================================================

enum Fsm {
    Start,
    FullName,
    Age,
    FavouriteMusic,
}

impl Default for Fsm {
    fn default() -> Self {
        Self::Start
    }
}

// ============================================================================
// [Our Session type]
// ============================================================================

#[derive(Default)]
struct Session {
    user: User,
    fsm: Fsm,
}

// ============================================================================
// [Control our FSM]
// ============================================================================

type Ctx = SessionHandlerCtx<Message, Session>;
type Res = Result<SessionState<Session>, RequestError>;

async fn send_favourite_music_types(ctx: &Ctx) -> Result<(), RequestError> {
    ctx.bot
        .send_message(ctx.chat_id(), "Good. Now choose your favourite music:")
        .reply_markup(FavouriteMusic::markup())
        .send()
        .await?;
    Ok(())
}

async fn start(ctx: Ctx) -> Res {
    ctx.reply("Let's start! First, what's your full name?")
        .await?;
    ctx.session.state = Fsm::FullName;
    Ok(SessionState::Next(ctx.session))
}

async fn full_name(mut ctx: Ctx) -> Res {
    ctx.reply("What a wonderful name! Your age?").await?;
    ctx.session.user.full_name = Some(ctx.update.text().unwrap().to_owned());
    ctx.session.fsm = Fsm::Age;
    Ok(SessionState::Next(ctx.session))
}

async fn age(mut ctx: Ctx) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(ok) => {
            send_favourite_music_types(&ctx).await?;
            ctx.session.user.age = Some(ok);
            ctx.session.fsm = Fsm::FavouriteMusic;
        }
        Err(_) => ctx.reply("Oh, please, enter a number!").await?,
    }

    Ok(SessionState::Next(ctx.session))
}

async fn favourite_music(mut ctx: Ctx) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(ok) => {
            ctx.session.user.favourite_music = Some(ok);
            ctx.reply(format!("Fine. {}", ctx.session.user)).await?;
            Ok(SessionState::Exit)
        }
        Err(_) => {
            ctx.reply("Oh, please, enter from the keyboard!").await?;
            Ok(SessionState::Next(ctx.session))
        }
    }
}

async fn handle_message(ctx: Ctx) -> Res {
    match ctx.session.fsm {
        Fsm::Start => start(ctx).await,
        Fsm::FullName => full_name(ctx).await,
        Fsm::Age => age(ctx).await,
        Fsm::FavouriteMusic => favourite_music(ctx).await,
    }
}

// ============================================================================
// [Run!]
// ============================================================================

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "simple_fsm=trace");
    pretty_env_logger::init();
    log::info!("Starting the simple_fsm bot!");

    Dispatcher::new(Bot::new("YourAwesomeToken"))
        .message_handler(SessionDispatcher::new(|ctx| async move {
            handle_message(ctx)
                .await
                .expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
