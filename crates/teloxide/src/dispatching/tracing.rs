use super::{DpHandlerDescription, UpdateHandler};

use dptree::{di::Injectable, prelude::DependencyMap, HandlerDescription};
use tracing::{Instrument, Span};

pub trait UpdateHandlerTracingExt<E> {
    /// Returns an `UpdateHandler` with tracing enabled.
    fn with_tracing_span<F, FnArgs>(self, f: F) -> Self
    where
        F: Injectable<DependencyMap, Span, FnArgs> + Send + Sync + Clone + 'static;
}

impl<E: 'static> UpdateHandlerTracingExt<E> for UpdateHandler<E> {
    fn with_tracing_span<F, FnArgs>(self, f: F) -> UpdateHandler<E>
    where
        F: Injectable<DependencyMap, Span, FnArgs> + Send + Sync + Clone + 'static,
    {
        // FIXME: This is a hacky replacement for `handler.description().clone()`.
        // Ideally cloning `DpHandlerDescription` would be supported by `teloxide`.
        let description = DpHandlerDescription::entry().merge_chain(self.description());

        dptree::from_fn_with_description(description, move |deps: DependencyMap, cont| {
            let self_c = self.clone();
            let f_c = f.clone();
            async move {
                let span = f_c.inject(&deps)().await;
                self_c.execute(deps, cont).instrument(span).await
            }
        })
    }
}
