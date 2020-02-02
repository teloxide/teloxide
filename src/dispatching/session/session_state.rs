/// Continue or terminate a user session.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum SessionState<Session> {
    Next(Session),
    Exit,
}
