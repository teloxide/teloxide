//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{Message, MessageId, Recipient, Seconds, ThreadId};

impl_payload! {
    /// Use this method to forward messages of any kind. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub ForwardMessage (ForwardMessageSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
            pub from_chat_id: Recipient [into],
            /// Message identifier in the chat specified in _from\_chat\_id_
            #[serde(flatten)]
            pub message_id: MessageId,
        }
        optional {
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// New start timestamp for the forwarded video in the message
            pub video_start_timestamp: Seconds,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
        }
    }
}
