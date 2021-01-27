use crate::dispatching::{
    core::{Guard, Guards, HandleFuture, HandleResult, Handler},
};
use std::{marker::PhantomData, sync::Arc};
use crate::dispatching::core::Context;

pub struct GuardsHandler<Upd, Ctx: Context<Upd = Upd>> {
    guards: Arc<Guards<Ctx>>,
}

impl<Upd, Ctx: Context<Upd = Upd>> GuardsHandler<Upd, Ctx> {
    pub fn new(guards: Guards<Ctx>) -> Self {
        GuardsHandler { guards: Arc::new(guards) }
    }
}

impl<Upd, Ctx, Err> Handler<Ctx, Err> for GuardsHandler<Upd, Ctx>
where
    Upd: Send + Sync + 'static,
    Ctx: Context<Upd = Upd> + Send + Sync+ 'static,
{
    fn handle(&self, ctx: Ctx) -> HandleFuture<Err, Ctx> {
        let guards = self.guards.clone();

        Box::pin(async move {
            match guards.check(&ctx).await {
                true => Err(ctx),
                false => Ok(HandleResult::Ok),
            }
        })
    }
}

pub struct GuardHandler<Guard, Handler, Err> {
    guard: Arc<Guard>,
    wrong_handler: Arc<Handler>,
    phantom: PhantomData<tokio::sync::Mutex<Err>>,
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

impl<Ctx, Upd, GuardT, HandlerT, Err> Handler<Ctx, Err>
    for GuardHandler<GuardT, HandlerT, Err>
where
    Ctx: Context<Upd = Upd> + Send + Sync + 'static,
    Upd: Send + Sync + 'static,
    GuardT: Guard<Ctx> + Send + Sync + 'static,
    HandlerT: Handler<Ctx, Err> + Send + Sync + 'static,
    Err: 'static,
{
    fn handle(&self, data: Ctx) -> HandleFuture<Err, Ctx> {
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
