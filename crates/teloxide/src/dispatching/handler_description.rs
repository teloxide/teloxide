use std::collections::HashSet;

use dptree::{
    description::{EventKind, InterestSet},
    HandlerDescription,
};
use teloxide_core::types::AllowedUpdate;

/// Handler description that is used by [`Dispatcher`].
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[derive(Debug, Clone)]
pub struct DpHandlerDescription {
    allowed: InterestSet<Kind>,
}

impl DpHandlerDescription {
    pub(crate) fn of(allowed: AllowedUpdate) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(Kind(allowed));
        Self { allowed: InterestSet::new_filter(set) }
    }

    pub(crate) fn allowed_updates(&self) -> Vec<AllowedUpdate> {
        self.allowed.observed.iter().map(|&Kind(x)| x).collect()
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Kind(AllowedUpdate);

impl EventKind for Kind {
    fn full_set() -> HashSet<Self> {
        use AllowedUpdate::*;

        // NB: We need to specify all update kinds by hand, because telegram doesn't
        //     enable `ChatMember` by default:
        //
        //     > A JSON-serialized list of the update types you want your bot to
        //     > receive. For example, specify [“message”, “edited_channel_post”,
        //     > “callback_query”] to only receive updates of these types. See Update
        //     > for a complete list of available update types. Specify an empty list
        //     > to receive all update types except chat_member (default). If not
        //     > specified, the previous setting will be used.

        [
            Message,
            EditedMessage,
            ChannelPost,
            EditedChannelPost,
            BusinessConnection,
            BusinessMessage,
            EditedBusinessMessage,
            DeletedBusinessMessages,
            MessageReaction,
            MessageReactionCount,
            InlineQuery,
            ChosenInlineResult,
            CallbackQuery,
            ShippingQuery,
            PreCheckoutQuery,
            Poll,
            PollAnswer,
            MyChatMember,
            ChatMember,
            ChatJoinRequest,
            ChatBoost,
            RemovedChatBoost,
        ]
        .into_iter()
        .map(Kind)
        .collect()
    }

    fn empty_set() -> HashSet<Self> {
        HashSet::new()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "macros")]
    use crate::{
        self as teloxide, // fixup for the `BotCommands` macro
        dispatching::{handler_description::Kind, HandlerExt, UpdateFilterExt},
        types::{AllowedUpdate::*, Update},
        utils::command::BotCommands,
    };
    #[cfg(feature = "macros")]
    use dptree::description::EventKind;

    #[cfg(feature = "macros")]
    #[derive(BotCommands, Clone)]
    #[command(rename_rule = "lowercase")]
    enum Cmd {
        B,
    }

    // <https://github.com/teloxide/teloxide/discussions/648>
    #[test]
    #[cfg(feature = "macros")]
    fn discussion_648() {
        let h =
            dptree::entry().branch(Update::filter_my_chat_member().endpoint(|| async {})).branch(
                Update::filter_message()
                    .branch(dptree::entry().filter_command::<Cmd>().endpoint(|| async {}))
                    .endpoint(|| async {}),
            );

        let mut v = h.description().allowed_updates();

        // Hash set randomizes element order, so to compare we need to sort
        v.sort_by_key(|&a| a as u8);

        assert_eq!(v, [Message, MyChatMember])
    }

    #[test]
    #[ignore = "this test requires `macros` feature"]
    #[cfg(not(feature = "macros"))]
    fn discussion_648() {
        panic!("this test requires `macros` feature")
    }

    // Test that all possible updates are specified in `Kind::full_set()`
    #[test]
    #[cfg(feature = "macros")]
    fn allowed_updates_full_set() {
        let full_set = Kind::full_set();
        let allowed_updates_reference = vec![
            Message,
            EditedMessage,
            ChannelPost,
            EditedChannelPost,
            MessageReaction,
            MessageReactionCount,
            InlineQuery,
            ChosenInlineResult,
            CallbackQuery,
            ShippingQuery,
            PreCheckoutQuery,
            Poll,
            PollAnswer,
            MyChatMember,
            ChatMember,
            ChatJoinRequest,
            ChatBoost,
            RemovedChatBoost,
        ];

        for update in allowed_updates_reference {
            match update {
                // CAUTION: Don't forget to add new `UpdateKind` to `allowed_updates_reference`!
                Message
                | EditedMessage
                | ChannelPost
                | EditedChannelPost
                | MessageReaction
                | MessageReactionCount
                | InlineQuery
                | ChosenInlineResult
                | CallbackQuery
                | ShippingQuery
                | PreCheckoutQuery
                | Poll
                | PollAnswer
                | MyChatMember
                | ChatMember
                | ChatJoinRequest
                | ChatBoost
                | RemovedChatBoost
                | BusinessMessage
                | BusinessConnection
                | EditedBusinessMessage
                | DeletedBusinessMessages => {
                    assert!(full_set.contains(&Kind(update)))
                }
            }
        }
    }
}
