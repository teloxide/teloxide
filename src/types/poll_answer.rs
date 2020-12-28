use crate::types::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: String,

    /// The user, who changed the answer to the poll.
    pub user: User,

    /// 0-based identifiers of answer options, chosen by the user.
    ///
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<i32>,
}

impl PollAnswer {
    pub fn new<S, O>(poll_id: S, user: User, option_ids: O) -> Self
    where
        S: Into<String>,
        O: Into<Vec<i32>>,
    {
        Self {
            poll_id: poll_id.into(),
            user,
            option_ids: option_ids.into(),
        }
    }

    pub fn poll_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.poll_id = val.into();
        self
    }

    pub fn user(mut self, val: User) -> Self {
        self.user = val;
        self
    }

    pub fn option_ids<S>(mut self, val: S) -> Self
    where
        S: Into<Vec<i32>>,
    {
        self.option_ids = val.into();
        self
    }
}
