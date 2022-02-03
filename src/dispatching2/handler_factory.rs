use dptree::{di::DependencyMap, Handler};

/// Something that can construct a handler (for internal usage only).
pub trait HandlerFactory {
    type Out;

    fn handler() -> Handler<'static, DependencyMap, Self::Out>;
}
