//! Update dispatching.

pub mod filter;
pub mod handler;
pub mod simple;
pub mod updater;

pub use filter::Filter;
pub use handler::Handler;

use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Dispatcher<'a, U> {
    async fn dispatch(&'a mut self, updater: U);
}
