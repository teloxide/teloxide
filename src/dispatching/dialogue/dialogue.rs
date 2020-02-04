/// A type, encapsulating a dialogue state and arbitrary data.
///
/// ## Example
/// ```
/// use teloxide::dispatching::dialogue::Dialogue;
///
/// enum MyState {
///     FullName,
///     Age,
///     FavouriteMusic,
/// }
///
/// #[derive(Default)]
/// struct User {
///     full_name: Option<String>,
///     age: Option<u8>,
///     favourite_music: Option<String>,
/// }
///
/// let _dialogue = Dialogue::new(MyState::FullName, User::default());
/// ```
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
