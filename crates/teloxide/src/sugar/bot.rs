//! Additions to [`Bot`].
//!
//! [`Bot`]: crate::Bot
use crate::{prelude::*, types::*};
use teloxide_core::{
    payloads::*,
    requests::{JsonRequest, MultipartRequest},
};

/// Syntax sugar for [`Message`] manipulations.
///
/// [`Message`]: crate::types::Message
pub trait BotMessagesExt {
    /// This function is the same as [`Bot::forward_message`],
    /// but can take in [`Message`] to forward it.
    ///
    /// [`Bot::forward_message`]: crate::Bot::forward_message
    /// [`Message`]: crate::types::Message
    fn forward<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<ForwardMessage>
    where
        C: Into<Recipient>;

    /// This function is the same as [`Bot::edit_message_live_location`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_live_location`]: crate::Bot::edit_message_live_location
    /// [`Message`]: crate::types::Message
    fn edit_live_location(
        &self,
        message: &Message,
        latitude: f64,
        longitude: f64,
    ) -> JsonRequest<EditMessageLiveLocation>;

    /// This function is the same as [`Bot::stop_message_live_location`],
    /// but can take in [`Message`] to stop the live location in it.
    ///
    /// [`Bot::stop_message_live_location`]: crate::Bot::stop_message_live_location
    /// [`Message`]: crate::types::Message
    fn stop_live_location(&self, message: &Message) -> JsonRequest<StopMessageLiveLocation>;

    /// This function is the same as [`Bot::set_message_reaction`],
    /// but can take in [`Message`] to set a reaction on it.
    ///
    /// [`Bot::set_message_reaction`]: crate::Bot::set_message_reaction
    /// [`Message`]: crate::types::Message
    fn set_reaction(&self, message: &Message) -> JsonRequest<SetMessageReaction>;

    /// This function is the same as [`Bot::pin_chat_message`],
    /// but can take in [`Message`] to pin it.
    ///
    /// [`Bot::pin_chat_message`]: crate::Bot::pin_chat_message
    /// [`Message`]: crate::types::Message
    fn pin(&self, message: &Message) -> JsonRequest<PinChatMessage>;

    /// This function is the same as [`Bot::unpin_chat_message`],
    /// but can take in [`Message`] to unpin it.
    ///
    /// [`Bot::unpin_chat_message`]: crate::Bot::unpin_chat_message
    /// [`Message`]: crate::types::Message
    fn unpin(&self, message: &Message) -> JsonRequest<UnpinChatMessage>;

    /// This function is the same as [`Bot::edit_message_text`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_text`]: crate::Bot::edit_message_text
    /// [`Message`]: crate::types::Message
    fn edit_text<T>(&self, message: &Message, text: T) -> JsonRequest<EditMessageText>
    where
        T: Into<String>;

    /// This function is the same as [`Bot::edit_message_caption`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_caption`]: crate::Bot::edit_message_caption
    /// [`Message`]: crate::types::Message
    fn edit_caption(&self, message: &Message) -> JsonRequest<EditMessageCaption>;

    /// This function is the same as [`Bot::edit_message_media`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_media`]: crate::Bot::edit_message_media
    /// [`Message`]: crate::types::Message
    fn edit_media(
        &self,
        message: &Message,
        media: InputMedia,
    ) -> MultipartRequest<EditMessageMedia>;

    /// This function is the same as [`Bot::edit_message_reply_markup`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_reply_markup`]: crate::Bot::edit_message_reply_markup
    /// [`Message`]: crate::types::Message
    fn edit_reply_markup(&self, message: &Message) -> JsonRequest<EditMessageReplyMarkup>;

    /// This function is the same as [`Bot::stop_poll`],
    /// but can take in [`Message`] to stop the poll in it.
    ///
    /// [`Bot::stop_poll`]: crate::Bot::stop_poll
    /// [`Message`]: crate::types::Message
    fn stop_poll_message(&self, message: &Message) -> JsonRequest<StopPoll>;

    /// This function is the same as [`Bot::delete_message`],
    /// but can take in [`Message`] to delete it.
    ///
    /// [`Bot::delete_message`]: crate::Bot::delete_message
    /// [`Message`]: crate::types::Message
    fn delete(&self, message: &Message) -> JsonRequest<DeleteMessage>;

    /// This function is the same as [`Bot::copy_message`],
    /// but can take in [`Message`] to copy it.
    ///
    /// [`Bot::copy_messages`]: crate::Bot::copy_message
    /// [`Message`]: crate::types::Message
    fn copy<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<CopyMessage>
    where
        C: Into<Recipient>;
}

impl BotMessagesExt for Bot {
    fn forward<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<ForwardMessage>
    where
        C: Into<Recipient>,
    {
        self.forward_message(to_chat_id, message.chat.id, message.id)
    }

    fn edit_live_location(
        &self,
        message: &Message,
        latitude: f64,
        longitude: f64,
    ) -> JsonRequest<EditMessageLiveLocation> {
        self.edit_message_live_location(message.chat.id, message.id, latitude, longitude)
    }

    fn stop_live_location(&self, message: &Message) -> JsonRequest<StopMessageLiveLocation> {
        self.stop_message_live_location(message.chat.id, message.id)
    }

    fn set_reaction(&self, message: &Message) -> JsonRequest<SetMessageReaction> {
        self.set_message_reaction(message.chat.id, message.id)
    }

    fn pin(&self, message: &Message) -> JsonRequest<PinChatMessage> {
        self.pin_chat_message(message.chat.id, message.id)
    }

    fn unpin(&self, message: &Message) -> JsonRequest<UnpinChatMessage> {
        self.unpin_chat_message(message.chat.id).message_id(message.id)
    }

    fn edit_text<T>(&self, message: &Message, text: T) -> JsonRequest<EditMessageText>
    where
        T: Into<String>,
    {
        self.edit_message_text(message.chat.id, message.id, text)
    }

    fn edit_caption(&self, message: &Message) -> JsonRequest<EditMessageCaption> {
        self.edit_message_caption(message.chat.id, message.id)
    }

    fn edit_media(
        &self,
        message: &Message,
        media: InputMedia,
    ) -> MultipartRequest<EditMessageMedia> {
        self.edit_message_media(message.chat.id, message.id, media)
    }

    fn edit_reply_markup(&self, message: &Message) -> JsonRequest<EditMessageReplyMarkup> {
        self.edit_message_reply_markup(message.chat.id, message.id)
    }

    fn stop_poll_message(&self, message: &Message) -> JsonRequest<StopPoll> {
        self.stop_poll(message.chat.id, message.id)
    }

    fn delete(&self, message: &Message) -> JsonRequest<DeleteMessage> {
        self.delete_message(message.chat.id, message.id)
    }

    fn copy<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<CopyMessage>
    where
        C: Into<Recipient>,
    {
        self.copy_message(to_chat_id, message.chat.id, message.id)
    }
}
