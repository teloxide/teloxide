use crate::dispatching::core::{
    HandleResult, Handler, IntoHandler, Parser, ParserHandler, RecombineFrom,
};
use std::{future::Future, marker::PhantomData};

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
    GenUpd: RecombineFrom<ParserT, From = NextUpd, Rest = Rest>,
{
    pub fn new(parser: ParserT) -> Self {
        UpdateParser { parser, phantom: PhantomData }
    }

    pub fn by<F, H, Fut>(self, f: F) -> ParserHandler<ParserT, GenUpd, NextUpd, Rest, Err, H, Fut>
    where
        H: Handler<NextUpd, Err, Fut> + 'static,
        F: IntoHandler<H>,
        Fut: Future + Send + 'static,
        Fut::Output: Into<HandleResult<Err>>,
    {
        let UpdateParser { parser, .. } = self;
        ParserHandler::new(parser, f)
    }
}
