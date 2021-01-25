use crate::dispatching::{
    core::{Guard, Guards, HandleFuture, HandleResult, Handler},
    dispatcher_context::DispatcherContext,
};
use std::{marker::PhantomData, sync::Arc};

pub struct GuardsHandler<Upd> {
    guards: Arc<Guards<DispatcherContext<Upd>>>,
}

impl<Upd> GuardsHandler<Upd> {
    pub fn new(guards: Guards<DispatcherContext<Upd>>) -> Self {
        GuardsHandler { guards: Arc::new(guards) }
    }
}

impl<Upd, Err> Handler<DispatcherContext<Upd>, Err> for GuardsHandler<Upd>
where
    Upd: Send + Sync + 'static,
{
    fn handle(&self, data: DispatcherContext<Upd>) -> HandleFuture<Err, DispatcherContext<Upd>> {
        let guards = self.guards.clone();

        Box::pin(async move {
            match guards.check(&data).await {
                true => Err(data),
                false => Ok(HandleResult::Ok),
            }
        })
    }
}

pub struct GuardHandler<Guard, Handler, Err> {
    guard: Arc<Guard>,
    wrong_handler: Arc<Handler>,
    phantom: PhantomData<Err>,
}

impl<Guard, Handler, Err> GuardHandler<Guard, Handler, Err> {
    pub fn new(guard: Guard, wrong_handler: Handler) -> Self {
        GuardHandler {
            guard: Arc::new(guard),
            wrong_handler: Arc::new(wrong_handler),
            phantom: PhantomData,
        }
    }
}

impl<Upd, GuardT, HandlerT, Err> Handler<DispatcherContext<Upd>, Err>
    for GuardHandler<GuardT, HandlerT, Err>
where
    Upd: Send + Sync + 'static,
    GuardT: Guard<DispatcherContext<Upd>> + Send + Sync + 'static,
    HandlerT: Handler<DispatcherContext<Upd>, Err> + Send + Sync + 'static,
    Err: 'static,
{
    fn handle(&self, data: DispatcherContext<Upd>) -> HandleFuture<Err, DispatcherContext<Upd>> {
        let guard = self.guard.clone();
        let wrong_handler = self.wrong_handler.clone();

        Box::pin(async move {
            match guard.check(&data).await {
                true => Err(data),
                false => wrong_handler.handle(data).await,
            }
        })
    }
}
