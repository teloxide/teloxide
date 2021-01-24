use crate::{
    dispatching::{
        core::{Demux, HandleFuture, Handler, MapParser, Parser, ParserOut, RecombineFrom},
        dispatcher_context::DispatcherContext,
        updates::UpdateRest,
    },
    types::{Message, Update},
};
use std::sync::Arc;

type UpdMesParser<Parser1, Parser2> = MapParser<Parser1, Parser2, Message, UpdateRest, (), Message>;

pub struct MessageHandler<Parser1, Parser2, HandlerT, Err> {
    parser: Arc<UpdMesParser<Parser1, Parser2>>,
    handler: Arc<HandlerT>,
    demux: Arc<Demux<DispatcherContext<Message>, Err>>,
}

impl<Parser1, Parser2, HandlerT, Err> MessageHandler<Parser1, Parser2, HandlerT, Err> {
    pub fn new(
        parser: UpdMesParser<Parser1, Parser2>,
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

impl<Parser1, Parser2, Err, HandlerT> Handler<DispatcherContext<Update>, Err>
    for MessageHandler<Parser1, Parser2, HandlerT, Err>
where
    Err: Send + 'static,
    Parser1: Parser<Update, Message, UpdateRest> + Send + Sync + 'static,
    Parser2: Parser<Message, Message, ()> + Send + Sync + 'static,
    HandlerT: Handler<DispatcherContext<Message>, Err> + Send + Sync + 'static,
    Update: RecombineFrom<Parser1, Message, UpdateRest>,
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
                    .map_err(|e| <DispatcherContext<Update>>::recombine(ParserOut::new(e, rest.0))),
            }
        })
    }
}
