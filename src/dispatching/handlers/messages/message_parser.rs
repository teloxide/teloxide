use crate::dispatching::handlers::common::UpdateKindHandlerBuilder;
use crate::types::Message;

pub type MessageHandlerBuilder<Ctx, Parser, Err> = UpdateKindHandlerBuilder<Message, Ctx, Parser, Err>;
