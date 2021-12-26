use crate::{types::Message, utils::command::BotCommand};
use dptree::{
    di::{DependencySupplier, Insert},
    Handler,
};

pub trait HandlerExt<IR> {
    fn add_command<C>(self, bot_name: String) -> Self
    where
        C: BotCommand + Send,
        IR: Insert<C>;
}

impl<Input, Output, IR> HandlerExt<IR> for Handler<'_, Input, Output, IR>
where
    Input: Send + Sync + 'static,
    Output: Send + Sync + 'static,
    IR: Send + Sync + 'static + Clone + DependencySupplier<Message>,
{
    fn add_command<C>(self, bot_name: String) -> Self
    where
        C: BotCommand + Send,
        IR: Insert<C>,
    {
        self.chain(dptree::filter_map(move |message: Message| {
            let bot_name = bot_name.clone();
            async move { message.text().and_then(|text| C::parse(text, bot_name).ok()) }
        }))
    }
}
