//! Tidy dispatching for the incoming updates.
//!
//! Dispatching in teloxide is a distribution of incoming updates got by polling or using webhooks
//! into user-defined handler functions. The way of distribution base on matching and guards.
//!
//! Guard is a function that give reference to the object and returns bool value. The simplest kind
//! of guards is arithmetic operations `==`, `>=`, `>`, etc. It decides: is there are need to handle
//! the incoming update or not.
//!
//! Dispatching completes using this algorithm:
//! 1. Update incomes.
//! 2. Dispatches feeds the update to a first handler in a list.
//! 3. Handler call the guards on the update. If it all returns the `true` then handler will call function that you
//! pass to a `by` function.
//! 4. If the handler return an error, error will be passed to the error handler which you define in the
//! [`DispatcherBuilder::error_handler`] method using [`DispatchError::NoHandler`] variant. If you do not define this
//! method explicitly, error will be ignored.
//! 5. If at least one guard returns `false` then update will be returned to the dispatcher, and dispatcher tries to
//! feeds the update to the next handler in a list.
//! 6. If all handlers does not handle the update, it will be passed to the error handler using
//! [`DispatchError::HandlerError`] variant.
//!
//! The key abstraction here is [`Dispatcher`]. It contains global information and methods for handling the updates.
//! It can be used in other dispatchers, like in [`DialogueDispatcher`] which provide support for the dialogues.
//!
//! First, you must create the builder using [`DispatcherBuilder::new`] method and pass to it bot struct and bot name. It is
//! default data needed which you can after using in your handlers.
//!
//! Than you can add some global data which you want to pass to the many handlers: it can be configs, database pool of
//! connectors, or some kind of services. It adds using [`DispatcherBuilder::data`] method. In handlers you can
//! get access to this data using [`tel::Data`] struct.
//!
//! The next stage is define the handlers. Its defines using [`DispatcherBuilder::handle`] method. Due to internal
//! architecture, functions does not implement [`Handler`] trait, so you must choose one of predefined functions
//! in the [`handlers::update`] module to build the dispatcher or create your own struct.
//!
//! In the [`handlers::update`] module you can find functions that uses to handle some kind of updates: `Message`,
//! `InlineQuery`, `CallbackQuery`, etc. These handlers will handle only specified kind of update. These methods
//! returns a builders. The builders have a methods `with_guard`, `or_with_guard` and `or_else` which you can use
//! to define flow of the dispatching. Most of the builders have predefined guard methods most of which
//! starts with `with_*`, `or_with_*`, `has_*`, `or_has_*`.
//!
//! After you must specify an error handler. You can use one of [`predefined error handlers`]  like
//! [`LoggingErrorHandler`] or create your own.
//!
//! At the end you must call [`DispatcherBuilder::build`] which create the [`Dispatcher`] struct.
//!
//! In common cases you just want to start the dispatching using [`Dispatcher::dispatch_with_listener`]
//! method and pass to it [`polling_default`] or webhooks (there are no webhook impl in the library,
//! but you can see an example in the repository).
//!
//! Example:
//! ```no_compile
//! use teloxide::dispatching::*;
//! use teloxide::prelude::*;
//! use teloxide::types;
//! use teloxide::dispatching::error_handlers::LoggingErrorHandler;
//!
//! let admin_id = 202040960;
//! let bot = Bot::new("token");
//! let bot_name = "bot_name";
//!
//! let dispatcher = DispatcherBuilder::new(bot.clone(), bot_name)
//!     .handle(
//!         updates::message()
//!             .with_from(|user: &types::User| user.id == admin_id)
//!             .by(|cx: UpdateWithCx<types::Message>| async move {
//!                 cx.answer_str("You are admin").await?;
//!             })
//!     )
//!     .handle(
//!         updates::message()
//!             .with_chat(|chat: &types::Chat| matches!(chat.kind, types::ChatKind::Public(_)))
//!             .or_else(|cx: UpdateWithCx<types::Message>| async move {
//!                 cx.answer_str("Bot works only in the public chats!").await?;
//!             })
//!             .by(|cx: UpdateWithCx<types::Message>| async move {
//!                 cx.answer_str("I'm work!").await?;
//!             })
//!     )
//!     .error_handler(LoggingErrorHandler::with_custom_text("An error from the dispatcher"))
//!     .build();
//!
//! dispatcher.dispatch_with_listener(polling(bot), &LoggingErrorHandler::with_custom_text("An error from the listener")).await;
//!
//! ```
//!
//! ## Handlers methods
//!
//! `with_guard` function is used to define a custom guard with signature `Fn(&Upd) -> Out` where `Out`
//! can be both `bool` and `impl Future<Output = bool>`.
//!
//! ```
//! use teloxide::dispatching::*;
//! use teloxide::prelude::*;
//! use teloxide::types;
//!
//! let handler = updates::message()
//!         .with_guard(|mes: &Message| mes.via_bot.is_none())
//!         // Some builders have `with_*` function that allows you to avoid excess `match`.
//!         // These guards will be called only if update has that field. Guard below will be called
//!         // only when `Message::via_bot` field is `Some`. If it `None`, guard return false.
//!         .with_via_bot(|via: &str| via == "some_bot")
//!         // Same as above but add `or` rule between this and previous guard. So, guard return `true`
//!         // if `with_via_bot` return true OR `or_with_document` return true.
//!         .or_with_document(|doc: &types::Document| doc.file_size.unwrap() < 20)
//!         // Some builders have `has_*` function that returns `true` when that field is `Some`,
//!         // and false otherwise.
//!         .has_audio()
//!         // Same as above but add `or` rule between this and previous guard. So, guard return `true`
//!         // if `has_audio` return true OR `or_has_animation` return true.
//!         .or_has_animation()
//!         // Returns `true` when that field is `None`, and true otherwise.
//!         .no_has_audio()
//!         .or_no_has_audio()
//!         .by(|cx: UpdateWithCx<types::Message>| async move {
//!             cx.answer_str("I'm work!").await?;
//!         });
//! ```
//!
//! `or_else` applies to the previously added guard. If it returns `false`, than handler passed to the `or_else`
//! will be called, and handling was stopped.
//!
//! You can define extensions methods and reuse these 3 kind of methods. There are example in the
//! [`DialogueHandlerBuilderExt`].
//!
//! [`DispatcherBuilder::error_handler`]: crate::dispatching::DispatcherBuilder::error_handler
//! [`DispatchError::NoHandler`]: crate::dispatching::DispatchError::NoHandler
//! [`DispatchError::HandlerError`]: crate::dispatching::DispatchError::HandlerError
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
//! [`DispatcherBuilder::new`]: crate::dispatching::DispatcherBuilder::new
//! [`DispatcherBuilder::data`]: crate::dispatching::DispatcherBuilder::data
//! [`tel::Data`]: crate::dispatching::tel::Data
//! [`Handler`]: crate::dispatching::dev::Handler
//! [`handlers::update`]: crate::dispatching::handlers::update
//! [`predefined error handlers`]: crate::dispatching::error_handlers
//! [`LoggingErrorHandler`]: crate::dispatching::error_handlers::LoggingErrorHandler
//! [`DispatcherBuilder::build`]: crate::dispatching::DispatcherBuilder::build
//! [`Dispatcher::dispatch_with_listener`]: crate::dispatching::Dispatcher::dispatch_with_listener
//! [`polling_default`]: crate::dispatching::update_listeners::polling_default
//! [`DialogueHandlerBuilderExt`]: crate::dispatching::dialogue::DialogueHandlerBuilderExt

pub(crate) mod core;
pub mod dialogue;
mod dispatcher;
mod dispatcher_context;
pub mod error_handlers;
pub mod handlers;
pub(crate) mod repls;
#[cfg(test)]
mod tests;
pub mod update_listeners;
pub mod update_with_cx;

pub use dispatcher::{Dispatcher, DispatcherBuilder};
pub use handlers::updates;
pub use update_with_cx::UpdateWithCx;
pub use dev::DispatchError;

pub mod dev {
    pub use super::core::*;

    pub use super::dispatcher_context::DispatcherContext;
}

pub mod tel {
    pub use super::handlers::commands::Command;
    use crate::dispatching::{
        core::{Context, FromContext, GetCtx},
        dispatcher_context::DispatcherContext,
    };
    use std::{ops::Deref, sync::Arc};

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
        Ctx: Context<Upd = Upd> + GetCtx<DispatcherContext<Upd>>,
    {
        fn from_context(context: &Ctx) -> Option<Self> {
            let t = context.get().global_data.get::<T>();
            match t {
                Some(data) => Some(Data(data.clone())),
                None => {
                    log::warn!(
                        "There are no {} dependency in global data!",
                        std::any::type_name::<T>()
                    );
                    None
                }
            }
        }
    }
}
