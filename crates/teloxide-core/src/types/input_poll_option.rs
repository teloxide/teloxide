use serde::{Deserialize, Serialize};

use crate::types::{MessageEntity, ParseMode};

/// This object contains information about one answer option in a poll to send.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloption).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InputPollOption {
    /// Option text, 1-100 characters.
    pub text: String,

    /// Mode for parsing entities in the text. See [formatting options] for more
    /// details. Currently, only custom emoji entities are allowed.
    ///
    /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
    pub text_parse_mode: Option<ParseMode>,

    /// A JSON-serialized list of special entities that appear in the poll
    /// option text. It can be specified instead of _text\_parse\_mode_.
    pub text_entities: Option<Vec<MessageEntity>>,
}
