use crate::{
    dispatching::{
        core::{Parser, ParserOut},
        handlers::common::UpdateKindHandlerBuilder,
    },
    types::Message,
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

pub type MessageHandlerBuilder<Parser, Err> = UpdateKindHandlerBuilder<Message, Parser, Err>;
