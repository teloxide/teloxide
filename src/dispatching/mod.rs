//! Update dispatching.

use async_trait::async_trait;
pub use filters::Filter;
pub use handler::Handler;

pub mod dispatchers;
pub mod filters;
pub mod handler;
pub mod updater;

#[async_trait(? Send)]
pub trait Dispatcher<'a, U> {
    async fn dispatch(&'a mut self, updater: U);
}
