//! The module contains all the traits needed to work with dispatchers contexts.

use crate::dispatching::{
    core::{ParserOut, RecombineFrom},
    dispatcher_context::DispatcherContext,
};

/// The trait is used to recognize which update store the `Context`. It is
/// implemented for [DispatcherContext] and [DialogueContext]. If you create
/// your own dispatcher with your own context, you must implement the trait for
/// it.
///
/// [DispatcherContext]: TODO
/// [DialogueContext]: TODO
pub trait Context {
    type Upd;

    fn get_upd(&self) -> &Self::Upd;
}

/// The trait is used to define the Self with another update as an element. It
/// is the hack to simulate the `GAT`. It is implemented for [DispatcherContext]
/// and [DialogueContext]. If you create your own dispatcher with your own
/// context, you must implement the trait for it.
///
/// [DispatcherContext]: TODO
/// [DialogueContext]: TODO
pub trait ContextWith<Elem>: Context {
    type Context: Context<Upd = Elem>;
}

/// The trait is used to parse a context with `Upd1` to context with `Upd2`. It
/// is implemented for [DispatcherContext] and [DialogueContext]. If you create
/// your own dispatcher with your own context, you must implement the trait for
/// it.
///
/// [DispatcherContext]: TODO
/// [DialogueContext]: TODO
pub trait ParseContext<To>: ContextWith<To> + Sized {
    fn parse<Rest>(
        self,
        f: impl Fn(Self::Upd) -> Result<ParserOut<To, Rest>, Self::Upd>,
    ) -> Result<(Self::Context, Rest), Self>;
    fn recombine<Parser, Rest>(info: ParserOut<Self::Context, Rest>) -> Self
    where
        Self::Upd: RecombineFrom<Parser, To, Rest>;
}

/// The trait is used to get the data from the context and pass it to the
/// handler.
///
/// All types thet implement the trait can be passed to the handler function.
/// E.g. if you have your own `Typ` thet implement `FromContext` and you point
/// it in the handler function signature, the `from_context` function will be
/// called. If it return `None` than handling will be stop and update pass to
/// the other handler functions.
///
/// Usually you want to define `Ctx` generic and use `GetCtx` trait with needed
/// context as first type argument. This is need to get your type from the
/// different kinds of contexts from different dispatchers.
///
/// Example:
/// ```
/// use teloxide::{
///     dispatching::{dev::*, updates, Dispatcher, DispatcherBuilder},
///     types::*,
///     Bot,
/// };
///
/// struct ChatId(i64);
/// impl<Ctx> FromContext<Ctx> for ChatId
/// where
///     Ctx: Context<Upd = Update>,
/// {
///     fn from_context(context: &Ctx) -> Option<Self> {
///         let upd = context.get_upd();
///         match &upd.kind {
///             UpdateKind::Message(m) => Some(ChatId(m.chat_id())),
///             _ => None,
///         }
///     }
/// }
///
/// let dispatcher = DispatcherBuilder::new(Bot::new(""), "bot_name")
///     .handle(updates::any().by(|_: Update, chat_id: ChatId| {
///         assert_ne!(chat_id.0, 0);
///     }))
///     .error_handler(|_| async { unreachable!() })
///     .build();
/// ```
pub trait FromContext<Ctx>: Sized {
    fn from_context(context: &Ctx) -> Option<Self>;
}

/// The trait is used to get the data from the context and pass it to the
/// handler by ownership.
///
/// It calls after `FromContext` trait before pass to the handler.
/// `FromContextOwn` argument must be first in the handler.
///
/// `RequireCtx` - the Ctx which you want to get from the `Ctx`. It is needed
/// only if your `RequireCtx` contains generic arguments without depends from
/// the `Self`.
pub trait FromContextOwn<Ctx, RequireCtx = Ctx>: Sized {
    fn from_context(context: Ctx) -> Self;
}

impl<Upd> FromContextOwn<DispatcherContext<Upd>> for Upd {
    fn from_context(context: DispatcherContext<Upd>) -> Self {
        context.upd
    }
}

/// The trait is used to get the required context from the generic `Ctx`
/// parameter.
///
/// It is used to implement [`FromContext`] and [`FromContextOwn`] for different
/// kinds of `Context`.
///
/// [`FromContext`]: TODO
/// [`FromContextOwn`]: TODO
pub trait GetCtx<Ctx> {
    fn get(&self) -> &Ctx;
    fn get_own(self) -> Ctx;
}
