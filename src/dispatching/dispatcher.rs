use crate::{
    dispatching::{
        core::{Demux, DemuxBuilder, DispatchError, HandleResult, Handler},
        error_handlers::ErrorHandler,
        update_listeners::UpdateListener,
    },
    types::Update,
};
use futures::StreamExt;

pub struct Dispatcher<Err, ErrHandler> {
    demux: Demux<Update, Err>,
    error_handler: ErrHandler,
}

impl<Err, ErrHandler> Dispatcher<Err, ErrHandler>
where
    Err: 'static,
    ErrHandler: ErrorHandler<DispatchError<Update, Err>>,
{
    pub async fn dispatch_one(&self, upd: Update) {
        match self.demux.handle(upd).await {
            Ok(res) => match res {
                HandleResult::Ok => {}
                HandleResult::Err(e) => {
                    self.error_handler.handle_error(DispatchError::HandlerError(e)).await
                }
            },
            Err(e) => self.error_handler.handle_error(DispatchError::NoHandler(e)).await,
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
    demux: DemuxBuilder<Update, Err>,
    error_handler: Handler,
}

impl<Err> DispatcherBuilder<Err, ()> {
    pub fn new() -> Self {
        DispatcherBuilder { demux: DemuxBuilder::new(), error_handler: () }
    }

    pub fn error_handler<H>(self, error_handler: H) -> DispatcherBuilder<Err, H>
    where
        H: ErrorHandler<DispatchError<Update, Err>>,
    {
        let DispatcherBuilder { demux, .. } = self;
        DispatcherBuilder { demux, error_handler }
    }
}

impl<Err, ErrHandler> DispatcherBuilder<Err, ErrHandler> {
    pub fn handle(mut self, handler: impl Handler<Update, Err> + Send + Sync + 'static) -> Self {
        self.demux.add_service(handler);
        self
    }
}

impl<Err, ErrHandler> DispatcherBuilder<Err, ErrHandler>
where
    ErrHandler: ErrorHandler<DispatchError<Update, Err>>,
{
    pub fn build(self) -> Dispatcher<Err, ErrHandler> {
        let DispatcherBuilder { demux, error_handler } = self;
        Dispatcher { demux: demux.build(), error_handler }
    }
}
