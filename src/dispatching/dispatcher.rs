use crate::{
    dispatching::{
        error_handlers, update_listeners, update_listeners::UpdateListener,
        AsyncHandler,
    },
    requests::{Request, ResponseResult},
    types::{
        CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll,
        PreCheckoutQuery, ShippingQuery, UpdateKind,
    },
    Bot,
};
use futures::StreamExt;
use std::{fmt::Debug, sync::Arc};

/// A dispatcher's handler's context of a bot and an update.
pub struct HandlerCtx<Upd> {
    pub bot: Arc<Bot>,
    pub update: Upd,
}

impl HandlerCtx<Message> {
    pub fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }

    pub async fn reply<T>(self, text: T) -> ResponseResult<()>
    where
        T: Into<String>,
    {
        self.bot
            .send_message(self.chat_id(), text)
            .send()
            .await
            .map(|_| ())
    }
}

type H<'a, Upd, HandlerE> =
    Option<Box<dyn AsyncHandler<HandlerCtx<Upd>, Result<(), HandlerE>> + 'a>>;

/// The main dispatcher to rule them all.
pub struct Dispatcher<'a, HandlerE> {
    bot: Arc<Bot>,

    handlers_error_handler: Box<dyn AsyncHandler<HandlerE, ()> + 'a>,

    message_handler: H<'a, Message, HandlerE>,
    edited_message_handler: H<'a, Message, HandlerE>,
    channel_post_handler: H<'a, Message, HandlerE>,
    edited_channel_post_handler: H<'a, Message, HandlerE>,
    inline_query_handler: H<'a, InlineQuery, HandlerE>,
    chosen_inline_result_handler: H<'a, ChosenInlineResult, HandlerE>,
    callback_query_handler: H<'a, CallbackQuery, HandlerE>,
    shipping_query_handler: H<'a, ShippingQuery, HandlerE>,
    pre_checkout_query_handler: H<'a, PreCheckoutQuery, HandlerE>,
    poll_handler: H<'a, Poll, HandlerE>,
}

impl<'a, HandlerE> Dispatcher<'a, HandlerE>
where
    HandlerE: Debug + 'a,
{
    /// Constructs a new dispatcher with this `bot`.
    #[must_use]
    pub fn new(bot: Bot) -> Self {
        Self {
            bot: Arc::new(bot),
            handlers_error_handler: Box::new(error_handlers::Log),
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

    #[must_use]
    pub fn handlers_error_handler<T>(mut self, val: T) -> Self
    where
        T: AsyncHandler<HandlerE, ()> + 'a,
    {
        self.handlers_error_handler = Box::new(val);
        self
    }

    #[must_use]
    pub fn message_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<Message>, Result<(), HandlerE>> + 'a,
    {
        self.message_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn edited_message_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<Message>, Result<(), HandlerE>> + 'a,
    {
        self.edited_message_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn channel_post_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<Message>, Result<(), HandlerE>> + 'a,
    {
        self.channel_post_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn edited_channel_post_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<Message>, Result<(), HandlerE>> + 'a,
    {
        self.edited_channel_post_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn inline_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<InlineQuery>, Result<(), HandlerE>> + 'a,
    {
        self.inline_query_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn chosen_inline_result_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<ChosenInlineResult>, Result<(), HandlerE>>
            + 'a,
    {
        self.chosen_inline_result_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn callback_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<CallbackQuery>, Result<(), HandlerE>> + 'a,
    {
        self.callback_query_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn shipping_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<ShippingQuery>, Result<(), HandlerE>> + 'a,
    {
        self.shipping_query_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn pre_checkout_query_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<PreCheckoutQuery>, Result<(), HandlerE>>
            + 'a,
    {
        self.pre_checkout_query_handler = Some(Box::new(h));
        self
    }

    #[must_use]
    pub fn poll_handler<H>(mut self, h: H) -> Self
    where
        H: AsyncHandler<HandlerCtx<Poll>, Result<(), HandlerE>> + 'a,
    {
        self.poll_handler = Some(Box::new(h));
        self
    }

    /// Starts your bot.
    pub async fn dispatch(&'a self) {
        self.dispatch_with_listener(
            update_listeners::polling_default(Arc::clone(&self.bot)),
            &error_handlers::Log,
        )
        .await;
    }

    /// Starts your bot with custom `update_listener` and
    /// `update_listener_error_handler`.
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
                    UpdateKind::Message(message) => {
                        if let Some(message_handler) = &self.message_handler {
                            if let Err(error) = message_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: message,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::EditedMessage(message) => {
                        if let Some(edited_message_handler) =
                            &self.edited_message_handler
                        {
                            if let Err(error) = edited_message_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: message,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::ChannelPost(post) => {
                        if let Some(channel_post_handler) =
                            &self.channel_post_handler
                        {
                            if let Err(error) = channel_post_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: post,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::EditedChannelPost(post) => {
                        if let Some(edited_channel_post_handler) =
                            &self.edited_channel_post_handler
                        {
                            if let Err(error) = edited_channel_post_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: post,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::InlineQuery(query) => {
                        if let Some(inline_query_handler) =
                            &self.inline_query_handler
                        {
                            if let Err(error) = inline_query_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: query,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::ChosenInlineResult(result) => {
                        if let Some(chosen_inline_result_handler) =
                            &self.chosen_inline_result_handler
                        {
                            if let Err(error) = chosen_inline_result_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: result,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::CallbackQuery(query) => {
                        if let Some(callback_query_handler) =
                            &self.callback_query_handler
                        {
                            if let Err(error) = callback_query_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: query,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::ShippingQuery(query) => {
                        if let Some(shipping_query_handler) =
                            &self.shipping_query_handler
                        {
                            if let Err(error) = shipping_query_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: query,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::PreCheckoutQuery(query) => {
                        if let Some(pre_checkout_query_handler) =
                            &self.pre_checkout_query_handler
                        {
                            if let Err(error) = pre_checkout_query_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: query,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                    UpdateKind::Poll(poll) => {
                        if let Some(poll_handler) = &self.poll_handler {
                            if let Err(error) = poll_handler
                                .handle(HandlerCtx {
                                    bot: Arc::clone(&self.bot),
                                    update: poll,
                                })
                                .await
                            {
                                self.handlers_error_handler.handle(error).await;
                            }
                        }
                    }
                }
            })
            .await
    }
}
