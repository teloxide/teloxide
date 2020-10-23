use serde::{Deserialize, Serialize};

use crate::types::{ChatId, Message, ParseMode, ReplyMarkup};

impl_payload! {
    /// Use this method to send text messages.
    ///
    /// On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[serde_with_macros::skip_serializing_none]
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
    pub SendMessage (SendMessageSetters) => Message {
        required {
            ///	Unique identifier for the target chat or username of the target channel
            /// (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Text of the message to be sent
            pub text: String [into],
        }
        optional {
            /// Send [Markdown] or [HTML], if you want Telegram apps to show
            /// [bold, italic, fixed-width text or inline URLs] in your bot's message.
            ///
            /// [Markdown]: crate::types::ParseMode::Markdown
            /// [HTML]: crate::types::ParseMode::HTML
            /// [bold, italic, fixed-width text or inline URLs]: crate::types::ParseMode
            pub parse_mode: ParseMode,
            /// Disables link previews for links in this message
            pub disable_web_page_preview: bool,
            /// Sends the message silently.
            /// Users will receive a notification with no sound.
            pub disable_notification: bool,
            /// If the message is a reply, [id] of the original message
            ///
            /// [id]: crate::types::Message::id
            pub reply_to_message_id: i32,
            /// Additional interface options.
            pub reply_markup: ReplyMarkup,
        }
    }
}
