use crate::{
    dispatching::{
        core::{
            DemuxBuilder, Guard, Guards, Handler, IntoGuard, IntoHandler, MapParser, OrGuard,
            Parser, ParserOut, RecombineFrom,
        },
        handlers::messages::{
            guard_handlers::{GuardHandler, GuardsHandler},
            message_handler::MessageHandler,
        },
        updates::UpdateRest,
    },
    types::{Message, Update},
};

pub(crate) mod parser {
    pub struct Common;
    pub struct NewChatMembers;
    pub struct LeftChatMember;
    pub struct NewChatTitle;
    pub struct NewChatPhoto;
    pub struct DeleteChatPhoto;
    pub struct GroupChatCreated;
    pub struct SupergroupChatCreated;
    pub struct ChannelChatCreated;
    pub struct Migrate;
    pub struct Pinned;
    pub struct Invoice;
    pub struct SuccessfulPayment;
    pub struct ConnectedWebsite;
    pub struct PassportData;
    pub struct Dice;
}

macro_rules! impl_parser {
        ($($ty:ident,)*) => {
            $(
                impl Parser<Message, Message, ()> for parser::$ty {
                    fn parse(&self, update: Message) -> Result<ParserOut<Message, ()>, Message> {
                        match &update.kind {
                            crate::types::MessageKind::$ty(_) => Ok(ParserOut::new(update, ())),
                            _ => Err(update),
                        }
                    }
                }
            )*
        }
    }

impl_parser!(
    Common,
    NewChatMembers,
    LeftChatMember,
    NewChatTitle,
    NewChatPhoto,
    DeleteChatPhoto,
    GroupChatCreated,
    SupergroupChatCreated,
    ChannelChatCreated,
    Migrate,
    Pinned,
    Invoice,
    SuccessfulPayment,
    ConnectedWebsite,
    PassportData,
    Dice,
);
impl<Parser1, Parser2> RecombineFrom<MapParser<Parser1, Parser2, Message, UpdateRest, (), Message>>
    for Update
where
    Update: RecombineFrom<Parser1, From = Message, Rest = UpdateRest>,
{
    type From = Message;
    type Rest = (UpdateRest, ());

    fn recombine(info: ParserOut<Self::From, Self::Rest>) -> Self {
        let (out, (rest1, _)) = info.into_inner();
        <Update as RecombineFrom<Parser1>>::recombine(ParserOut::new(out, rest1))
    }
}

pub struct MessageParser<UpdateParser, ParserT, Err> {
    update_parser: UpdateParser,
    parser: ParserT,
    demux: DemuxBuilder<Message, Err>,
    guards: Guards<Message>,
    last_guard: Option<Box<dyn Guard<Message> + Send + Sync>>,
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err>
where
    UpdateParser: Parser<Update, Message, UpdateRest>,
    ParserT: Parser<Message, Message, ()> + 'static,
    Update: RecombineFrom<UpdateParser, From = Message, Rest = UpdateRest>,
{
    pub fn new(update_parser: UpdateParser, parser: ParserT) -> Self {
        MessageParser {
            update_parser,
            parser,
            demux: DemuxBuilder::new(),
            guards: Guards::new(),
            last_guard: None,
        }
    }
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err>
where
    Err: Send + Sync + 'static,
    UpdateParser: Parser<Update, Message, UpdateRest>,
    ParserT: Parser<Message, Message, ()> + 'static,
    Update: RecombineFrom<UpdateParser, From = Message, Rest = UpdateRest>,
{
    pub fn by<F, H>(mut self, f: F) -> MessageHandler<UpdateParser, ParserT, H, Err>
    where
        H: Handler<Message, Err> + 'static,
        F: IntoHandler<H>,
    {
        self.create_guards_service();

        let MessageParser { update_parser: parent, parser, demux, .. } = self;
        let parser = MapParser::new(parent, parser);
        MessageHandler::new(parser, f.into_handler(), demux.build())
    }
}

impl<UpdateParser, ParserT, Err: Send + Sync + 'static> MessageParser<UpdateParser, ParserT, Err> {
    pub fn with_guard<G: Guard<Message> + Send + Sync + 'static>(
        mut self,
        guard: impl IntoGuard<Message, G> + 'static,
    ) -> Self {
        self.add_last_to_guards();
        self.last_guard = Some(Box::new(guard.into_guard()) as _);
        self
    }

    pub fn or_with_guard<G: Guard<Message> + Send + Sync + 'static>(
        mut self,
        guard: impl IntoGuard<Message, G> + 'static,
    ) -> Self {
        let prev = self
            .last_guard
            .take()
            .expect("or function must be called after using .with_* funtion!");
        self.last_guard = Some(Box::new(OrGuard::new(prev, guard.into_guard())) as _);
        self
    }

    pub fn or_else<F, H>(mut self, func: F) -> Self
    where
        F: IntoHandler<H>,
        H: Handler<Message, Err> + Send + Sync + 'static,
        Err: Send + Sync + 'static,
    {
        let prev_guard = self
            .last_guard
            .take()
            .expect("or_else function must be called after using .with_* funtion!");
        let wrong_handler = func.into_handler();

        self.create_guards_service();
        self.demux.add_service(GuardHandler::new(prev_guard, wrong_handler));

        self
    }

    fn create_guards_service(&mut self) {
        self.add_last_to_guards();

        if !self.guards.is_empty() {
            let mut guards = Guards::new();
            std::mem::swap(&mut guards, &mut self.guards);
            self.demux.add_service(GuardsHandler::new(guards));
        }
    }

    fn add_last_to_guards(&mut self) {
        let prev = self.last_guard.take();
        if let Some(prev) = prev {
            self.guards.add_boxed_guard(prev);
        }
    }
}
