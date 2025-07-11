//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{
    BusinessConnectionId, DiceEmoji, EffectId, Message, Recipient, ReplyMarkup, ReplyParameters,
    ThreadId,
};

impl_payload! {
    /// Use this method to send an animated emoji that will display a random value. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendDice (SendDiceSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message will be sent
            pub business_connection_id: BusinessConnectionId,
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
            pub emoji: DiceEmoji,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
            pub allow_paid_broadcast: bool,
            /// Unique identifier of the message effect to be added to the message; for private chats only
            pub message_effect_id: EffectId,
            /// Description of the message to reply to
            pub reply_parameters: ReplyParameters,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove a reply keyboard or to force a reply from the user. Not supported for messages sent on behalf of a business account.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
