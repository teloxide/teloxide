//! The module for the `Demux` and `DemuxBuilder` structs.
//!
//! The `Demux` struct try to handle the input update by the vector of handlers
//! and return the update if no handler had handle it.
//!
//! The `DemuxBuilder` struct is the builder for the `Demux`.
//!
//! Usage:
//! ```
//! use std::convert::Infallible;
//! use teloxide::{
//!     dispatching::{dev::*, updates},
//!     types::Update,
//! };
//!
//! let mut builder = DemuxBuilder::<Update, Infallible>::new();
//! builder.add_service(updates::any());
//! let demux = builder.build();
//! ```

use crate::dispatching::core::{HandleFuture, Handler};
use std::sync::Arc;

/// The `Demux` struct try to handle the input update by the vector of handlers
/// and return the update if no handler had handle it.
pub struct Demux<Upd, Err> {
    handlers: Arc<[Box<dyn Handler<Upd, Err> + Send + Sync>]>,
}

/// The builder for the [`Demux`] struct.
pub struct DemuxBuilder<Upd, Err> {
    handlers: Vec<Box<dyn Handler<Upd, Err> + Send + Sync>>,
}

impl<Upd, Err> DemuxBuilder<Upd, Err> {
    /// Create the builder.
    pub fn new() -> Self {
        DemuxBuilder { handlers: Vec::new() }
    }

    /// Add the handler. [`Demux`] has no methods for adding handlers.
    ///
    /// [`Demux`]: crate::dispatching::dev::Demux
    pub fn add_service(&mut self, service: impl Handler<Upd, Err> + Send + Sync + 'static) {
        self.handlers.push(Box::new(service) as _);
    }

    /// Build the [`Demux`].
    ///
    /// [`Demux`]: crate::dispatching::dev::Demux
    pub fn build(self) -> Demux<Upd, Err> {
        Demux { handlers: self.handlers.into() }
    }
}

impl<Upd: Send + 'static, Err: 'static> Handler<Upd, Err> for Demux<Upd, Err> {
    fn handle(&self, update: Upd) -> HandleFuture<Err, Upd> {
        let handlers = self.handlers.clone();
        Box::pin(async move {
            let mut update = update;
            for handler in handlers.iter() {
                match handler.handle(update).await {
                    Ok(res) => return Ok(res),
                    Err(upd) => {
                        update = upd;
                        continue;
                    }
                }
            }
            Err(update)
        })
    }
}
