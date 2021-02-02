use std::convert::Infallible;

/// The result of the handling by the user function.
///
/// We use our enum isntead of `std::result::Result` due to restrictions of the
/// orphan rules.
pub enum HandleResult<Err> {
    Ok,
    Err(Err),
}

impl<Error> From<Result<(), Error>> for HandleResult<Error> {
    fn from(other: Result<(), Error>) -> HandleResult<Error> {
        match other {
            Ok(()) => HandleResult::Ok,
            Err(e) => HandleResult::Err(e),
        }
    }
}

impl From<()> for HandleResult<Infallible> {
    fn from(_: ()) -> HandleResult<Infallible> {
        HandleResult::Ok
    }
}

/// The result of the handling by the internal handlers or dispatchers.
///
/// It must be returned from the [`Handler`] (see [`HandleFuture`] type).
///
/// [`Handler`]: crate::dispatching::dev::Handler
/// [`HandleFuture`]: crate::dispatching::dev::HandleFuture
#[derive(Debug, PartialEq)]
pub enum DispatchError<Upd, Err> {
    /// There are no handler to handle the incoming update.
    NoHandler(Upd),
    /// There are handler which try to handle the update but it return the
    /// error.
    HandlerError(Err),
}
