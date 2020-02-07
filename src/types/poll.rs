use crate::types::PollType;
use serde::{Deserialize, Serialize};

/// This object contains information about a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#poll).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,

    /// Poll question, 1-255 characters.
    pub question: String,

    /// List of poll options.
    pub options: Vec<PollOption>,

    /// `true`, if the poll is closed.
    pub is_closed: bool,

    /// Total number of users that voted in the poll
    pub total_voter_count: i32,

    /// True, if the poll is anonymous
    pub is_anonymous: bool,

    /// Poll type, currently can be “regular” or “quiz”
    pub poll_type: PollType,

    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,

    /// 0-based identifier of the correct answer option. Available only for
    /// polls in the quiz mode, which are closed, or was sent (not
    /// forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<i32>,
}

/// This object contains information about one answer option in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloption).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters.
    pub text: String,

    /// Number of users that voted for this option.
    pub voter_count: i32,
}
