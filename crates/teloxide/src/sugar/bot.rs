//! Additions to [`Bot`].
//!
//! [`Bot`]: crate::Bot
use futures::stream::{self, Stream, StreamExt};

use crate::{prelude::*, types::*};

/// Syntax sugar for [`Message`] manipulations.
///
/// [`Message`]: crate::types::Message
pub trait BotMessagesExt: Requester {
    /// This function is the same as [`Bot::forward_message`],
    /// but can take in [`Message`] to forward it.
    ///
    /// [`Bot::forward_message`]: crate::Bot::forward_message
    /// [`Message`]: crate::types::Message
    fn forward<C>(&self, to_chat_id: C, message: &Message) -> Self::ForwardMessage
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
    ) -> Self::EditMessageLiveLocation;

    /// This function is the same as [`Bot::stop_message_live_location`],
    /// but can take in [`Message`] to stop the live location in it.
    ///
    /// [`Bot::stop_message_live_location`]: crate::Bot::stop_message_live_location
    /// [`Message`]: crate::types::Message
    fn stop_live_location(&self, message: &Message) -> Self::StopMessageLiveLocation;

    /// This function is the same as [`Bot::set_message_reaction`],
    /// but can take in [`Message`] to set a reaction on it.
    ///
    /// [`Bot::set_message_reaction`]: crate::Bot::set_message_reaction
    /// [`Message`]: crate::types::Message
    fn set_reaction(&self, message: &Message) -> Self::SetMessageReaction;

    /// This function is the same as [`Bot::pin_chat_message`],
    /// but can take in [`Message`] to pin it.
    ///
    /// [`Bot::pin_chat_message`]: crate::Bot::pin_chat_message
    /// [`Message`]: crate::types::Message
    fn pin(&self, message: &Message) -> Self::PinChatMessage;

    /// This function is the same as [`Bot::unpin_chat_message`],
    /// but can take in [`Message`] to unpin it.
    ///
    /// [`Bot::unpin_chat_message`]: crate::Bot::unpin_chat_message
    /// [`Message`]: crate::types::Message
    fn unpin(&self, message: &Message) -> Self::UnpinChatMessage;

    /// This function is the same as [`Bot::edit_message_text`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_text`]: crate::Bot::edit_message_text
    /// [`Message`]: crate::types::Message
    fn edit_text<T>(&self, message: &Message, text: T) -> Self::EditMessageText
    where
        T: Into<String>;

    /// This function is the same as [`Bot::edit_message_caption`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_caption`]: crate::Bot::edit_message_caption
    /// [`Message`]: crate::types::Message
    fn edit_caption(&self, message: &Message) -> Self::EditMessageCaption;

    /// This function is the same as [`Bot::edit_message_media`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_media`]: crate::Bot::edit_message_media
    /// [`Message`]: crate::types::Message
    fn edit_media(&self, message: &Message, media: InputMedia) -> Self::EditMessageMedia;

    /// This function is the same as [`Bot::edit_message_reply_markup`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_reply_markup`]: crate::Bot::edit_message_reply_markup
    /// [`Message`]: crate::types::Message
    fn edit_reply_markup(&self, message: &Message) -> Self::EditMessageReplyMarkup;

    /// This function is the same as [`Bot::stop_poll`],
    /// but can take in [`Message`] to stop the poll in it.
    ///
    /// [`Bot::stop_poll`]: crate::Bot::stop_poll
    /// [`Message`]: crate::types::Message
    fn stop_poll_message(&self, message: &Message) -> Self::StopPoll;

    /// This function is the same as [`Bot::delete_message`],
    /// but can take in [`Message`] to delete it.
    ///
    /// [`Bot::delete_message`]: crate::Bot::delete_message
    /// [`Message`]: crate::types::Message
    fn delete(&self, message: &Message) -> Self::DeleteMessage;

    /// This function is the same as [`Bot::copy_message`],
    /// but can take in [`Message`] to copy it.
    ///
    /// [`Bot::copy_messages`]: crate::Bot::copy_message
    /// [`Message`]: crate::types::Message
    fn copy<C>(&self, to_chat_id: C, message: &Message) -> Self::CopyMessage
    where
        C: Into<Recipient>;

    fn iter_star_transactions(&self) -> impl Stream<Item = StarTransaction>;
}

impl<R> BotMessagesExt for R
where
    R: Requester,
{
    fn forward<C>(&self, to_chat_id: C, message: &Message) -> Self::ForwardMessage
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
    ) -> Self::EditMessageLiveLocation {
        self.edit_message_live_location(message.chat.id, message.id, latitude, longitude)
    }

    fn stop_live_location(&self, message: &Message) -> Self::StopMessageLiveLocation {
        self.stop_message_live_location(message.chat.id, message.id)
    }

    fn set_reaction(&self, message: &Message) -> Self::SetMessageReaction {
        self.set_message_reaction(message.chat.id, message.id)
    }

    fn pin(&self, message: &Message) -> Self::PinChatMessage {
        self.pin_chat_message(message.chat.id, message.id)
    }

    fn unpin(&self, message: &Message) -> Self::UnpinChatMessage {
        self.unpin_chat_message(message.chat.id).message_id(message.id)
    }

    fn edit_text<T>(&self, message: &Message, text: T) -> Self::EditMessageText
    where
        T: Into<String>,
    {
        self.edit_message_text(message.chat.id, message.id, text)
    }

    fn edit_caption(&self, message: &Message) -> Self::EditMessageCaption {
        self.edit_message_caption(message.chat.id, message.id)
    }

    fn edit_media(&self, message: &Message, media: InputMedia) -> Self::EditMessageMedia {
        self.edit_message_media(message.chat.id, message.id, media)
    }

    fn edit_reply_markup(&self, message: &Message) -> Self::EditMessageReplyMarkup {
        self.edit_message_reply_markup(message.chat.id, message.id)
    }

    fn stop_poll_message(&self, message: &Message) -> Self::StopPoll {
        self.stop_poll(message.chat.id, message.id)
    }

    fn delete(&self, message: &Message) -> Self::DeleteMessage {
        self.delete_message(message.chat.id, message.id)
    }

    fn copy<C>(&self, to_chat_id: C, message: &Message) -> Self::CopyMessage
    where
        C: Into<Recipient>,
    {
        self.copy_message(to_chat_id, message.chat.id, message.id)
    }

    fn iter_star_transactions(&self) -> impl Stream<Item = StarTransaction> {
        stream::unfold(0, move |state| async move {
            let transactions: Result<StarTransactions, <R as Requester>::Err> =
                self.get_star_transactions().offset(state).await;

            match transactions {
                Ok(transactions) => Some((stream::iter(transactions.transactions), state + 100)),
                Err(_) => None,
            }
        })
        .flatten()
    }
}
