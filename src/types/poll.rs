use serde::{Deserialize, Serialize};

/// This object contains information about a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#poll).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,

    /// Poll question, 1-255 characters.
    pub question: String,

    /// List of poll options.
    pub options: Vec<PollOption>,

    /// `true`, if the poll is closed.
    pub is_closed: bool,
}

/// This object contains information about one answer option in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloption).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PollOption {
    /// Option text, 1-100 characters.
    pub text: String,

    /// Number of users that voted for this option.
    pub voter_count: i32,
}
