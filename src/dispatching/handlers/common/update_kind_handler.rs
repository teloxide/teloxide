use crate::{
    dispatching::{
        core::{
            Context, ContextWith, Demux, HandleFuture, Handler, ParseContext, Parser, ParserOut,
            RecombineFrom,
        },
        updates::UpdateRest,
    },
    types::Update,
};
use std::sync::Arc;

pub struct UpdateKindHandler<Upd, Ctx: Context<Upd = Upd>, ParserT, HandlerT, Err> {
    parser: Arc<ParserT>,
    handler: Arc<HandlerT>,
    demux: Arc<Demux<Ctx, Err>>,
}

impl<Upd, Ctx, ParserT, HandlerT, Err> UpdateKindHandler<Upd, Ctx, ParserT, HandlerT, Err>
where
    Ctx: Context<Upd = Upd>,
{
    pub fn new(parser: ParserT, handler: HandlerT, demux: Demux<Ctx, Err>) -> Self {
        UpdateKindHandler {
            parser: Arc::new(parser),
            handler: Arc::new(handler),
            demux: Arc::new(demux),
        }
    }
}

impl<Upd, Ctx, ParserT, Err, HandlerT> Handler<Ctx, Err>
    for UpdateKindHandler<Upd, Ctx::Context, ParserT, HandlerT, Err>
where
    Upd: Send + Sync + 'static,
    Err: Send + 'static,
    Ctx: Context<Upd = Update> + ParseContext<Upd> + Send + 'static,
    <Ctx as ContextWith<Upd>>::Context: Send,
    ParserT: Parser<Update, Upd, UpdateRest> + Send + Sync + 'static,
    HandlerT: Handler<Ctx::Context, Err> + Send + Sync + 'static,
    Update: RecombineFrom<ParserT, Upd, UpdateRest>,
{
    fn handle(&self, cx: Ctx) -> HandleFuture<Err, Ctx> {
        let parser = self.parser.clone();
        let demux = self.demux.clone();
        let handler = self.handler.clone();

        Box::pin(async move {
            let (cx, rest) = cx.parse(|upd| parser.as_ref().parse(upd))?;
            match demux.handle(cx).await {
                Ok(res) => Ok(res),
                Err(upd) => {
                    handler.handle(upd).await.map_err(|e| Ctx::recombine(ParserOut::new(e, rest)))
                }
            }
        })
    }
}
