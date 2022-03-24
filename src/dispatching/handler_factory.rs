use dptree::{di::DependencyMap, Handler};

/// Something that can construct a handler.
pub trait HandlerFactory {
    type Out;

    fn handler() -> Handler<'static, DependencyMap, Self::Out>;
}
