use crate::{
    dispatching::{
        core::{Parser, RecombineFrom},
        handlers::{
            messages::message_parser::{parser, MessageParser},
            parser::UpdateParser,
        },
        updates::UpdateRest,
    },
    types::{Message, Update},
};

impl<ParserT, Err> UpdateParser<Update, Message, UpdateRest, Err, ParserT>
where
    ParserT: Parser<Update, Message, UpdateRest>,
    Update: RecombineFrom<ParserT, Message, UpdateRest>,
{
    pub fn common(self) -> MessageParser<ParserT, parser::Common, Err> {
        MessageParser::new(self.into_inner(), parser::Common)
    }

    pub fn new_chat_members(self) -> MessageParser<ParserT, parser::NewChatMembers, Err> {
        MessageParser::new(self.into_inner(), parser::NewChatMembers)
    }

    pub fn left_chat_member(self) -> MessageParser<ParserT, parser::LeftChatMember, Err> {
        MessageParser::new(self.into_inner(), parser::LeftChatMember)
    }

    pub fn new_chat_title(self) -> MessageParser<ParserT, parser::NewChatTitle, Err> {
        MessageParser::new(self.into_inner(), parser::NewChatTitle)
    }

    pub fn new_chat_photo(self) -> MessageParser<ParserT, parser::NewChatPhoto, Err> {
        MessageParser::new(self.into_inner(), parser::NewChatPhoto)
    }

    pub fn delete_chat_photo(self) -> MessageParser<ParserT, parser::DeleteChatPhoto, Err> {
        MessageParser::new(self.into_inner(), parser::DeleteChatPhoto)
    }

    pub fn group_chat_created(self) -> MessageParser<ParserT, parser::GroupChatCreated, Err> {
        MessageParser::new(self.into_inner(), parser::GroupChatCreated)
    }

    pub fn supergroup_chat_created(
        self,
    ) -> MessageParser<ParserT, parser::SupergroupChatCreated, Err> {
        MessageParser::new(self.into_inner(), parser::SupergroupChatCreated)
    }

    pub fn channel_chat_created(self) -> MessageParser<ParserT, parser::ChannelChatCreated, Err> {
        MessageParser::new(self.into_inner(), parser::ChannelChatCreated)
    }

    pub fn migrate(self) -> MessageParser<ParserT, parser::Migrate, Err> {
        MessageParser::new(self.into_inner(), parser::Migrate)
    }

    pub fn pinned(self) -> MessageParser<ParserT, parser::Pinned, Err> {
        MessageParser::new(self.into_inner(), parser::Pinned)
    }

    pub fn invoice(self) -> MessageParser<ParserT, parser::Invoice, Err> {
        MessageParser::new(self.into_inner(), parser::Invoice)
    }

    pub fn successful_payment(self) -> MessageParser<ParserT, parser::SuccessfulPayment, Err> {
        MessageParser::new(self.into_inner(), parser::SuccessfulPayment)
    }

    pub fn connected_website(self) -> MessageParser<ParserT, parser::ConnectedWebsite, Err> {
        MessageParser::new(self.into_inner(), parser::ConnectedWebsite)
    }

    pub fn passport_data(self) -> MessageParser<ParserT, parser::PassportData, Err> {
        MessageParser::new(self.into_inner(), parser::PassportData)
    }

    pub fn dice(self) -> MessageParser<ParserT, parser::Dice, Err> {
        MessageParser::new(self.into_inner(), parser::Dice)
    }
}
