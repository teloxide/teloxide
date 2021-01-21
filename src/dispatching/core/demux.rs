use crate::dispatching::core::{HandleFuture, Handler};
use std::sync::Arc;

pub struct Demux<Upd, Err> {
    handlers: Arc<[Box<dyn Handler<Upd, Err, HandleFuture<Err>>>]>,
}

pub struct DemuxBuilder<Upd, Err> {
    handlers: Vec<Box<dyn Handler<Upd, Err, HandleFuture<Err>>>>,
}

impl<Upd, Err> DemuxBuilder<Upd, Err> {
    pub fn new() -> Self {
        DemuxBuilder { handlers: Vec::new() }
    }

    pub fn add_service(&mut self, service: impl Handler<Upd, Err, HandleFuture<Err>> + 'static) {
        self.handlers.push(Box::new(service) as _);
    }

    pub fn build(self) -> Demux<Upd, Err> {
        Demux { handlers: self.handlers.into() }
    }
}

impl<Upd: 'static, Err> Handler<Upd, Err, HandleFuture<Err>> for Demux<Upd, Err> {
    fn handle(&self, update: Upd) -> Result<HandleFuture<Err>, Upd> {
        let mut update = update;
        for handler in self.handlers.iter() {
            match handler.handle(update) {
                Ok(fut) => return Ok(fut),
                Err(upd) => {
                    update = upd;
                    continue;
                }
            }
        }
        Err(update)
    }
}
