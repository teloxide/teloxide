use crate::{
    dispatching::{
        core::{Demux, HandleFuture, Handler, Parser, ParserOut, RecombineFrom},
        dispatcher_context::DispatcherContext,
        updates::UpdateRest,
    },
    types::Update,
};
use std::sync::Arc;

pub struct UpdateKindHandler<Upd, ParserT, HandlerT, Err> {
    parser: Arc<ParserT>,
    handler: Arc<HandlerT>,
    demux: Arc<Demux<DispatcherContext<Upd>, Err>>,
}

impl<Upd, ParserT, HandlerT, Err> UpdateKindHandler<Upd, ParserT, HandlerT, Err> {
    pub fn new(
        parser: ParserT,
        handler: HandlerT,
        demux: Demux<DispatcherContext<Upd>, Err>,
    ) -> Self {
        UpdateKindHandler {
            parser: Arc::new(parser),
            handler: Arc::new(handler),
            demux: Arc::new(demux),
        }
    }
}

impl<Upd, ParserT, Err, HandlerT> Handler<DispatcherContext<Update>, Err>
    for UpdateKindHandler<Upd, ParserT, HandlerT, Err>
where
    Upd: Send + Sync + 'static,
    Err: Send + 'static,
    ParserT: Parser<Update, Upd, UpdateRest> + Send + Sync + 'static,
    HandlerT: Handler<DispatcherContext<Upd>, Err> + Send + Sync + 'static,
    Update: RecombineFrom<ParserT, Upd, UpdateRest>,
{
    fn handle(
        &self,
        cx: DispatcherContext<Update>,
    ) -> HandleFuture<Err, DispatcherContext<Update>> {
        let parser = self.parser.clone();
        let demux = self.demux.clone();
        let handler = self.handler.clone();

        Box::pin(async move {
            let ParserOut { data: cx, rest } = cx.parse_upd(parser.as_ref())?;
            match demux.handle(cx).await {
                Ok(res) => Ok(res),
                Err(upd) => handler
                    .handle(upd)
                    .await
                    .map_err(|e| <DispatcherContext<Update>>::recombine(ParserOut::new(e, rest))),
            }
        })
    }
}
