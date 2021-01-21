use crate::{
    dispatching::core::{Guard, Guards, HandleFuture, HandleResult, Handler},
    types::Message,
};
use futures::FutureExt;
use std::{future::Future, marker::PhantomData};

pub struct GuardsHandler {
    guards: Guards<Message>,
}

impl GuardsHandler {
    pub fn new(guards: Guards<Message>) -> Self {
        GuardsHandler { guards }
    }
}

impl<Err> Handler<Message, Err, HandleFuture<Err>> for GuardsHandler {
    fn handle(&self, data: Message) -> Result<HandleFuture<Err>, Message> {
        match self.guards.check(&data) {
            true => Err(data),
            false => Ok(Box::pin(async { HandleResult::Ok })),
        }
    }
}

pub struct GuardHandler<Guard, Handler, Err, HFut> {
    guard: Guard,
    wrong_handler: Handler,
    phantom: PhantomData<(Err, HFut)>,
}

impl<Guard, Handler, Err, HFut> GuardHandler<Guard, Handler, Err, HFut> {
    pub fn new(guard: Guard, wrong_handler: Handler) -> Self {
        GuardHandler { guard, wrong_handler, phantom: PhantomData }
    }
}

impl<GuardT, HandlerT, Err, HFut> Handler<Message, Err, HandleFuture<Err>>
    for GuardHandler<GuardT, HandlerT, Err, HFut>
where
    GuardT: Guard<Message>,
    HandlerT: Handler<Message, Err, HFut>,
    HFut: Future + Send + 'static,
    HFut::Output: Into<HandleResult<Err>> + 'static,
    Err: 'static,
{
    fn handle(&self, data: Message) -> Result<HandleFuture<Err>, Message> {
        match self.guard.check(&data) {
            true => Err(data),
            false => self.wrong_handler.handle(data).map(|fut| Box::pin(fut.map(Into::into)) as _),
        }
    }
}
