use crate::{
    dispatching::core::{Guard, Guards, HandleFuture, HandleResult, Handler},
    types::Message,
};
use std::{marker::PhantomData, sync::Arc};

pub struct GuardsHandler {
    guards: Arc<Guards<Message>>,
}

impl GuardsHandler {
    pub fn new(guards: Guards<Message>) -> Self {
        GuardsHandler { guards: Arc::new(guards) }
    }
}

impl<Err> Handler<Message, Err> for GuardsHandler {
    fn handle(&self, data: Message) -> HandleFuture<Err, Message> {
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

impl<GuardT, HandlerT, Err> Handler<Message, Err> for GuardHandler<GuardT, HandlerT, Err>
where
    GuardT: Guard<Message> + Send + Sync + 'static,
    HandlerT: Handler<Message, Err> + Send + Sync + 'static,
    Err: 'static,
{
    fn handle(&self, data: Message) -> HandleFuture<Err, Message> {
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
