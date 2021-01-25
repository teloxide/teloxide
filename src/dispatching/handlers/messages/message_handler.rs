use crate::{
    dispatching::{
        core::{Demux, HandleFuture, Handler, MapParser, Parser, ParserOut, RecombineFrom},
        dispatcher_context::DispatcherContext,
        updates::UpdateRest,
    },
    types::{Message, Update},
};
use std::sync::Arc;

pub struct MessageHandler<ParserT, HandlerT, Err> {
    parser: Arc<ParserT>,
    handler: Arc<HandlerT>,
    demux: Arc<Demux<DispatcherContext<Message>, Err>>,
}

impl<ParserT, HandlerT, Err> MessageHandler<ParserT, HandlerT, Err> {
    pub fn new(
        parser: ParserT,
        handler: HandlerT,
        demux: Demux<DispatcherContext<Message>, Err>,
    ) -> Self {
        MessageHandler {
            parser: Arc::new(parser),
            handler: Arc::new(handler),
            demux: Arc::new(demux),
        }
    }
}

impl<ParserT, Err, HandlerT> Handler<DispatcherContext<Update>, Err>
    for MessageHandler<ParserT, HandlerT, Err>
where
    Err: Send + 'static,
    ParserT: Parser<Update, Message, UpdateRest> + Send + Sync + 'static,
    HandlerT: Handler<DispatcherContext<Message>, Err> + Send + Sync + 'static,
    Update: RecombineFrom<ParserT, Message, UpdateRest>,
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
