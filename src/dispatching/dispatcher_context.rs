use crate::{
    dispatching::core::{
        Context, ContextWith, GetCtx, ParseContext, Parser, ParserOut, RecombineFrom,
    },
    Bot,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct DispatcherContext<Upd> {
    pub upd: Upd,
    pub bot: Bot,
    pub bot_name: Arc<str>,
}

impl<Upd> Context for DispatcherContext<Upd> {
    type Upd = Upd;

    fn get_upd(&self) -> &Self::Upd {
        &self.upd
    }
}

impl<Upd1, Upd2> ContextWith<Upd2> for DispatcherContext<Upd1> {
    type Context = DispatcherContext<Upd2>;
}

impl<Upd1, Upd2> ParseContext<Upd2> for DispatcherContext<Upd1> {
    fn parse<Rest>(
        self,
        f: impl Fn(Upd1) -> Result<ParserOut<Upd2, Rest>, Upd1>,
    ) -> Result<(DispatcherContext<Upd2>, Rest), Self> {
        let Self { upd, bot, bot_name } = self;
        let ParserOut { data: upd, rest } = match f(upd) {
            Ok(t) => t,
            Err(upd) => return Err(DispatcherContext { upd, bot, bot_name }),
        };
        Ok((DispatcherContext { upd, bot, bot_name }, rest))
    }

    fn recombine<Parser, Rest>(info: ParserOut<Self::Context, Rest>) -> Self
    where
        Upd1: RecombineFrom<Parser, Upd2, Rest>,
    {
        let ParserOut { data: DispatcherContext { upd, bot, bot_name }, rest } = info;
        DispatcherContext { upd: Upd1::recombine(ParserOut::new(upd, rest)), bot, bot_name }
    }
}

impl<Upd> DispatcherContext<Upd> {
    pub fn new(upd: Upd, bot: Bot, bot_name: impl Into<Arc<str>>) -> Self {
        DispatcherContext { upd, bot, bot_name: bot_name.into() }
    }

    pub fn parse_upd<OtherUpd, Rest>(
        self,
        f: &impl Parser<Upd, OtherUpd, Rest>,
    ) -> Result<ParserOut<DispatcherContext<OtherUpd>, Rest>, Self> {
        let DispatcherContext { upd, bot, bot_name } = self;
        let ParserOut { data, rest } = match f.parse(upd) {
            Ok(out) => out,
            Err(upd) => return Err(DispatcherContext { upd, bot, bot_name }),
        };
        Ok(ParserOut::new(DispatcherContext::new(data, bot, bot_name), rest))
    }
}

impl<Upd> GetCtx<DispatcherContext<Upd>> for DispatcherContext<Upd> {
    fn get(&self) -> &DispatcherContext<Upd> {
        self
    }

    fn get_own(self) -> DispatcherContext<Upd> {
        self
    }
}

impl<ParserT, UpdFrom, UpdTo, Rest> RecombineFrom<ParserT, DispatcherContext<UpdFrom>, Rest>
    for DispatcherContext<UpdTo>
where
    UpdTo: RecombineFrom<ParserT, UpdFrom, Rest>,
{
    fn recombine(info: ParserOut<DispatcherContext<UpdFrom>, Rest>) -> Self {
        let (DispatcherContext { upd, bot, bot_name }, rest) = info.into_inner();
        let new_upd = UpdTo::recombine(ParserOut::new(upd, rest));
        DispatcherContext { upd: new_upd, bot, bot_name }
    }
}
