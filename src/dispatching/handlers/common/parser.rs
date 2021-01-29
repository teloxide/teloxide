use crate::{
    dispatching::{
        core::{HandleFuture, Handler, IntoHandler},
        dev::Context,
    },
    types::Update,
};
use std::{marker::PhantomData, sync::Arc};

pub struct UpdateHandlerBuilder<Ctx, Err> {
    phantom: PhantomData<tokio::sync::Mutex<(Ctx, Err)>>,
}

impl<Ctx, Err> UpdateHandlerBuilder<Ctx, Err>
where
    Ctx: Context<Upd = Update>,
{
    pub fn new() -> Self {
        UpdateHandlerBuilder { phantom: PhantomData }
    }

    pub fn by<F, H>(self, f: F) -> UpdateHandler<Ctx, Err, H>
    where
        H: Handler<Ctx, Err> + 'static,
        F: IntoHandler<H>,
    {
        UpdateHandler::new(f)
    }
}

pub struct UpdateHandler<Ctx, Err, HandlerT> {
    handler: Arc<HandlerT>,
    phantom: PhantomData<tokio::sync::Mutex<(Ctx, Err)>>,
}

impl<Ctx, Err, HandlerT> UpdateHandler<Ctx, Err, HandlerT>
where
    HandlerT: Handler<Ctx, Err>,
    Ctx: Context<Upd = Update>,
{
    pub fn new<H>(handler: H) -> Self
    where
        H: IntoHandler<HandlerT>,
    {
        UpdateHandler { handler: Arc::new(handler.into_handler()), phantom: PhantomData }
    }
}

impl<Ctx, Err, HandlerT> Handler<Ctx, Err> for UpdateHandler<Ctx, Err, HandlerT>
where
    Err: 'static,
    Ctx: Context<Upd = Update>,
    HandlerT: Handler<Ctx, Err> + Send + Sync + 'static,
{
    fn handle(&self, cx: Ctx) -> HandleFuture<Err, Ctx> {
        self.handler.handle(cx)
    }
}
