use std::collections::HashSet;

use dptree::{description::EventKind, HandlerDescription};
use teloxide_core::types::AllowedUpdate;

/// Handler description that is used by [`Dispatcher`].
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct DpHandlerDescription {
    allowed: EventKind<AllowedUpdate>,
}

impl DpHandlerDescription {
    pub(crate) fn of(allowed: AllowedUpdate) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(allowed);
        Self { allowed: EventKind::InterestList(set) }
    }

    pub(crate) fn allowed_updates(&self) -> Vec<AllowedUpdate> {
        use AllowedUpdate::*;

        match &self.allowed {
            EventKind::InterestList(set) => set.iter().copied().collect(),
            EventKind::Entry => panic!("No updates were allowed"),
            EventKind::UserDefined => vec![
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

impl HandlerDescription for DpHandlerDescription {
    fn entry() -> Self {
        Self { allowed: HandlerDescription::entry() }
    }

    fn user_defined() -> Self {
        Self { allowed: HandlerDescription::user_defined() }
    }

    fn merge_chain(&self, other: &Self) -> Self {
        Self { allowed: self.allowed.merge_chain(&other.allowed) }
    }

    fn merge_branch(&self, other: &Self) -> Self {
        Self { allowed: self.allowed.merge_branch(&other.allowed) }
    }
}
