use crate::{
    dispatching::{
        core::{Demux, DemuxBuilder, DispatchError, HandleResult, Handler, Store},
        dispatcher_context::DispatcherContext,
        error_handlers::ErrorHandler,
        update_listeners::UpdateListener,
    },
    types::Update,
    Bot,
};
use futures::StreamExt;
use std::{future::Future, sync::Arc};
use crate::dispatching::error_handlers::{IgnoringErrorHandler};

pub struct Dispatcher<Err, Ctx = DispatcherContext<Update>> {
    bot: Bot,
    bot_name: Arc<str>,
    demux: Demux<Ctx, Err>,
    error_handler: Arc<dyn ErrorHandler<DispatchError<Ctx, Err>> + Send + Sync>,
    global_data: Arc<Store>,
}

impl<Err> Dispatcher<Err>
where
    Err: 'static,
{
    pub async fn dispatch_one(&self, upd: Update) {
        self.dispatch_one_with_cx(self.make_cx(upd)).await;
    }

    pub async fn dispatch_with_listener<ListenerErr>(
        &self,
        listener: impl UpdateListener<ListenerErr>,
        listener_error_handler: &impl ErrorHandler<ListenerErr>,
    ) {
        self.dispatch_with_listener_and_cx_factory(listener, listener_error_handler, &|upd| {
            futures::future::ready(self.make_cx(upd))
        })
        .await;
    }
}

impl<Err, Ctx> Dispatcher<Err, Ctx>
where
    Ctx: Send + 'static,
    Err: 'static,
{
    pub async fn dispatch_one_with_cx(&self, cx: Ctx) {
        match self.demux.handle(cx).await {
            Ok(res) => match res {
                HandleResult::Ok => {}
                HandleResult::Err(e) => {
                    self.error_handler.handle_error(DispatchError::HandlerError(e)).await
                }
            },
            Err(e) => self.error_handler.handle_error(DispatchError::NoHandler(e)).await,
        }
    }

    pub async fn dispatch_with_listener_and_cx_factory<ListenerErr, Fut>(
        &self,
        listener: impl UpdateListener<ListenerErr>,
        listener_error_handler: &impl ErrorHandler<ListenerErr>,
        cx_factory: &impl Fn(Update) -> Fut,
    ) where
        Fut: Future<Output = Ctx>,
    {
        listener
            .for_each_concurrent(None, |res| async move {
                match res {
                    Ok(upd) => self.dispatch_one_with_cx(cx_factory(upd).await).await,
                    Err(e) => listener_error_handler.handle_error(e).await,
                };
            })
            .await;
    }

    pub fn make_cx(&self, upd: Update) -> DispatcherContext<Update> {
        DispatcherContext::new(
            upd,
            self.bot.clone(),
            self.bot_name.clone(),
            self.global_data.clone(),
        )
    }
}

pub struct DispatcherBuilder<Err, Ctx = DispatcherContext<Update>> {
    bot: Bot,
    bot_name: Arc<str>,
    demux: DemuxBuilder<Ctx, Err>,
    error_handler: Option<Arc<dyn ErrorHandler<DispatchError<Ctx, Err>> + Send + Sync>>,
    global_data: Store,
}

impl<Err, Ctx> DispatcherBuilder<Err, Ctx>
where
    Err: Send,
    Ctx: Send,
{
    pub fn new(bot: Bot, bot_name: impl Into<Arc<str>>) -> Self {
        DispatcherBuilder {
            bot,
            bot_name: bot_name.into(),
            demux: DemuxBuilder::new(),
            error_handler: None,
            global_data: Store::new(),
        }
    }

    pub fn error_handler<H>(self, error_handler: H) -> DispatcherBuilder<Err, Ctx>
    where
        H: ErrorHandler<DispatchError<Ctx, Err>> + Send + Sync + 'static,
    {
        let error_handler = Arc::new(error_handler) as _;
        let DispatcherBuilder { bot, bot_name, demux, global_data, .. } = self;
        DispatcherBuilder { bot, bot_name, demux, error_handler: Some(error_handler), global_data }
    }
}

impl<Err, Ctx> DispatcherBuilder<Err, Ctx> {
    pub fn data<T: Send + Sync + 'static>(mut self, data: T) -> Self {
        self.global_data.insert(data);
        self
    }

    pub fn handle(mut self, handler: impl Handler<Ctx, Err> + Send + Sync + 'static) -> Self {
        self._add_handler(handler);
        self
    }

    pub fn _add_handler(&mut self, handler: impl Handler<Ctx, Err> + Send + Sync + 'static) {
        self.demux.add_service(handler);
    }
}

impl<Err, Ctx> DispatcherBuilder<Err, Ctx> {
    pub fn build(self) -> Dispatcher<Err, Ctx> {
        let DispatcherBuilder { bot, bot_name, demux, error_handler, global_data } = self;
        Dispatcher {
            bot,
            bot_name,
            demux: demux.build(),
            error_handler: error_handler.unwrap_or_else(|| IgnoringErrorHandler::new() as _),
            global_data: Arc::new(global_data),
        }
    }
}
