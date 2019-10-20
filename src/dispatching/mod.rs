//! Update dispatching.

use async_trait::async_trait;
pub use filter::Filter;
pub use handler::Handler;

pub mod filters;
pub mod handler;
pub mod filters;
pub mod updater;

#[async_trait(? Send)]
pub trait Dispatcher<'a, U> {
    async fn dispatch(&'a mut self, updater: U);
}
