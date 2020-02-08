use crate::{
    dispatching::{
        error_handlers::ErrorHandler, update_listeners,
        update_listeners::UpdateListener, CtxHandler, DispatcherHandlerCtx,
        DispatcherHandlerResult, LoggingErrorHandler,
    },
    types::{
        CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll,
        PollAnswer, PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
    },
    Bot,
};
use futures::{stream, StreamExt};
use std::{fmt::Debug, future::Future, sync::Arc};

type Handlers<'a, Upd, HandlerE> = Vec<
    Box<
        dyn CtxHandler<
                DispatcherHandlerCtx<Upd>,
                DispatcherHandlerResult<Upd, HandlerE>,
            > + 'a,
    >,
>;

/// One dispatcher to rule them all.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
pub struct Dispatcher<'a, HandlerE> {
    bot: Arc<Bot>,

    handlers_error_handler: Box<dyn ErrorHandler<HandlerE> + 'a>,

    update_handlers: Handlers<'a, Update, HandlerE>,
    message_handlers: Handlers<'a, Message, HandlerE>,
    edited_message_handlers: Handlers<'a, Message, HandlerE>,
    channel_post_handlers: Handlers<'a, Message, HandlerE>,
    edited_channel_post_handlers: Handlers<'a, Message, HandlerE>,
    inline_query_handlers: Handlers<'a, InlineQuery, HandlerE>,
    chosen_inline_result_handlers: Handlers<'a, ChosenInlineResult, HandlerE>,
    callback_query_handlers: Handlers<'a, CallbackQuery, HandlerE>,
    shipping_query_handlers: Handlers<'a, ShippingQuery, HandlerE>,
    pre_checkout_query_handlers: Handlers<'a, PreCheckoutQuery, HandlerE>,
    poll_handlers: Handlers<'a, Poll, HandlerE>,
    poll_answer_handlers: Handlers<'a, PollAnswer, HandlerE>,
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
            handlers_error_handler: Box::new(LoggingErrorHandler::new(
                "An error from a Dispatcher's handler",
            )),
            update_handlers: Vec::new(),
            message_handlers: Vec::new(),
            edited_message_handlers: Vec::new(),
            channel_post_handlers: Vec::new(),
            edited_channel_post_handlers: Vec::new(),
            inline_query_handlers: Vec::new(),
            chosen_inline_result_handlers: Vec::new(),
            callback_query_handlers: Vec::new(),
            shipping_query_handlers: Vec::new(),
            pre_checkout_query_handlers: Vec::new(),
            poll_handlers: Vec::new(),
            poll_answer_handlers: Vec::new(),
        }
    }

    /// Registers a handler of errors, produced by other handlers.
    #[must_use]
    pub fn handlers_error_handler<T>(mut self, val: T) -> Self
    where
        T: ErrorHandler<HandlerE> + 'a,
    {
        self.handlers_error_handler = Box::new(val);
        self
    }

    #[must_use]
    pub fn message_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<Message>, I> + 'a,
        I: Into<DispatcherHandlerResult<Message, HandlerE>> + 'a,
    {
        self.message_handlers = register_handler(self.message_handlers, h);
        self
    }

    #[must_use]
    pub fn edited_message_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<Message>, I> + 'a,
        I: Into<DispatcherHandlerResult<Message, HandlerE>> + 'a,
    {
        self.edited_message_handlers =
            register_handler(self.edited_message_handlers, h);
        self
    }

    #[must_use]
    pub fn channel_post_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<Message>, I> + 'a,
        I: Into<DispatcherHandlerResult<Message, HandlerE>> + 'a,
    {
        self.channel_post_handlers =
            register_handler(self.channel_post_handlers, h);
        self
    }

    #[must_use]
    pub fn edited_channel_post_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<Message>, I> + 'a,
        I: Into<DispatcherHandlerResult<Message, HandlerE>> + 'a,
    {
        self.edited_channel_post_handlers =
            register_handler(self.edited_channel_post_handlers, h);
        self
    }

    #[must_use]
    pub fn inline_query_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<InlineQuery>, I> + 'a,
        I: Into<DispatcherHandlerResult<InlineQuery, HandlerE>> + 'a,
    {
        self.inline_query_handlers =
            register_handler(self.inline_query_handlers, h);
        self
    }

    #[must_use]
    pub fn chosen_inline_result_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<ChosenInlineResult>, I> + 'a,
        I: Into<DispatcherHandlerResult<ChosenInlineResult, HandlerE>> + 'a,
    {
        self.chosen_inline_result_handlers =
            register_handler(self.chosen_inline_result_handlers, h);
        self
    }

    #[must_use]
    pub fn callback_query_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<CallbackQuery>, I> + 'a,
        I: Into<DispatcherHandlerResult<CallbackQuery, HandlerE>> + 'a,
    {
        self.callback_query_handlers =
            register_handler(self.callback_query_handlers, h);
        self
    }

    #[must_use]
    pub fn shipping_query_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<ShippingQuery>, I> + 'a,
        I: Into<DispatcherHandlerResult<ShippingQuery, HandlerE>> + 'a,
    {
        self.shipping_query_handlers =
            register_handler(self.shipping_query_handlers, h);
        self
    }

    #[must_use]
    pub fn pre_checkout_query_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<PreCheckoutQuery>, I> + 'a,
        I: Into<DispatcherHandlerResult<PreCheckoutQuery, HandlerE>> + 'a,
    {
        self.pre_checkout_query_handlers =
            register_handler(self.pre_checkout_query_handlers, h);
        self
    }

    #[must_use]
    pub fn poll_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<Poll>, I> + 'a,
        I: Into<DispatcherHandlerResult<Poll, HandlerE>> + 'a,
    {
        self.poll_handlers = register_handler(self.poll_handlers, h);
        self
    }

    #[must_use]
    pub fn poll_answer_handler<H, I>(mut self, h: H) -> Self
    where
        H: CtxHandler<DispatcherHandlerCtx<PollAnswer>, I> + 'a,
        I: Into<DispatcherHandlerResult<PollAnswer, HandlerE>> + 'a,
    {
        self.poll_answer_handlers =
            register_handler(self.poll_answer_handlers, h);
        self
    }

    /// Starts your bot with the default parameters.
    ///
    /// The default parameters are a long polling update listener and log all
    /// errors produced by this listener).
    pub async fn dispatch(&'a self) {
        self.dispatch_with_listener(
            update_listeners::polling_default(Arc::clone(&self.bot)),
            &LoggingErrorHandler::new("An error from the update listener"),
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
        Eh: ErrorHandler<ListenerE> + 'a,
        ListenerE: Debug,
    {
        let update_listener = Box::pin(update_listener);

        update_listener
            .for_each_concurrent(None, move |update| async move {
                let update = match update {
                    Ok(update) => update,
                    Err(error) => {
                        update_listener_error_handler.handle_error(error).await;
                        return;
                    }
                };

                let update =
                    match self.handle(&self.update_handlers, update).await {
                        Some(update) => update,
                        None => return,
                    };

                match update.kind {
                    UpdateKind::Message(message) => {
                        self.handle(&self.message_handlers, message).await;
                    }
                    UpdateKind::EditedMessage(message) => {
                        self.handle(&self.edited_message_handlers, message)
                            .await;
                    }
                    UpdateKind::ChannelPost(post) => {
                        self.handle(&self.channel_post_handlers, post).await;
                    }
                    UpdateKind::EditedChannelPost(post) => {
                        self.handle(&self.edited_channel_post_handlers, post)
                            .await;
                    }
                    UpdateKind::InlineQuery(query) => {
                        self.handle(&self.inline_query_handlers, query).await;
                    }
                    UpdateKind::ChosenInlineResult(result) => {
                        self.handle(
                            &self.chosen_inline_result_handlers,
                            result,
                        )
                        .await;
                    }
                    UpdateKind::CallbackQuery(query) => {
                        self.handle(&self.callback_query_handlers, query).await;
                    }
                    UpdateKind::ShippingQuery(query) => {
                        self.handle(&self.shipping_query_handlers, query).await;
                    }
                    UpdateKind::PreCheckoutQuery(query) => {
                        self.handle(&self.pre_checkout_query_handlers, query)
                            .await;
                    }
                    UpdateKind::Poll(poll) => {
                        self.handle(&self.poll_handlers, poll).await;
                    }
                    UpdateKind::PollAnswer(answer) => {
                        self.handle(&self.poll_answer_handlers, answer).await;
                    }
                }
            })
            .await
    }

    // Handles a single update.
    #[allow(clippy::ptr_arg)]
    async fn handle<Upd>(
        &self,
        handlers: &Handlers<'a, Upd, HandlerE>,
        update: Upd,
    ) -> Option<Upd> {
        stream::iter(handlers)
            .fold(Some(update), |acc, handler| async move {
                // Option::and_then is not working here, because
                // Middleware::handle is asynchronous.
                match acc {
                    Some(update) => {
                        let DispatcherHandlerResult { next, result } = handler
                            .handle_ctx(DispatcherHandlerCtx {
                                bot: Arc::clone(&self.bot),
                                update,
                            })
                            .await;

                        if let Err(error) = result {
                            self.handlers_error_handler
                                .handle_error(error)
                                .await
                        }

                        next
                    }
                    None => None,
                }
            })
            .await
    }
}

// Transforms Future<Output = T> into Future<Output = U> by applying an Into
// conversion.
async fn intermediate_fut0<T, U>(fut: impl Future<Output = T>) -> U
where
    T: Into<U>,
{
    fut.await.into()
}

fn intermediate_fut1<'a, Upd, HandlerE, H, I>(
    h: H,
) -> impl CtxHandler<DispatcherHandlerCtx<Upd>, DispatcherHandlerResult<Upd, HandlerE>>
where
    H: CtxHandler<DispatcherHandlerCtx<Upd>, I> + 'a,
    I: Into<DispatcherHandlerResult<Upd, HandlerE>> + 'a,
{
    move |ctx| intermediate_fut0(h.handle_ctx(ctx))
}

/// Registers a single handler.
fn register_handler<'a, Upd, H, I, HandlerE>(
    mut handlers: Handlers<'a, Upd, HandlerE>,
    h: H,
) -> Handlers<'a, Upd, HandlerE>
where
    H: CtxHandler<DispatcherHandlerCtx<Upd>, I> + 'a,
    I: Into<DispatcherHandlerResult<Upd, HandlerE>> + 'a,
{
    //  handlers.push(Box::new());
    handlers
}
