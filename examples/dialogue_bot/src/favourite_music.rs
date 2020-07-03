use parse_display::{Display, FromStr};

use teloxide::types::{KeyboardButton, ReplyKeyboardMarkup};

#[derive(Copy, Clone, Display, FromStr)]
pub enum FavouriteMusic {
    Rock,
    Metal,
    Pop,
    Other,
}

impl FavouriteMusic {
    pub fn markup() -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::default().append_row(vec![
            KeyboardButton::new("Rock"),
            KeyboardButton::new("Metal"),
            KeyboardButton::new("Pop"),
            KeyboardButton::new("Other"),
        ])
    }
}
