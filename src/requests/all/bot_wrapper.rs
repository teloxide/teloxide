use crate::Bot;
use std::ops::Deref;

/// A wrapper that implements `Clone`, Copy, `PartialEq`, `Eq`, `Debug`, but
/// performs no copying, cloning and comparison.
///
/// Used in the requests bodies.
#[derive(Debug)]
pub struct BotWrapper<'a>(pub &'a Bot);

impl PartialEq for BotWrapper<'_> {
    fn eq(&self, _: &BotWrapper<'_>) -> bool {
        true
    }
}

impl Eq for BotWrapper<'_> {}

impl<'a> Clone for BotWrapper<'a> {
    fn clone(&self) -> BotWrapper<'a> {
        Self(self.0)
    }
}

impl Copy for BotWrapper<'_> {}

impl Deref for BotWrapper<'_> {
    type Target = Bot;

    fn deref(&self) -> &Bot {
        &self.0
    }
}
