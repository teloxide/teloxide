use crate::types::MessageId;

use serde::{Deserialize, Serialize};

/// Reply thread identifier.
///
/// A message that isn't a reply and other messages that reply to it directly or
/// indirectly through a reply chain constitute a reply thread. All messages
/// except the initial root message have an additional [`thread_id`] field that
/// is equal to the root message's id.
///
/// In other words a thread id can be found recursively following
/// `reply_to_message_id`, until you find a message which does not reply to
/// anything, it's id is the thread id.
///
/// For example:
///
/// ```text
/// lizard: Hi {id:17} <-------------+--------------+----+--------------+---------------------+
///                                  |              |    |              |                     |
/// wizard: hewwo {id:18, reply: 17 -+, thread: 17 -+}   |              |                     |
///                                                      |              |                     |
/// lizard: I've been wondering [...] {id:19, reply: 17 -+, thread: 17 -+} <---+              |
///                                                                            |              |
/// neushoorn: wait, did y'all know that [...] {id:20} <-----------------------)--------------)--+-----+
///                                                                            |              |  |     |
/// wizard: so this is not an easy question, actually [...] {id:21, reply: 19 -+, thread: 17 -+} |     |
///                                                                                     +--------+     |
///                                                                                     |              |
/// wizard: everyone knows that, how did you not know that before?... {id:22, reply:20 -+, thread: 20 -+}
/// ```
///
/// Note that channel comments and forum topics, reuse threads for different
/// meanings. For channel comments every comment (indirectly) replies to the
/// channel post forwarded to the linked chat (i.e. the forwarded channel post
/// is the root of the comments thread). For forum topics every message in a
/// topic is in the same thread too (i.e. they (indirectly) reply to the start
/// of the topic).
///
/// [`thread_id`]: crate::types::Message::thread_id
#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(from = "ThreadIdRaw", into = "ThreadIdRaw")]
pub struct ThreadId(/** Identifier of the root message in a reply thread. */ pub MessageId);

// N.B. this is a hack to [de]serialize `ThreadId` as just a number
//      we need this since `MessageId` is [de]serialized as `{"message_id":n}`.

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct ThreadIdRaw(i32);

impl From<ThreadIdRaw> for ThreadId {
    fn from(ThreadIdRaw(message_id): ThreadIdRaw) -> Self {
        ThreadId(MessageId(message_id))
    }
}

impl From<ThreadId> for ThreadIdRaw {
    fn from(ThreadId(MessageId(message_id)): ThreadId) -> Self {
        ThreadIdRaw(message_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{MessageId, ThreadId};

    #[test]
    fn smoke_deser() {
        let json = "123";
        let mid: ThreadId = serde_json::from_str(json).unwrap();
        assert_eq!(mid, ThreadId(MessageId(123)));
    }

    #[test]
    fn smoke_ser() {
        let mid: ThreadId = ThreadId(MessageId(123));
        let json = serde_json::to_string(&mid).unwrap();
        assert_eq!(json, "123");
    }
}
