/// A type, encapsulating a dialogue state and arbitrary data.
#[derive(Default, Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Dialogue<State, T> {
    pub state: State,
    pub data: T,
}

impl<State, T> Dialogue<State, T> {
    /// Creates new `Dialogue` with the provided fields.
    #[must_use]
    pub fn new(state: State, data: T) -> Self {
        Self { state, data }
    }
}
