pub(crate) mod core;
pub mod dialogue;
mod dispatcher;
mod dispatcher_context;
pub mod error_handlers;
mod handlers;
pub(crate) mod repls;
#[cfg(test)]
mod tests;
mod update_listeners;
pub mod update_with_cx;

pub use dispatcher::{Dispatcher, DispatcherBuilder};
pub use handlers::updates;
pub use update_with_cx::UpdateWithCx;

pub mod dev {
    pub use super::core::*;

    pub use super::dispatcher_context::DispatcherContext;
}

pub mod tel {
    pub use super::handlers::commands::Command;
    use crate::dispatching::core::{FromContext, GetCtx, Context};
    use crate::dispatching::dispatcher_context::DispatcherContext;
    use std::sync::Arc;
    use std::ops::Deref;

    #[derive(Debug, PartialEq)]
    pub struct Data<T>(pub Arc<T>);

    impl<T> Deref for Data<T> {
        type Target = Arc<T>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<Upd, Ctx, T> FromContext<Ctx> for Data<T>
    where
        T: Send + Sync + 'static,
        Ctx: Context<Upd = Upd> + GetCtx<DispatcherContext<Upd>>
    {
        fn from_context(context: &Ctx) -> Option<Self> {
            let t = context.get().global_data.get::<T>();
            match t {
                Some(data) => Some(Data(data.clone())),
                None => {
                    log::warn!("There are no {} dependency in global data!", std::any::type_name::<T>());
                    None
                }
            }
        }
    }
}
