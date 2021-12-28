use dptree::di::DependencyMap;
use dptree::Handler;

pub trait HandlerFactory {
    type Out;

    fn handler() -> Handler<'static, DependencyMap, Self::Out>;
}