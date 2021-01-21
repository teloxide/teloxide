use std::convert::Infallible;

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

pub enum DispatchError<Upd, Err> {
    NoHandler(Upd),
    HandlerError(Err),
}
