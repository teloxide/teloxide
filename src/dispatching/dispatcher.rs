use crate::{
    dispatching::{
        error_handlers,
        session::{SessionDispatcher, SessionHandlerCtx, SessionState},
        update_listeners,
        update_listeners::UpdateListener,
        AsyncHandler,
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
    pub bot: &'a Bot,
    pub update: Upd,
}

/// The main dispatcher that joins all the parts together.
pub struct Dispatcher<'a, Session1, Session2, H1, H2, HandlerE> {
    bot: &'a Bot,

    handlers_error_handler: Box<dyn AsyncHandler<HandlerE, ()> + 'a>,

    private_message_dp: Option<SessionDispatcher<'a, Session1, H1>>,
    private_edited_message_dp: Option<SessionDispatcher<'a, Session2, H2>>,

    message_handler:
        Option<Box<dyn AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a>>,
    edited_message_handler:
        Option<Box<dyn AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a>>,
    channel_post_handler:
        Option<Box<dyn AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a>>,
    edited_channel_post_handler:
        Option<Box<dyn AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a>>,
    inline_query_handler: Option<
        Box<dyn AsyncHandler<BasicHandlerCtx<'a, InlineQuery>, ()> + 'a>,
    >,
    chosen_inline_result_handler: Option<
        Box<dyn AsyncHandler<BasicHandlerCtx<'a, ChosenInlineResult>, ()> + 'a>,
    >,
    callback_query_handler: Option<
        Box<dyn AsyncHandler<BasicHandlerCtx<'a, CallbackQuery>, ()> + 'a>,
    >,
    shipping_query_handler: Option<
        Box<dyn AsyncHandler<BasicHandlerCtx<'a, ShippingQuery>, ()> + 'a>,
    >,
    pre_checkout_query_handler: Option<
        Box<dyn AsyncHandler<BasicHandlerCtx<'a, PreCheckoutQuery>, ()> + 'a>,
    >,
    poll_handler:
        Option<Box<dyn AsyncHandler<BasicHandlerCtx<'a, Poll>, ()> + 'a>>,
}

impl<'a, Session1, Session2, H1, H2, HandlerE>
    Dispatcher<'a, Session1, Session2, H1, H2, HandlerE>
where
    Session1: Default + 'a,
    Session2: Default + 'a,
    H1: AsyncHandler<
            SessionHandlerCtx<'a, Message, Session1>,
            SessionState<Session1>,
        > + 'a,
    H2: AsyncHandler<
            SessionHandlerCtx<'a, Message, Session2>,
            SessionState<Session2>,
        > + 'a,
    HandlerE: Debug + 'a,
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

    pub fn handlers_error_handler<T>(mut self, val: T) -> Self
    where
        T: AsyncHandler<HandlerE, ()> + 'a,
    {
        self.handlers_error_handler = Box::new(val);
        self
    }

    pub fn private_message_dp(
        mut self,
        dp: SessionDispatcher<'a, Session1, H1>,
    ) -> Self {
        self.private_message_dp = Some(dp);
        self
    }

    pub fn private_edited_message_dp(
        mut self,
        dp: SessionDispatcher<'a, Session2, H2>,
    ) -> Self {
        self.private_edited_message_dp = Some(dp);
        self
    }

    pub fn message_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a,
    {
        self.message_handler = Some(Box::new(h));
        self
    }

    pub fn edited_message_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a,
    {
        self.edited_message_handler = Some(Box::new(h));
        self
    }

    pub fn channel_post_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a,
    {
        self.channel_post_handler = Some(Box::new(h));
        self
    }

    pub fn edited_channel_post_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, Message>, ()> + 'a,
    {
        self.edited_channel_post_handler = Some(Box::new(h));
        self
    }

    pub fn inline_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, InlineQuery>, ()> + 'a,
    {
        self.inline_query_handler = Some(Box::new(h));
        self
    }

    pub fn chosen_inline_result_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, ChosenInlineResult>, ()> + 'a,
    {
        self.chosen_inline_result_handler = Some(Box::new(h));
        self
    }

    pub fn callback_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, CallbackQuery>, ()> + 'a,
    {
        self.callback_query_handler = Some(Box::new(h));
        self
    }

    pub fn shipping_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, ShippingQuery>, ()> + 'a,
    {
        self.shipping_query_handler = Some(Box::new(h));
        self
    }

    pub fn pre_checkout_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, PreCheckoutQuery>, ()> + 'a,
    {
        self.pre_checkout_query_handler = Some(Box::new(h));
        self
    }

    pub fn poll_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<BasicHandlerCtx<'a, Poll>, ()> + 'a,
    {
        self.poll_handler = Some(Box::new(h));
        self
    }

    pub async fn dispatch(&'a mut self) {
        self.dispatch_with_listener(
            update_listeners::polling_default(self.bot),
            &error_handlers::Log,
        )
        .await;
    }

    pub async fn dispatch_with_listener<UListener, ListenerE, Eh>(
        &'a self,
        update_listener: UListener,
        update_listener_error_handler: &'a Eh,
    ) where
        UListener: UpdateListener<ListenerE> + 'a,
        Eh: AsyncHandler<ListenerE, ()> + 'a,
        ListenerE: Debug,
    {
        let update_listener = Box::pin(update_listener);

        update_listener
            .for_each_concurrent(None, move |update| async move {
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
                                &self.private_message_dp
                            {
                                private_message_dp
                                    .dispatch(self.bot, message)
                                    .await;
                            }
                        }
                        _ => {
                            if let Some(message_handler) = &self.message_handler
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
                                    &self.private_edited_message_dp
                                {
                                    private_edited_message_dp
                                        .dispatch(self.bot, message)
                                        .await;
                                }
                            }
                            _ => {
                                if let Some(edited_message_handler) =
                                    &self.edited_message_handler
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
                            &self.channel_post_handler
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
                            &self.edited_channel_post_handler
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
                            &self.inline_query_handler
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
                            &self.chosen_inline_result_handler
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
                            &self.callback_query_handler
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
                            &self.shipping_query_handler
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
                            &self.pre_checkout_query_handler
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
                        if let Some(poll_handler) = &self.poll_handler {
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
