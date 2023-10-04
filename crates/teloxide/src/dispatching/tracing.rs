use super::{DpHandlerDescription, UpdateHandler};

use dptree::{
    di::{Asyncify, Injectable},
    prelude::DependencyMap,
    HandlerDescription,
};
use std::sync::Arc;
use tracing::{Instrument, Span};

pub trait UpdateHandlerTracingExt<E> {
    /// Returns an `UpdateHandler` wrapped in an async span.
    fn instrument_with_async<F, FnArgs>(self, f: F) -> Self
    where
        F: Injectable<DependencyMap, Span, FnArgs> + Send + Sync + 'static;

    /// Returns an `UpdateHandler` wrapped in a span.
    fn instrument_with<F, FnArgs>(self, f: F) -> Self
    where
        Asyncify<F>: Injectable<DependencyMap, Span, FnArgs> + Send + Sync + 'static;
}

impl<E: 'static> UpdateHandlerTracingExt<E> for UpdateHandler<E> {
    fn instrument_with_async<F, FnArgs>(self, f: F) -> UpdateHandler<E>
    where
        F: Injectable<DependencyMap, Span, FnArgs> + Send + Sync + 'static,
    {
        // FIXME: This is a hacky replacement for `handler.description().clone()`.
        // Ideally cloning `DpHandlerDescription` would be supported by `teloxide`.
        let description = DpHandlerDescription::entry().merge_chain(self.description());

        let f = Arc::new(f);

        dptree::from_fn_with_description(description, move |deps: DependencyMap, cont| {
            let self_c = self.clone();
            let f = f.clone();

            async move {
                let f = f.inject(&deps);
                let span = f().await;
                drop(f);

                self_c.execute(deps, cont).instrument(span).await
            }
        })
    }

    fn instrument_with<F, FnArgs>(self, f: F) -> Self
    where
        Asyncify<F>: Injectable<DependencyMap, Span, FnArgs> + Send + Sync + 'static,
    {
        self.instrument_with_async(Asyncify(f))
    }
}
