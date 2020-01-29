use crate::{
    dispatching::{
        error_handlers,
        session::{SessionDispatcher, SessionHandlerCtx, SessionState},
        update_listeners,
        update_listeners::UpdateListener,
        Handler,
    },
    types::{
        CallbackQuery, ChatKind, ChosenInlineResult, InlineQuery, Message,
        Poll, PreCheckoutQuery, ShippingQuery, UpdateKind,
    },
    Bot,
};
use futures::StreamExt;
use std::fmt::Debug;

pub struct BasicHandlerCtx<'a, Upd> {
    bot: &'a Bot,
    update: Upd,
}

pub struct Dispatcher<'a, Session1, Session2, H1, H2, HandlerE> {
    bot: &'a Bot,

    handlers_error_handler: Box<dyn Handler<HandlerE, ()>>,

    private_message_dp: Option<SessionDispatcher<'a, Session1, H1>>,
    private_edited_message_dp: Option<SessionDispatcher<'a, Session2, H2>>,

    message_handler: Option<Box<dyn Handler<BasicHandlerCtx<'a, Message>, ()>>>,
    edited_message_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, Message>, ()>>>,
    channel_post_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, Message>, ()>>>,
    edited_channel_post_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, Message>, ()>>>,
    inline_query_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, InlineQuery>, ()>>>,
    chosen_inline_result_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, ChosenInlineResult>, ()>>>,
    callback_query_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, CallbackQuery>, ()>>>,
    shipping_query_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, ShippingQuery>, ()>>>,
    pre_checkout_query_handler:
        Option<Box<dyn Handler<BasicHandlerCtx<'a, PreCheckoutQuery>, ()>>>,
    poll_handler: Option<Box<dyn Handler<BasicHandlerCtx<'a, Poll>, ()>>>,
}

impl<'a, Session1, Session2, H1, H2, HandlerE>
    Dispatcher<'a, Session1, Session2, H1, H2, HandlerE>
where
    Session1: Default,
    Session2: Default,
    H1: Handler<
        SessionHandlerCtx<'a, Message, Session1>,
        SessionState<Session1>,
    >,
    H2: Handler<
        SessionHandlerCtx<'a, Message, Session2>,
        SessionState<Session2>,
    >,
    HandlerE: Debug,
{
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            handlers_error_handler: Box::new(error_handlers::Log),
            private_message_dp: None,
            private_edited_message_dp: None,
            message_handler: None,
            edited_message_handler: None,
            channel_post_handler: None,
            edited_channel_post_handler: None,
            inline_query_handler: None,
            chosen_inline_result_handler: None,
            callback_query_handler: None,
            shipping_query_handler: None,
            pre_checkout_query_handler: None,
            poll_handler: None,
        }
    }

    pub fn private_message_dp(
        mut self,
        dp: SessionDispatcher<'a, Session1, H1>,
    ) -> Self {
        self.private_message_dp = Some(dp);
        self
    }

    async fn dispatch(&'a mut self)
    where
        Session1: 'a,
        Session2: 'a,
        H1: 'a,
        H2: 'a,
        HandlerE: 'a,
    {
        self.dispatch_with_listener(
            update_listeners::polling_default(self.bot),
            error_handlers::Log,
        )
        .await;
    }

    async fn dispatch_with_listener<UListener, ListenerE, Eh>(
        &'a mut self,
        update_listener: UListener,
        update_listener_error_handler: Eh,
    ) where
        UListener: UpdateListener<ListenerE> + 'a,
        Eh: Handler<ListenerE, ()> + 'a,
        Session1: 'a,
        Session2: 'a,
        H1: 'a,
        H2: 'a,
        HandlerE: 'a,
        ListenerE: Debug,
    {
        update_listener
            .for_each_concurrent(None, move |update| async {
                let update = match update {
                    Ok(update) => update,
                    Err(error) => {
                        update_listener_error_handler.handle(error).await;
                        return;
                    }
                };

                match update.kind {
                    UpdateKind::Message(message) => match message.chat.kind {
                        ChatKind::Private { .. } => {
                            if let Some(private_message_dp) =
                                &mut self.private_message_dp
                            {
                                private_message_dp
                                    .dispatch(self.bot, message)
                                    .await;
                            }
                        }
                        _ => {
                            if let Some(message_handler) =
                                &mut self.message_handler
                            {
                                message_handler
                                    .handle(BasicHandlerCtx {
                                        bot: self.bot,
                                        update: message,
                                    })
                                    .await
                            }
                        }
                    },

                    UpdateKind::EditedMessage(message) => {
                        match message.chat.kind {
                            ChatKind::Private { .. } => {
                                if let Some(private_edited_message_dp) =
                                    &mut self.private_edited_message_dp
                                {
                                    private_edited_message_dp
                                        .dispatch(self.bot, message)
                                        .await;
                                }
                            }
                            _ => {
                                if let Some(edited_message_handler) =
                                    &mut self.edited_message_handler
                                {
                                    edited_message_handler
                                        .handle(BasicHandlerCtx {
                                            bot: self.bot,
                                            update: message,
                                        })
                                        .await
                                }
                            }
                        }
                    }
                    UpdateKind::ChannelPost(post) => {
                        if let Some(channel_post_handler) =
                            &mut self.channel_post_handler
                        {
                            channel_post_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: post,
                                })
                                .await;
                        }
                    }
                    UpdateKind::EditedChannelPost(post) => {
                        if let Some(edited_channel_post_handler) =
                            &mut self.edited_channel_post_handler
                        {
                            edited_channel_post_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: post,
                                })
                                .await;
                        }
                    }
                    UpdateKind::InlineQuery(query) => {
                        if let Some(inline_query_handler) =
                            &mut self.inline_query_handler
                        {
                            inline_query_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: query,
                                })
                                .await;
                        }
                    }
                    UpdateKind::ChosenInlineResult(result) => {
                        if let Some(chosen_inline_result_handler) =
                            &mut self.chosen_inline_result_handler
                        {
                            chosen_inline_result_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: result,
                                })
                                .await;
                        }
                    }
                    UpdateKind::CallbackQuery(query) => {
                        if let Some(callback_query_handler) =
                            &mut self.callback_query_handler
                        {
                            callback_query_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: query,
                                })
                                .await;
                        }
                    }
                    UpdateKind::ShippingQuery(query) => {
                        if let Some(shipping_query_handler) =
                            &mut self.shipping_query_handler
                        {
                            shipping_query_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: query,
                                })
                                .await;
                        }
                    }
                    UpdateKind::PreCheckoutQuery(query) => {
                        if let Some(pre_checkout_query_handler) =
                            &mut self.pre_checkout_query_handler
                        {
                            pre_checkout_query_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: query,
                                })
                                .await;
                        }
                    }
                    UpdateKind::Poll(poll) => {
                        if let Some(poll_handler) = &mut self.poll_handler {
                            poll_handler
                                .handle(BasicHandlerCtx {
                                    bot: self.bot,
                                    update: poll,
                                })
                                .await;
                        }
                    }
                }
            })
            .await
    }
}
