use crate::{
    dispatching::{
        core::{Demux, HandleFuture, Handler, MapParser, Parser, ParserOut, RecombineFrom},
        updates::UpdateRest,
    },
    types::{Message, Update},
};
use std::sync::Arc;

type UpdMesParser<Parser1, Parser2> = MapParser<Parser1, Parser2, Message, UpdateRest, (), Message>;

pub struct MessageHandler<Parser1, Parser2, HandlerT, Err> {
    parser: Arc<UpdMesParser<Parser1, Parser2>>,
    handler: Arc<HandlerT>,
    demux: Arc<Demux<Message, Err>>,
}

impl<Parser1, Parser2, HandlerT, Err> MessageHandler<Parser1, Parser2, HandlerT, Err> {
    pub fn new(
        parser: UpdMesParser<Parser1, Parser2>,
        handler: HandlerT,
        demux: Demux<Message, Err>,
    ) -> Self {
        MessageHandler {
            parser: Arc::new(parser),
            handler: Arc::new(handler),
            demux: Arc::new(demux),
        }
    }
}

impl<Parser1, Parser2, Err, HandlerT> Handler<Update, Err>
    for MessageHandler<Parser1, Parser2, HandlerT, Err>
where
    Err: Send + 'static,
    Parser1: Parser<Update, Message, UpdateRest> + Send + Sync + 'static,
    Parser2: Parser<Message, Message, ()> + Send + Sync + 'static,
    HandlerT: Handler<Message, Err> + Send + Sync + 'static,
    Update: RecombineFrom<Parser1, From = Message, Rest = UpdateRest>,
{
    fn handle(&self, update: Update) -> HandleFuture<Err, Update> {
        let parser = self.parser.clone();
        let demux = self.demux.clone();
        let handler = self.handler.clone();

        Box::pin(async move {
            let ParserOut { data: mes, rest } = parser.parse(update)?;
            match demux.handle(mes).await {
                Ok(res) => Ok(res),
                Err(upd) => handler.handle(upd).await.map_err(|e| {
                    <Update as RecombineFrom<Parser1>>::recombine(ParserOut::new(e, rest.0))
                }),
            }
        })
    }
}
