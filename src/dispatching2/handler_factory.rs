use dptree::{di::DependencyMap, Handler};

pub trait HandlerFactory {
    type Out;

    fn handler() -> Handler<'static, DependencyMap, Self::Out>;
}
