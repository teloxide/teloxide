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
        ReplyKeyboardMarkup {
            keyboard: vec![vec![
                KeyboardButton {
                    text: "Rock".to_owned(),
                    request: None,
                },
                KeyboardButton {
                    text: "Metal".to_owned(),
                    request: None,
                },
                KeyboardButton {
                    text: "Pop".to_owned(),
                    request: None,
                },
                KeyboardButton {
                    text: "Other".to_owned(),
                    request: None,
                },
            ]],
            resize_keyboard: None,
            one_time_keyboard: None,
            selective: None,
        }
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
// [Some macros]
// ============================================================================

#[macro_export]
macro_rules! reply {
    ($ctx:ident, $text:expr) => {
        $ctx.reply($text).await?;
    };
}

// ============================================================================
// [Control our FSM]
// ============================================================================

async fn send_favourite_music_types(ctx: &Ctx) -> Result<(), RequestError> {
    ctx.bot
        .send_message(ctx.chat_id(), "Good. Now choose your favourite music:")
        .reply_markup(FavouriteMusic::markup())
        .send()
        .await?;
    Ok(())
}

type Ctx = SessionHandlerCtx<Message, User>;
type Res = Result<SessionState<User>, RequestError>;

async fn start(ctx: Ctx) -> Res {
    reply!(ctx, "Let's start! First, what's your full name?");
    Ok(SessionState::Next(ctx.session))
}

async fn full_name(mut ctx: Ctx) -> Res {
    reply!(ctx, "What a wonderful name! Your age?");
    ctx.session.full_name = Some(ctx.update.text().unwrap().to_owned());
    Ok(SessionState::Next(ctx.session))
}

async fn age(mut ctx: Ctx) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(ok) => {
            send_favourite_music_types(&ctx).await?;
            ctx.session.age = Some(ok);
        }
        Err(_) => reply!(ctx, "Oh, please, enter a number!"),
    }

    Ok(SessionState::Next(ctx.session))
}

async fn favourite_music(mut ctx: Ctx) -> Res {
    match ctx.update.text().unwrap().parse() {
        Ok(ok) => {
            ctx.session.favourite_music = Some(ok);
            reply!(ctx, format!("Fine. {}", ctx.session));
            Ok(SessionState::Exit)
        }
        Err(_) => {
            reply!(ctx, "Oh, please, enter from the keyboard!");
            Ok(SessionState::Next(ctx.session))
        }
    }
}

async fn handle_message(ctx: Ctx) -> Res {
    if ctx.session.full_name.is_none() {
        return full_name(ctx).await;
    }

    if ctx.session.age.is_none() {
        return age(ctx).await;
    }

    if ctx.session.favourite_music.is_none() {
        return favourite_music(ctx).await;
    }

    Ok(SessionState::Exit)
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
