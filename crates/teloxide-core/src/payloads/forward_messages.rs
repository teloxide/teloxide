//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{MessageId, Recipient, ThreadId};

impl_payload! {
    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [`MessageId`] of the sent messages is returned.
    ///
    /// [`MessageId`]: crate::types::MessageId
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub ForwardMessages (ForwardMessagesSetters) => Vec<MessageId> {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
            pub from_chat_id: Recipient [into],
            /// A JSON-serialized list of 1-100 identifiers of messages in the chat _from\_chat\_id_ to forward. The identifiers must be specified in a strictly increasing order.
            #[serde(with = "crate::types::vec_msg_id_as_vec_int")]
            pub message_ids: Vec<MessageId> [collect],
        }
        optional {
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
        }
    }
}
