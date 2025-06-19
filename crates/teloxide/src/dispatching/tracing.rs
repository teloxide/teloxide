use super::UpdateHandler;

use dptree::{
    di::{Asyncify, Injectable},
    prelude::DependencyMap,
    HandlerSignature,
};
use std::{collections::BTreeSet, sync::Arc};
use tracing::{Instrument, Span};

pub trait UpdateHandlerTracingExt<E> {
    /// Returns an `UpdateHandler` wrapped in an async span.
    fn instrument_with_async<F, FnArgs>(self, f: F) -> Self
    where
        F: Injectable<Span, FnArgs> + Send + Sync + 'static;

    /// Returns an `UpdateHandler` wrapped in a span.
    fn instrument_with<F, FnArgs>(self, f: F) -> Self
    where
        Asyncify<F>: Injectable<Span, FnArgs> + Send + Sync + 'static;
}

impl<E: 'static> UpdateHandlerTracingExt<E> for UpdateHandler<E> {
    fn instrument_with_async<F, FnArgs>(self, f: F) -> UpdateHandler<E>
    where
        F: Injectable<Span, FnArgs> + Send + Sync + 'static,
    {
        let f = Arc::new(f);

        dptree::from_fn_with_description(
            self.description().clone(),
            move |deps: DependencyMap, cont| {
                let self_c = self.clone();
                let f = Arc::clone(&f);

                async move {
                    let f = f.inject(&deps);
                    let span = f().await;
                    drop(f);

                    self_c.execute(deps, cont).instrument(span).await
                }
            },
            HandlerSignature::Other {
                obligations: F::obligations(),
                guaranteed_outcomes: BTreeSet::new(),
                conditional_outcomes: BTreeSet::new(),
                continues: true,
            },
        )
    }

    fn instrument_with<F, FnArgs>(self, f: F) -> Self
    where
        Asyncify<F>: Injectable<Span, FnArgs> + Send + Sync + 'static,
    {
        self.instrument_with_async(Asyncify(f))
    }
}
