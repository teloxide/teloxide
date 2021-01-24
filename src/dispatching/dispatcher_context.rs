use crate::dispatching::core::{Parser, ParserOut, RecombineFrom};
use std::sync::Arc;

pub struct DispatcherContext<Upd> {
    pub upd: Upd,
    pub bot_name: Arc<str>,
}

impl<Upd> DispatcherContext<Upd> {
    pub fn new(upd: Upd, bot_name: impl Into<Arc<str>>) -> Self {
        DispatcherContext { upd, bot_name: bot_name.into() }
    }

    pub fn parse_upd<OtherUpd, Rest>(
        self,
        f: &impl Parser<Upd, OtherUpd, Rest>,
    ) -> Result<ParserOut<DispatcherContext<OtherUpd>, Rest>, Self> {
        let DispatcherContext { upd, bot_name } = self;
        let ParserOut { data, rest } = match f.parse(upd) {
            Ok(out) => out,
            Err(upd) => return Err(DispatcherContext { upd, bot_name }),
        };
        Ok(ParserOut::new(DispatcherContext::new(data, bot_name), rest))
    }
}

impl<ParserT, UpdFrom, UpdTo, Rest> RecombineFrom<ParserT, DispatcherContext<UpdFrom>, Rest>
    for DispatcherContext<UpdTo>
where
    UpdTo: RecombineFrom<ParserT, UpdFrom, Rest>,
{
    fn recombine(info: ParserOut<DispatcherContext<UpdFrom>, Rest>) -> Self {
        let (DispatcherContext { upd, bot_name }, rest) = info.into_inner();
        let new_upd = UpdTo::recombine(ParserOut::new(upd, rest));
        DispatcherContext { upd: new_upd, bot_name }
    }
}
