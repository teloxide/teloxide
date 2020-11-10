use crate::types::{
    CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, Message,
    ReplyKeyboardMarkup,
};

/// `View` is used to prevent errors on defining `View` type in `ViewFactory`
/// trait. It is implement for: 1. `InlineKeyboardButton`.
/// 2. `InlineKeyboardMarkup`.
/// 3. `ReplyKeyboardMarkup`.
/// 4. `KeyboardButton`.
///
/// It is sealed trait, so you cannot implement it for your types.
pub trait View: seal::SealedView {
    type ProducedUpdate;
}

// SealedView prevents the implement View trait on user types to prevent logic
// errors.
mod seal {
    pub trait SealedView {}
}
impl seal::SealedView for InlineKeyboardButton {}
impl seal::SealedView for InlineKeyboardMarkup {}
impl seal::SealedView for ReplyKeyboardMarkup {}
impl seal::SealedView for KeyboardButton {}

// Someone buttons cannot produce an callback query, we are know.
impl View for InlineKeyboardButton {
    type ProducedUpdate = CallbackQuery;
}
impl View for InlineKeyboardMarkup {
    type ProducedUpdate = CallbackQuery;
}
impl View for ReplyKeyboardMarkup {
    type ProducedUpdate = Message;
}
impl View for KeyboardButton {
    type ProducedUpdate = Message;
}
