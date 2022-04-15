use dptree::{di::DependencyMap, Handler};

use crate::dispatching::DpHandlerDescription;

/// Something that can construct a handler.
#[deprecated(note = "Use the teloxide::handler! API")]
pub trait HandlerFactory {
    type Out;

    fn handler() -> Handler<'static, DependencyMap, Self::Out, DpHandlerDescription>;
}
