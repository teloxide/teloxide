use std::{
    fmt,
    future::Future,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};

use tokio::sync::Notify;

/// A token which used to shutdown [`Dispatcher`].
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[derive(Clone)]
pub struct ShutdownToken {
    // FIXME: use a single arc
    dispatcher_state: Arc<DispatcherState>,
    shutdown_notify_back: Arc<Notify>,
}

/// This error is returned from [`ShutdownToken::shutdown`] when trying to
/// shutdown an idle [`Dispatcher`].
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[derive(Debug)]
pub struct IdleShutdownError;

impl ShutdownToken {
    /// Tries to shutdown dispatching.
    ///
    /// Returns an error if the dispatcher is idle at the moment.
    ///
    /// If you don't need to wait for shutdown, the returned future can be
    /// ignored.
    pub fn shutdown(&self) -> Result<impl Future<Output = ()> + '_, IdleShutdownError> {
        match shutdown_inner(&self.dispatcher_state) {
            Ok(()) | Err(Ok(AlreadyShuttingDown)) => Ok(async move {
                log::info!("Trying to shutdown the dispatcher...");
                self.shutdown_notify_back.notified().await
            }),
            Err(Err(err)) => Err(err),
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            dispatcher_state: Arc::new(DispatcherState {
                inner: AtomicU8::new(ShutdownState::Idle as _),
                notify: <_>::default(),
            }),
            shutdown_notify_back: <_>::default(),
        }
    }

    pub(crate) async fn wait_for_changes(&self) {
        self.dispatcher_state.notify.notified().await;
    }

    pub(crate) fn start_dispatching(&self) {
        if let Err(actual) =
            self.dispatcher_state.compare_exchange(ShutdownState::Idle, ShutdownState::Running)
        {
            panic!(
                "Dispatching is already running: expected `{:?}` state, found `{:?}`",
                ShutdownState::Idle,
                actual
            );
        }
    }

    pub(crate) fn is_shutting_down(&self) -> bool {
        matches!(self.dispatcher_state.load(), ShutdownState::ShuttingDown)
    }

    pub(crate) fn done(&self) {
        if self.is_shutting_down() {
            // Stopped because of a `shutdown` call.

            // Notify `shutdown`s that we finished
            self.shutdown_notify_back.notify_waiters();
            log::info!("Dispatching has been shut down.");
        } else {
            log::info!("Dispatching has been stopped (listener returned `None`).");
        }

        self.dispatcher_state.store(ShutdownState::Idle);
    }
}

impl fmt::Display for IdleShutdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dispatcher was idle and as such couldn't be shut down")
    }
}

impl std::error::Error for IdleShutdownError {}

struct DispatcherState {
    inner: AtomicU8,
    notify: Notify,
}

impl DispatcherState {
    // Ordering::Relaxed: only one atomic variable, nothing to synchronize.
    fn load(&self) -> ShutdownState {
        ShutdownState::from_u8(self.inner.load(Ordering::Relaxed))
    }

    fn store(&self, new: ShutdownState) {
        self.inner.store(new as _, Ordering::Relaxed);
        self.notify.notify_waiters();
    }

    fn compare_exchange(
        &self,
        current: ShutdownState,
        new: ShutdownState,
    ) -> Result<ShutdownState, ShutdownState> {
        self.inner
            .compare_exchange(current as _, new as _, Ordering::Relaxed, Ordering::Relaxed)
            .map(ShutdownState::from_u8)
            .map_err(ShutdownState::from_u8)
            // FIXME: `Result::inspect` when :(
            .inspect(|_| self.notify.notify_waiters())
    }
}

#[repr(u8)]
#[derive(Debug)]
enum ShutdownState {
    Running,
    ShuttingDown,
    Idle,
}

impl ShutdownState {
    fn from_u8(n: u8) -> Self {
        const RUNNING: u8 = ShutdownState::Running as u8;
        const SHUTTING_DOWN: u8 = ShutdownState::ShuttingDown as u8;
        const IDLE: u8 = ShutdownState::Idle as u8;

        match n {
            RUNNING => ShutdownState::Running,
            SHUTTING_DOWN => ShutdownState::ShuttingDown,
            IDLE => ShutdownState::Idle,
            _ => unreachable!(),
        }
    }
}

struct AlreadyShuttingDown;

fn shutdown_inner(
    state: &DispatcherState,
) -> Result<(), Result<AlreadyShuttingDown, IdleShutdownError>> {
    use ShutdownState::*;

    let res = state.compare_exchange(Running, ShuttingDown);

    match res {
        Ok(_) => Ok(()),
        Err(ShuttingDown) => Err(Ok(AlreadyShuttingDown)),
        Err(Idle) => Err(Err(IdleShutdownError)),
        Err(Running) => unreachable!(),
    }
}
