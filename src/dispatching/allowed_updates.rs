use std::collections::HashSet;

use dptree::{MaybeSpecial, UpdateSet};
use teloxide_core::types::AllowedUpdate;

pub struct AllowedUpdates {
    inner: MaybeSpecial<HashSet<AllowedUpdate>>,
}

impl AllowedUpdates {
    pub(crate) fn of(allowed: AllowedUpdate) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(allowed);
        Self { inner: MaybeSpecial::Known(set) }
    }

    pub(crate) fn get_param(&self) -> Vec<AllowedUpdate> {
        use AllowedUpdate::*;

        match &self.inner {
            MaybeSpecial::Known(set) => set.iter().cloned().collect(),
            MaybeSpecial::Invisible => panic!("No updates were allowed"),
            MaybeSpecial::Unknown => vec![
                Message,
                EditedMessage,
                ChannelPost,
                EditedChannelPost,
                InlineQuery,
                ChosenInlineResult,
                CallbackQuery,
                ShippingQuery,
                PreCheckoutQuery,
                Poll,
                PollAnswer,
                MyChatMember,
                ChatMember,
            ],
        }
    }
}

impl UpdateSet for AllowedUpdates {
    fn unknown() -> Self {
        Self { inner: UpdateSet::unknown() }
    }

    fn invisible() -> Self {
        Self { inner: UpdateSet::invisible() }
    }

    fn union(&self, other: &Self) -> Self {
        Self { inner: self.inner.union(&other.inner) }
    }

    fn intersection(&self, other: &Self) -> Self {
        Self { inner: self.inner.intersection(&other.inner) }
    }
}
