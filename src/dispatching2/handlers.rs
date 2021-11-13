use crate::{
    types,
    types::{Update, UpdateKind},
};
use dptree::{di::DependencySupplier, Handler, Replace};

pub type Replaced<C, T2> = <C as Replace<Update, T2>>::Out;

pub struct Handlers<C, Err>
where
    C: Replace<Update, types::Message>,
{
    message_handler: UpdateHandler<C, Err, Replaced<C, types::Message>>,
    edited_message_handler: UpdateHandler<C, Err, Replaced<C, types::Message>>,
}

macro_rules! new_handler {
    ($kind:ident) => {
        dptree::parser(|upd: &Update| match &upd.kind {
            UpdateKind::$kind(u) => Some(u.clone()),
            _ => None,
        })
    };
}

impl<C, IR, Err> Handlers<C, Err>
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
        Handlers {
            message_handler: new_handler!(Message),
            edited_message_handler: new_handler!(EditedMessage),
        }
    }

    pub fn message_handler(&mut self, handler: UpdateHandler<Replaced<C, types::Message>, Err>) {
        self.message_handler = self.message_handler.clone().branch(handler);
    }

    pub fn edited_message_handler(
        &mut self,
        handler: UpdateHandler<Replaced<C, types::Message>, Err>,
    ) {
        self.edited_message_handler = self.edited_message_handler.clone().branch(handler);
    }
}

// TODO: it is allowed to return message as answer on telegram request in
// webhooks, so we can allow this too. See more there: https://core.telegram.org/bots/api#making-requests-when-getting-updates
pub type UpdateHandler<C, Err, IR = C> = Handler<'static, C, Result<(), Err>, IR>;
