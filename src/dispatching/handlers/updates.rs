pub(crate) use impls::{parser, UpdateRest};

use crate::{
    dispatching::{
        core::Context,
        handlers::{
            common::UpdateParser, inline_queries::InlineQueriesHandlerBuilder,
            messages::MessageHandlerBuilder,
        },
    },
    types,
    types::Update,
};
use crate::dispatching::handlers::chosen_inline_result::ChosenInlineResultsHandlerBuilder;
use crate::dispatching::handlers::callback_queries::CallbackQueriesHandlerBuilder;

pub fn any<Err>() -> UpdateParser<Update, Update, (), Err, parser::Update> {
    UpdateParser::new(parser::Update)
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

pub fn shipping_query<Err>(
) -> UpdateParser<Update, types::ShippingQuery, UpdateRest, Err, parser::ShippingQuery> {
    UpdateParser::new(parser::ShippingQuery)
}

pub fn pre_checkout_query<Err>(
) -> UpdateParser<Update, types::PreCheckoutQuery, UpdateRest, Err, parser::PreCheckoutQuery> {
    UpdateParser::new(parser::PreCheckoutQuery)
}

pub fn poll<Err>() -> UpdateParser<Update, types::Poll, UpdateRest, Err, parser::Poll> {
    UpdateParser::new(parser::Poll)
}

pub fn poll_answer<Err>(
) -> UpdateParser<Update, types::PollAnswer, UpdateRest, Err, parser::PollAnswer> {
    UpdateParser::new(parser::PollAnswer)
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
