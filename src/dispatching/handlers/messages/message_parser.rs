use crate::{
    dispatching::{
        core::{
            DemuxBuilder, Guard, Guards, Handler, IntoGuard, IntoHandler, MapParser, OrGuard,
            Parser, ParserOut, RecombineFrom,
        },
        dispatcher_context::DispatcherContext,
        handlers::messages::{
            message_handler::MessageHandler,
        },
        updates::UpdateRest,
    },
    types::{Message, Update},
};
use crate::dispatching::handlers::update_kind_handler_builder::UpdateKindHandlerBuilder;

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
impl<Parser1, Parser2>
    RecombineFrom<
        MapParser<Parser1, Parser2, Message, UpdateRest, (), Message>,
        Message,
        (UpdateRest, ()),
    > for Update
where
    Update: RecombineFrom<Parser1, Message, UpdateRest>,
{
    fn recombine(info: ParserOut<Message, (UpdateRest, ())>) -> Self {
        let (out, (rest1, _)) = info.into_inner();
        <Update as RecombineFrom<Parser1, Message, UpdateRest>>::recombine(ParserOut::new(
            out, rest1,
        ))
    }
}

pub type MessageHandlerBuilder<Parser, Err> = UpdateKindHandlerBuilder<Message, Parser, Err>;