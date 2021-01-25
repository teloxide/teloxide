use crate::dispatching::{
    core::{Handler, IntoHandler, Parser, ParserHandler, RecombineFrom},
    dispatcher_context::DispatcherContext,
};
use std::marker::PhantomData;

pub struct UpdateParser<GenUpd, NextUpd, Rest, Err, ParserT> {
    parser: ParserT,
    phantom: PhantomData<(GenUpd, NextUpd, Rest, Err)>,
}

impl<GenUpd, NextUpd, Rest, Err, ParserT> UpdateParser<GenUpd, NextUpd, Rest, Err, ParserT> {
    pub fn into_inner(self) -> ParserT {
        self.parser
    }
}

impl<GenUpd, NextUpd, Rest, Err, ParserT> UpdateParser<GenUpd, NextUpd, Rest, Err, ParserT>
where
    GenUpd: 'static,
    ParserT: Parser<GenUpd, NextUpd, Rest> + 'static,
    GenUpd: RecombineFrom<ParserT, NextUpd, Rest>,
{
    pub fn new(parser: ParserT) -> Self {
        UpdateParser { parser, phantom: PhantomData }
    }

    pub fn by<F, H>(self, f: F) -> ParserHandler<ParserT, GenUpd, NextUpd, Rest, Err, H>
    where
        H: Handler<DispatcherContext<NextUpd>, Err> + 'static,
        F: IntoHandler<H>,
    {
        let UpdateParser { parser, .. } = self;
        ParserHandler::new(parser, f)
    }
}
