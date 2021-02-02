//! Methods for the handling the incoming updates.
//!
//! All methods except `any` returns `UpdateKindHandlerBuilder` which allow you to build the handler for specified update kind.
//! using guards and other things. After you are ready for build the handler you must call `by` function
//! and pass to it handler for the incoming update.
//!
//! All types except `PreCheckoutQuery` and `ShippingQuery` has custom methods for guards.
//!
//! [`UpdateHandlerBuilder`]: crate::dispatching::handlers::UpdateHandlerBuilder
//! [`Update`]: crate::types::Update

pub(crate) use impls::{parser, UpdateRest};

use crate::{
    dispatching::{
        core::Context,
        handlers::{
            callback_queries::CallbackQueriesHandlerBuilder,
            chosen_inline_result::ChosenInlineResultsHandlerBuilder,
            common::{UpdateHandlerBuilder, UpdateKindHandlerBuilder},
            inline_queries::InlineQueriesHandlerBuilder,
            messages::MessageHandlerBuilder,
            poll_answers::PollAnswersHandlerBuilder,
            polls::PollsHandlerBuilder,
        },
    },
    types,
    types::Update,
};

/// Handle the `Update` struct.
pub fn any<Ctx: Context<Upd = Update>, Err>() -> UpdateHandlerBuilder<Ctx, Err> {
    UpdateHandlerBuilder::new()
}

pub fn message<Ctx: Context<Upd = types::Message>, Err>(
) -> MessageHandlerBuilder<Ctx, parser::Message, Err> {
    MessageHandlerBuilder::new(parser::Message)
}

pub fn edited_message<Ctx: Context<Upd = types::Message>, Err>(
) -> MessageHandlerBuilder<Ctx, parser::EditedMessage, Err> {
    MessageHandlerBuilder::new(parser::EditedMessage)
}

pub fn channel_post<Ctx: Context<Upd = types::Message>, Err>(
) -> MessageHandlerBuilder<Ctx, parser::ChannelPost, Err> {
    MessageHandlerBuilder::new(parser::ChannelPost)
}

pub fn edited_channel_post<Ctx: Context<Upd = types::Message>, Err>(
) -> MessageHandlerBuilder<Ctx, parser::EditedChannelPost, Err> {
    MessageHandlerBuilder::new(parser::EditedChannelPost)
}

pub fn inline_query<Ctx: Context<Upd = types::InlineQuery>, Err>(
) -> InlineQueriesHandlerBuilder<Ctx, parser::InlineQuery, Err> {
    InlineQueriesHandlerBuilder::new(parser::InlineQuery)
}

pub fn chosen_inline_result<Ctx: Context<Upd = types::ChosenInlineResult>, Err>(
) -> ChosenInlineResultsHandlerBuilder<Ctx, parser::ChosenInlineResult, Err> {
    ChosenInlineResultsHandlerBuilder::new(parser::ChosenInlineResult)
}

pub fn callback_query<Ctx: Context<Upd = types::CallbackQuery>, Err>(
) -> CallbackQueriesHandlerBuilder<Ctx, parser::CallbackQuery, Err> {
    CallbackQueriesHandlerBuilder::new(parser::CallbackQuery)
}

pub fn shipping_query<Ctx: Context<Upd = types::ShippingQuery>, Err>(
) -> UpdateKindHandlerBuilder<types::ShippingQuery, Ctx, parser::ShippingQuery, Err> {
    UpdateKindHandlerBuilder::new(parser::ShippingQuery)
}

pub fn pre_checkout_query<Ctx: Context<Upd = types::PreCheckoutQuery>, Err>(
) -> UpdateKindHandlerBuilder<types::PreCheckoutQuery, Ctx, parser::PreCheckoutQuery, Err> {
    UpdateKindHandlerBuilder::new(parser::PreCheckoutQuery)
}

pub fn poll<Ctx: Context<Upd = types::Poll>, Err>() -> PollsHandlerBuilder<Ctx, parser::Poll, Err> {
    PollsHandlerBuilder::new(parser::Poll)
}

pub fn poll_answer<Ctx: Context<Upd = types::PollAnswer>, Err>(
) -> PollAnswersHandlerBuilder<Ctx, parser::PollAnswer, Err> {
    PollAnswersHandlerBuilder::new(parser::PollAnswer)
}

mod impls {
    use crate::{
        dispatching::core::{Parser, ParserOut, RecombineFrom},
        types::{Update, UpdateKind},
    };

    pub(crate) mod parser {
        pub struct Update;
        pub struct Message;
        pub struct EditedMessage;
        pub struct ChannelPost;
        pub struct EditedChannelPost;
        pub struct InlineQuery;
        pub struct ChosenInlineResult;
        pub struct CallbackQuery;
        pub struct ShippingQuery;
        pub struct PreCheckoutQuery;
        pub struct Poll;
        pub struct PollAnswer;
    }

    pub struct UpdateRest(i32);

    macro_rules! impl_parser {
        ($(($ty:ident, $teloxide_ty:ident),)*) => {
            $(
                impl RecombineFrom<parser::$ty, crate::types::$teloxide_ty, UpdateRest> for Update {
                    fn recombine(data: ParserOut<crate::types::$teloxide_ty, UpdateRest>) -> Update {
                        let (kind, UpdateRest(id)) = data.into_inner();
                        Update {
                            id,
                            kind: UpdateKind::$ty(kind),
                        }
                    }
                }
                impl Parser<Update, crate::types::$teloxide_ty, UpdateRest> for parser::$ty {
                    fn parse(&self, update: Update) -> Result<ParserOut<crate::types::$teloxide_ty, UpdateRest>, Update> {
                        let Update { id, kind } = update;
                        let rest = UpdateRest(id);
                        match kind {
                            UpdateKind::$ty(message) => Ok(ParserOut::new(message, rest)),
                            _ => Err(<Update as RecombineFrom<UpdateKind, UpdateKind, UpdateRest>>::recombine(ParserOut::new(kind, rest))),
                        }
                    }
                }
            )*
        };
    }

    impl RecombineFrom<UpdateKind, UpdateKind, UpdateRest> for Update {
        fn recombine(data: ParserOut<UpdateKind, UpdateRest>) -> Update {
            let (kind, UpdateRest(id)) = data.into_inner();
            Update { id, kind }
        }
    }

    impl RecombineFrom<parser::Update, Update, ()> for Update {
        fn recombine(data: ParserOut<Update, ()>) -> Update {
            let (update, _) = data.into_inner();
            update
        }
    }

    impl Parser<Update, Update, ()> for parser::Update {
        fn parse(&self, update: Update) -> Result<ParserOut<Update, ()>, Update> {
            Ok(ParserOut::new(update, ()))
        }
    }

    impl_parser!(
        (Message, Message),
        (EditedMessage, Message),
        (ChannelPost, Message),
        (EditedChannelPost, Message),
        (InlineQuery, InlineQuery),
        (ChosenInlineResult, ChosenInlineResult),
        (CallbackQuery, CallbackQuery),
        (ShippingQuery, ShippingQuery),
        (PreCheckoutQuery, PreCheckoutQuery),
        (Poll, Poll),
        (PollAnswer, PollAnswer),
    );
}
