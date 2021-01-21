use crate::{
    dispatching::{
        core::{Demux, HandleFuture, Handler, MapParser, Parser, ParserOut, RecombineFrom},
        updates::UpdateRest,
    },
    types::{Message, Update},
};

type UpdMesParser<Parser1, Parser2> = MapParser<Parser1, Parser2, Message, UpdateRest, (), Message>;

pub struct MessageHandler<Parser1, Parser2, HandlerT, Err> {
    parser: UpdMesParser<Parser1, Parser2>,
    handler: HandlerT,
    demux: Demux<Message, Err>,
}

impl<Parser1, Parser2, HandlerT, Err> MessageHandler<Parser1, Parser2, HandlerT, Err> {
    pub fn new(
        parser: UpdMesParser<Parser1, Parser2>,
        handler: HandlerT,
        demux: Demux<Message, Err>,
    ) -> Self {
        MessageHandler { parser, handler, demux }
    }
}

impl<Parser1, Parser2, Err, HandlerT> Handler<Update, Err, HandleFuture<Err>>
    for MessageHandler<Parser1, Parser2, HandlerT, Err>
where
    Parser1: Parser<Update, Message, UpdateRest>,
    Parser2: Parser<Message, Message, ()>,
    HandlerT: Handler<Message, Err, HandleFuture<Err>>,
    Update: RecombineFrom<Parser1, From = Message, Rest = UpdateRest>,
{
    fn handle(&self, update: Update) -> Result<HandleFuture<Err>, Update> {
        let ParserOut { data: mes, rest } = self.parser.parse(update)?;
        match self.demux.handle(mes) {
            Ok(fut) => Ok(fut),
            Err(upd) => self.handler.handle(upd).map_err(|e| {
                <Update as RecombineFrom<Parser1>>::recombine(ParserOut::new(e, rest.0))
            }),
        }
    }
}
