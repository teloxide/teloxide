use crate::{
    dispatching::core::{Parser, ParserOut, RecombineFrom},
    Bot,
};
use std::sync::Arc;
use crate::dispatching::core::GetCtx;

pub struct DispatcherContext<Upd> {
    pub upd: Upd,
    pub bot: Bot,
    pub bot_name: Arc<str>,
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
