use crate::{
    dispatching::{
        core::{Demux, DemuxBuilder, DispatchError, HandleResult, Handler},
        dispatcher_context::DispatcherContext,
        error_handlers::ErrorHandler,
        update_listeners::UpdateListener,
    },
    types::Update,
};
use futures::StreamExt;
use std::sync::Arc;

pub struct Dispatcher<Err, ErrHandler> {
    bot_name: Arc<str>,
    demux: Demux<DispatcherContext<Update>, Err>,
    error_handler: ErrHandler,
}

impl<Err, ErrHandler> Dispatcher<Err, ErrHandler>
where
    Err: 'static,
    ErrHandler: ErrorHandler<DispatchError<Update, Err>>,
{
    pub async fn dispatch_one(&self, upd: Update) {
        match self.demux.handle(DispatcherContext::new(upd, self.bot_name.clone())).await {
            Ok(res) => match res {
                HandleResult::Ok => {}
                HandleResult::Err(e) => {
                    self.error_handler.handle_error(DispatchError::HandlerError(e)).await
                }
            },
            Err(e) => self.error_handler.handle_error(DispatchError::NoHandler(e.upd)).await,
        }
    }

    pub async fn dispatch_with_listener<ListenerErr>(
        &self,
        listener: impl UpdateListener<ListenerErr>,
    ) where
        ListenerErr: Into<Err>,
    {
        listener
            .for_each_concurrent(None, |res| async move {
                match res {
                    Ok(upd) => self.dispatch_one(upd).await,
                    Err(e) => {
                        // TODO: UpdateListenerError
                        self.error_handler.handle_error(DispatchError::HandlerError(e.into())).await
                    }
                };
            })
            .await;
    }
}

pub struct DispatcherBuilder<Err, Handler> {
    bot_name: Arc<str>,
    demux: DemuxBuilder<DispatcherContext<Update>, Err>,
    error_handler: Handler,
}

impl<Err> DispatcherBuilder<Err, ()> {
    pub fn new(bot_name: impl Into<Arc<str>>) -> Self {
        DispatcherBuilder {
            bot_name: bot_name.into(),
            demux: DemuxBuilder::new(),
            error_handler: (),
        }
    }

    pub fn error_handler<H>(self, error_handler: H) -> DispatcherBuilder<Err, H>
    where
        H: ErrorHandler<DispatchError<Update, Err>>,
    {
        let DispatcherBuilder { bot_name, demux, .. } = self;
        DispatcherBuilder { bot_name, demux, error_handler }
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
        let DispatcherBuilder { bot_name, demux, error_handler } = self;
        Dispatcher { bot_name, demux: demux.build(), error_handler }
    }
}
