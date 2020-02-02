use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, AsyncHandler,
        HandlerCtx, LoggingHandler,
    },
    types::{
        CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll,
        PreCheckoutQuery, ShippingQuery, UpdateKind,
    },
    Bot,
};
use futures::StreamExt;
use std::{fmt::Debug, sync::Arc};

type H<'a, Upd, HandlerE> =
    Option<Box<dyn AsyncHandler<HandlerCtx<Upd>, Result<(), HandlerE>> + 'a>>;

/// One dispatcher to rule them all.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
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
            handlers_error_handler: Box::new(LoggingHandler::new(
                "An error from a Dispatcher's handler",
            )),
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

    /// Registers a handler of errors, produced by other handlers.
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

    /// Starts your bot with the default parameters.
    ///
    /// The default parameters are a long polling update listener and log all
    /// errors produced by this listener).
    pub async fn dispatch(&'a self) {
        self.dispatch_with_listener(
            update_listeners::polling_default(Arc::clone(&self.bot)),
            &LoggingHandler::new("An error from the update listener"),
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
                        self.handle(&self.message_handler, message).await
                    }
                    UpdateKind::EditedMessage(message) => {
                        self.handle(&self.edited_message_handler, message).await
                    }
                    UpdateKind::ChannelPost(post) => {
                        self.handle(&self.channel_post_handler, post).await
                    }
                    UpdateKind::EditedChannelPost(post) => {
                        self.handle(&self.edited_channel_post_handler, post)
                            .await
                    }
                    UpdateKind::InlineQuery(query) => {
                        self.handle(&self.inline_query_handler, query).await
                    }
                    UpdateKind::ChosenInlineResult(result) => {
                        self.handle(&self.chosen_inline_result_handler, result)
                            .await
                    }
                    UpdateKind::CallbackQuery(query) => {
                        self.handle(&self.callback_query_handler, query).await
                    }
                    UpdateKind::ShippingQuery(query) => {
                        self.handle(&self.shipping_query_handler, query).await
                    }
                    UpdateKind::PreCheckoutQuery(query) => {
                        self.handle(&self.pre_checkout_query_handler, query)
                            .await
                    }
                    UpdateKind::Poll(poll) => {
                        self.handle(&self.poll_handler, poll).await
                    }
                }
            })
            .await
    }

    // Handles a single update.
    async fn handle<Upd>(&self, handler: &H<'a, Upd, HandlerE>, update: Upd) {
        if let Some(handler) = &handler {
            if let Err(error) = handler
                .handle(HandlerCtx {
                    bot: Arc::clone(&self.bot),
                    update,
                })
                .await
            {
                self.handlers_error_handler.handle(error).await;
            }
        }
    }
}
