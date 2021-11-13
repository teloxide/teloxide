use crate::{
    dispatching2::handlers::{Handlers, Replaced, UpdateHandler},
    types,
    types::Update,
};
use dptree::{di::DependencySupplier, Replace};

pub struct Dispatcher<C, Err>
where
    C: Replace<Update, types::Message>,
{
    handlers: Handlers<C, Err>,
}

impl<C, IR, Err> Dispatcher<C, Err>
where
    C: DependencySupplier<Update>
        + Send
        + Sync
        + 'static
        + Replace<Update, types::Message, Out = IR>,
    IR: Send + Sync + 'static + Replace<types::Message, Update, Out = C>,
    Err: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Dispatcher { handlers: Handlers::new() }
    }

    pub fn message_handler(
        mut self,
        handler: UpdateHandler<Replaced<C, types::Message>, Err>,
    ) -> Self {
        self.handlers.message_handler(handler);
        self
    }

    pub fn edited_message_handler(
        mut self,
        handler: UpdateHandler<Replaced<C, types::Message>, Err>,
    ) -> Self {
        self.handlers.edited_message_handler(handler);
        self
    }
}
