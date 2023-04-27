use super::{DpHandlerDescription, UpdateHandler};

use dptree::{prelude::DependencyMap, HandlerDescription};
use tracing::Instrument;

/// Extension methods for adding `tracing` to [`UpdateHandler`].
///
/// This trait is only available if the `tracing` feature is enabled.
pub trait UpdateHandlerTracingExt<E> {
    /// Returns an `UpdateHandler` with tracing enabled.
    fn with_tracing_span<F>(self, f: F) -> Self
    where
        F: Fn(DependencyMap) -> tracing::Span + Send + Sync + 'static;
}

impl<E: 'static> UpdateHandlerTracingExt<E> for UpdateHandler<E> {
    fn with_tracing_span<F>(self, f: F) -> Self
    where
        F: Fn(DependencyMap) -> tracing::Span + Send + Sync + 'static,
    {
        // FIXME: This is a hacky replacement for `handler.description().clone()`.
        // Ideally cloning `DpHandlerDescription` would be supported by `teloxide`.
        let description = DpHandlerDescription::entry().merge_chain(self.description());

        dptree::from_fn_with_description(description, move |deps: DependencyMap, cont| {
            self.clone().execute(deps.clone(), cont).instrument(f(deps))
        })
    }
}
