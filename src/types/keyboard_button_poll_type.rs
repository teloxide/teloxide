use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    poll_type: String,
}

impl KeyboardButtonPollType {
    pub fn new<S>(poll_type: S) -> Self
    where
        S: Into<String>,
    {
        Self { poll_type: poll_type.into() }
    }

    pub fn poll_type<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.poll_type = val.into();
        self
    }
}
