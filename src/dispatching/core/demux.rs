use crate::dispatching::core::{HandleFuture, Handler};
use std::sync::Arc;

pub struct Demux<Upd, Err> {
    handlers: Arc<[Box<dyn Handler<Upd, Err> + Send + Sync>]>,
}

pub struct DemuxBuilder<Upd, Err> {
    handlers: Vec<Box<dyn Handler<Upd, Err> + Send + Sync>>,
}

impl<Upd, Err> DemuxBuilder<Upd, Err> {
    pub fn new() -> Self {
        DemuxBuilder { handlers: Vec::new() }
    }

    pub fn add_service(&mut self, service: impl Handler<Upd, Err> + Send + Sync + 'static) {
        self.handlers.push(Box::new(service) as _);
    }

    pub fn build(self) -> Demux<Upd, Err> {
        Demux { handlers: self.handlers.into() }
    }
}

impl<Upd: Send + 'static, Err: 'static> Handler<Upd, Err> for Demux<Upd, Err> {
    fn handle(&self, update: Upd) -> HandleFuture<Err, Upd> {
        let handlers = self.handlers.clone();
        Box::pin(async move {
            let mut update = update;
            for handler in handlers.iter() {
                match handler.handle(update).await {
                    Ok(res) => return Ok(res),
                    Err(upd) => {
                        update = upd;
                        continue;
                    }
                }
            }
            Err(update)
        })
    }
}
