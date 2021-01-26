use crate::{
    dispatching::{
        core::{Demux, DemuxBuilder, DispatchError, HandleResult, Handler},
        dispatcher_context::DispatcherContext,
        error_handlers::ErrorHandler,
        update_listeners::UpdateListener,
    },
    types::Update,
    Bot,
};
use futures::StreamExt;
use std::sync::Arc;
use crate::dispatching::core::FromContextOwn;

pub struct Dispatcher<Err, ErrHandler, Ctx=DispatcherContext<Update>> {
    bot: Bot,
    bot_name: Arc<str>,
    demux: Demux<Ctx, Err>,
    error_handler: ErrHandler,
}

impl<Err, ErrHandler> Dispatcher<Err, ErrHandler>
where
    Err: 'static,
    ErrHandler: ErrorHandler<DispatchError<Update, Err>>,
{
    pub async fn dispatch_one(&self, upd: Update) {
        self.dispatch_one_with_cx(self.make_cx(upd)).await;
    }

    pub async fn dispatch_with_listener<ListenerErr>(
        &self,
        listener: impl UpdateListener<ListenerErr>,
        listener_error_handler: &impl ErrorHandler<ListenerErr>,
    ) {
        self.dispatch_with_listener_and_cx_factory(
            listener,
            listener_error_handler,
            &|upd| self.make_cx(upd)
        ).await;
    }
}

impl<Err, ErrHandler, Ctx> Dispatcher<Err, ErrHandler, Ctx>
where
    Ctx: Send + 'static,
    Err: 'static,
    ErrHandler: ErrorHandler<DispatchError<Update, Err>>,
    Update: FromContextOwn<Ctx>,
{
    pub async fn dispatch_one_with_cx(&self, cx: Ctx) {
        match self
            .demux
            .handle(cx)
            .await
        {
            Ok(res) => match res {
                HandleResult::Ok => {}
                HandleResult::Err(e) => {
                    self.error_handler.handle_error(DispatchError::HandlerError(e)).await
                }
            },
            Err(e) => self.error_handler.handle_error(DispatchError::NoHandler(Update::from_context(e))).await,
        }
    }

    pub async fn dispatch_with_listener_and_cx_factory<ListenerErr>(
        &self,
        listener: impl UpdateListener<ListenerErr>,
        listener_error_handler: &impl ErrorHandler<ListenerErr>,
        cx_factory: &impl Fn(Update) -> Ctx,
    ) {
        listener
            .for_each_concurrent(None, |res| async move {
                match res {
                    Ok(upd) => self.dispatch_one_with_cx(cx_factory(upd)).await,
                    Err(e) => {
                        listener_error_handler.handle_error(e).await
                    }
                };
            })
            .await;
    }

    pub fn make_cx(&self, upd: Update) -> DispatcherContext<Update> {
        DispatcherContext::new(upd, self.bot.clone(), self.bot_name.clone())
    }
}

pub struct DispatcherBuilder<Err, Handler> {
    bot: Bot,
    bot_name: Arc<str>,
    demux: DemuxBuilder<DispatcherContext<Update>, Err>,
    error_handler: Handler,
}

impl<Err> DispatcherBuilder<Err, ()> {
    pub fn new(bot: Bot, bot_name: impl Into<Arc<str>>) -> Self {
        DispatcherBuilder {
            bot,
            bot_name: bot_name.into(),
            demux: DemuxBuilder::new(),
            error_handler: (),
        }
    }

    pub fn error_handler<H>(self, error_handler: H) -> DispatcherBuilder<Err, H>
    where
        H: ErrorHandler<DispatchError<Update, Err>>,
    {
        let DispatcherBuilder { bot, bot_name, demux, .. } = self;
        DispatcherBuilder { bot, bot_name, demux, error_handler }
    }
}

impl<Err, ErrHandler> DispatcherBuilder<Err, ErrHandler> {
    pub fn handle(
        mut self,
        handler: impl Handler<DispatcherContext<Update>, Err> + Send + Sync + 'static,
    ) -> Self {
        self.demux.add_service(handler);
        self
    }
}

impl<Err, ErrHandler> DispatcherBuilder<Err, ErrHandler>
where
    ErrHandler: ErrorHandler<DispatchError<Update, Err>>,
{
    pub fn build(self) -> Dispatcher<Err, ErrHandler> {
        let DispatcherBuilder { bot, bot_name, demux, error_handler } = self;
        Dispatcher { bot, bot_name, demux: demux.build(), error_handler }
    }
}
