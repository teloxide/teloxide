use crate::dispatching::handlers::common::UpdateKindHandlerBuilder;
use crate::types::Message;

pub type MessageHandlerBuilder<Parser, Err> = UpdateKindHandlerBuilder<Message, Parser, Err>;
