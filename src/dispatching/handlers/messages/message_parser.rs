use crate::{dispatching::handlers::common::UpdateKindHandlerBuilder, types::Message};

pub type MessageHandlerBuilder<Ctx, Parser, Err> =
    UpdateKindHandlerBuilder<Message, Ctx, Parser, Err>;
