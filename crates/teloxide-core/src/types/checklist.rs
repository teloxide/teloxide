use crate::types::{MessageEntity, ParseMode, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Describes a task in a checklist.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: u8,

    /// Text of the task
    pub text: String,

    /// Special entities that appear in the task text
    pub text_entities: Option<Vec<MessageEntity>>,

    /// User that completed the task; omitted if the task wasn't completed
    pub completed_by_user: Option<User>,

    /// Point in time (Unix timestamp) when the task was completed; 0 if the
    /// task wasn't completed
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub completion_date: Option<DateTime<Utc>>,
}

impl ChecklistTask {
    pub fn is_completed(&self) -> bool {
        match self.completion_date {
            Some(completion_date) => completion_date.timestamp() > 0,
            None => false,
        }
    }
}

/// Describes a checklist.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Checklist {
    /// Title of the checklist
    pub title: String,

    /// Special entities that appear in the checklist title
    pub title_entities: Option<Vec<MessageEntity>>,

    /// List of tasks in the checklist
    pub tasks: Vec<ChecklistTask>,

    /// `true`, if users other than the creator of the list can add tasks to the
    /// list
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub others_can_add_tasks: bool,

    /// `true`, if users other than the creator of the list can mark tasks as
    /// done or not done
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub others_can_mark_tasks_as_done: bool,
}

/// Describes a task to add to a checklist.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique among all
    /// task identifiers currently present in the checklist
    pub id: u8,

    /// Text of the task; 1-100 characters after entities parsing
    pub text: String,

    /// Mode for parsing entities in the text. See [formatting options] for more
    /// details.
    ///
    /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the text, which can be specified
    /// instead of parse_mode. Currently, only _bold_, _italic_, _underline_,
    /// _strikethrough_, _spoiler_, and _custom_emoji_ entities are allowed
    pub text_entities: Option<Vec<MessageEntity>>,
}

/// Describes a checklist to create.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InputChecklist {
    /// Title of the checklist; 1-255 characters after entities parsing
    pub title: String,

    /// Mode for parsing entities in the title. See [formatting options] for
    /// more details.
    ///
    /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the title, which can be
    /// specified instead of parse_mode. Currently, only _bold_, _italic_,
    /// _underline_, _strikethrough_, _spoiler_, and _custom_emoji_ entities
    /// are allowed
    pub title_entities: Option<Vec<MessageEntity>>,

    /// List of 1-30 tasks in the checklist
    pub tasks: Vec<InputChecklistTask>,

    /// Pass `true` if other users can add tasks to the checklist
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub others_can_add_tasks: bool,

    /// Pass `true` if other users can mark tasks as done or not done in the
    /// checklist
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub others_can_mark_tasks_as_done: bool,
}
