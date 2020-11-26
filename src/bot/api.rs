use crate::{
    payloads,
    requests::{
        AddStickerToSet, AnswerCallbackQuery, AnswerInlineQuery, AnswerPreCheckoutQuery,
        AnswerShippingQuery, CreateNewStickerSet, DeleteChatPhoto, DeleteChatStickerSet,
        DeleteMessage, DeleteStickerFromSet, DeleteWebhook, EditInlineMessageCaption,
        EditInlineMessageLiveLocation, EditInlineMessageMedia, EditInlineMessageReplyMarkup,
        EditInlineMessageText, EditMessageCaption, EditMessageLiveLocation, EditMessageMedia,
        EditMessageReplyMarkup, EditMessageText, ExportChatInviteLink, ForwardMessage, GetChat,
        GetChatAdministrators, GetChatMember, GetChatMembersCount, GetFile, GetGameHighScores,
        GetMe, GetMyCommands, GetStickerSet, GetUpdates, GetUpdatesNonStrict, GetUserProfilePhotos,
        GetWebhookInfo, JsonRequest, KickChatMember, LeaveChat, MultipartRequest, PinChatMessage,
        PromoteChatMember, Requester, RestrictChatMember, SendAnimation, SendAudio, SendChatAction,
        SendChatActionKind, SendContact, SendDice, SendDocument, SendGame, SendInvoice,
        SendLocation, SendMediaGroup, SendMessage, SendPhoto, SendPoll, SendSticker, SendVenue,
        SendVideo, SendVideoNote, SendVoice, SetChatAdministratorCustomTitle, SetChatDescription,
        SetChatPermissions, SetChatPhoto, SetChatStickerSet, SetChatTitle, SetGameScore,
        SetMyCommands, SetStickerPositionInSet, SetStickerSetThumb, SetWebhook,
        StopInlineMessageLiveLocation, StopMessageLiveLocation, StopPoll, UnbanChatMember,
        UnpinChatMessage, UploadStickerFile,
    },
    types::{
        BotCommand, ChatId, ChatPermissions, InlineQueryResult, InputFile, InputMedia,
        InputSticker, LabeledPrice, TargetMessage,
    },
    Bot,
};

impl Bot {
    /// Use this method to receive incoming updates using long polling ([wiki]).
    ///
    /// **Notes:**
    /// 1. This method will not work if an outgoing webhook is set up.
    /// 2. In order to avoid getting duplicate updates,
    ///    recalculate offset after each server response.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getupdates).
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates::new(self.clone())
    }

    /// This is non strict version of [`get_updates`], this means that if it
    /// will fail to deserialize some updates, it won't fail entirely, but
    /// will just return some errors.
    ///
    /// Note: this is not a 'real' telegram method, this is simply
    /// [`get_updates`] with changed return type.
    ///
    /// [`get_updates`]: crate::Bot::get_updates
    pub fn get_updates_non_strict(&self) -> GetUpdatesNonStrict {
        GetUpdatesNonStrict::new(self.clone())
    }

    /// Use this method to specify a url and receive incoming updates via an
    /// outgoing webhook.
    ///
    /// Whenever there is an update for the bot, we will send an
    /// HTTPS POST request to the specified url, containing a JSON-serialized
    /// [`Update`]. In case of an unsuccessful request, we will give up after a
    /// reasonable amount of attempts.
    ///
    /// If you'd like to make sure that the Webhook request comes from Telegram,
    /// we recommend using a secret path in the URL, e.g.
    /// `https://www.example.com/<token>`. Since nobody else knows your bot‘s
    /// token, you can be pretty sure it’s us.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setwebhook).
    ///
    /// # Params
    ///   - `url`: HTTPS url to send updates to.
    ///
    /// Use an empty string to remove webhook integration.
    ///
    /// [`Update`]: crate::types::Update
    pub fn set_webhook<U>(&self, url: U) -> SetWebhook
    where
        U: Into<String>,
    {
        SetWebhook::new(self.clone(), url)
    }

    /// Use this method to remove webhook integration if you decide to switch
    /// back to [Bot::get_updates].
    ///
    /// [The official docs](https://core.telegram.org/bots/api#deletewebhook).
    ///
    /// [Bot::get_updates]: crate::Bot::get_updates
    pub fn delete_webhook(&self) -> DeleteWebhook {
        DeleteWebhook::new(self.clone())
    }

    /// Use this method to get current webhook status.
    ///
    /// If the bot is using [`Bot::get_updates`], will return an object with the
    /// url field empty.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getwebhookinfo).
    ///
    /// [`Bot::get_updates`]: crate::Bot::get_updates
    pub fn get_webhook_info(&self) -> GetWebhookInfo {
        GetWebhookInfo::new(self.clone())
    }

    /// A simple method for testing your bot's auth token. Requires no
    /// parameters.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getme).
    pub fn get_me(&self) -> GetMe {
        GetMe::new(self.clone())
    }

    /// Use this method to send text messages.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendmessage).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `text`: Text of the message to be sent.
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn send_message<C, T>(&self, chat_id: C, text: T) -> SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessage::new(self.clone(), chat_id, text)
    }

    /// Use this method to forward messages of any kind.
    ///
    /// [`The official docs`](https://core.telegram.org/bots/api#forwardmessage).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `from_chat_id`: Unique identifier for the chat where the original
    ///     message was sent (or channel username in the format
    ///     `@channelusername`).
    ///   - `message_id`: Message identifier in the chat specified in
    ///     [`from_chat_id`].
    ///
    /// [`from_chat_id`]: ForwardMessage::from_chat_id
    pub fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: i32,
    ) -> ForwardMessage
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        ForwardMessage::new(self.clone(), chat_id, from_chat_id, message_id)
    }

    /// Use this method to send photos.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendphoto).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `photo`: Photo to send.
    ///
    /// Pass [`InputFile::File`] to send a photo that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn send_photo<C>(&self, chat_id: C, photo: InputFile) -> SendPhoto
    where
        C: Into<ChatId>,
    {
        SendPhoto::new(self.clone(), chat_id, photo)
    }

    ///
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn send_audio<C>(&self, chat_id: C, audio: InputFile) -> SendAudio
    where
        C: Into<ChatId>,
    {
        SendAudio::new(self.clone(), chat_id, audio)
    }

    /// Use this method to send general files.
    ///
    /// Bots can currently send files of any type of up to 50 MB in size, this
    /// limit may be changed in the future.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#senddocument).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `document`: File to send.
    ///
    /// Pass a file_id as String to send a file that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// `multipart/form-data`. [More info on Sending Files »].
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn send_document<C>(&self, chat_id: C, document: InputFile) -> SendDocument
    where
        C: Into<ChatId>,
    {
        SendDocument::new(self.clone(), chat_id, document)
    }

    /// Use this method to send video files, Telegram clients support mp4 videos
    /// (other formats may be sent as Document).
    ///
    /// Bots can currently send video files of up to 50 MB in size, this
    /// limit may be changed in the future.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendvideo).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `video`: Video to sent.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn send_video<C>(&self, chat_id: C, video: InputFile) -> SendVideo
    where
        C: Into<ChatId>,
    {
        SendVideo::new(self.clone(), chat_id, video)
    }

    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video
    /// without sound).
    ///
    /// Bots can currently send animation files of up to 50 MB in size, this
    /// limit may be changed in the future.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendanimation).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `animation`: Animation to send.
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn send_animation<C>(&self, chat_id: C, animation: InputFile) -> SendAnimation
    where
        C: Into<ChatId>,
    {
        SendAnimation::new(self.clone(), chat_id, animation)
    }

    /// Use this method to send audio files, if you want Telegram clients to
    /// display the file as a playable voice message.
    ///
    /// For this to work, your audio must be in an .ogg file encoded with OPUS
    /// (other formats may be sent as [`Audio`] or [`Document`]). Bots can
    /// currently send voice messages of up to 50 MB in size, this limit may
    /// be changed in the future.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendvoice).
    ///
    /// [`Audio`]: crate::types::Audio
    /// [`Document`]: crate::types::Document
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `voice`: Audio file to send.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn send_voice<C>(&self, chat_id: C, voice: InputFile) -> SendVoice
    where
        C: Into<ChatId>,
    {
        SendVoice::new(self.clone(), chat_id, voice)
    }

    /// As of [v.4.0], Telegram clients support rounded square mp4 videos of up
    /// to 1 minute long. Use this method to send video messages.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendvideonote).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `video_note`: Video note to send.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on the Telegram
    /// servers (recommended), pass an [`InputFile::Url`] for Telegram to get a
    /// .webp file from the Internet, or upload a new one using
    /// [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [v.4.0]: https://telegram.org/blog/video-messages-and-telescope
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files

    pub fn send_video_note<C>(&self, chat_id: C, video_note: InputFile) -> SendVideoNote
    where
        C: Into<ChatId>,
    {
        SendVideoNote::new(self.clone(), chat_id, video_note)
    }

    /// Use this method to send a group of photos or videos as an album.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendmediagroup).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `media`: A JSON-serialized array describing photos and videos to be
    ///     sent, must include 2–10 items.
    pub fn send_media_group<C, M>(&self, chat_id: C, media: M) -> SendMediaGroup
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        SendMediaGroup::new(self.clone(), chat_id, media)
    }

    /// Use this method to send point on the map.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendlocation).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `latitude`: Latitude of the location.
    ///   - `longitude`: Latitude of the location.
    pub fn send_location<C>(&self, chat_id: C, latitude: f32, longitude: f32) -> SendLocation
    where
        C: Into<ChatId>,
    {
        SendLocation::new(self.clone(), chat_id, latitude, longitude)
    }

    /// Use this method to edit live location messages.
    ///
    /// A location can be edited until its live_period expires or editing is
    /// explicitly disabled by a call to stopMessageLiveLocation. On success,
    /// the edited [`Message`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagelivelocation).
    ///
    /// [`Message`]: crate::types::Message
    ///
    /// # Params
    ///   - `latitude`: Latitude of new location.
    ///   - `longitude`: Longitude of new location.
    pub fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f32,
        longitude: f32,
    ) -> EditMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        EditMessageLiveLocation::new(self.clone(), chat_id, message_id, latitude, longitude)
    }

    /// Use this method to edit live location messages sent via the bot.
    ///
    /// A location can be edited until its live_period expires or editing is
    /// explicitly disabled by a call to stopMessageLiveLocation. On success,
    /// [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagelivelocation).
    ///
    /// [`True`]: crate::types::True
    ///
    /// # Params
    ///   - `latitude`: Latitude of new location.
    ///   - `longitude`: Longitude of new location.
    pub fn edit_inline_message_live_location<I>(
        &self,
        inline_message_id: I,
        latitude: f32,
        longitude: f32,
    ) -> EditInlineMessageLiveLocation
    where
        I: Into<String>,
    {
        EditInlineMessageLiveLocation::new(self.clone(), inline_message_id, latitude, longitude)
    }

    /// Use this method to stop updating a live location message before
    /// `live_period` expires.
    ///
    /// On success, the sent [`Message`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#stopmessagelivelocation).
    ///
    /// [`Message`]: crate::types::Message
    pub fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> StopMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        StopMessageLiveLocation::new(self.clone(), chat_id, message_id)
    }

    /// Use this method to stop updating a live location message (sent via the
    /// bot) before `live_period` expires.
    ///
    /// On success, [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#stopmessagelivelocation).
    ///
    /// [`True`]: crate::types::True
    pub fn stop_inline_message_live_location<I>(
        &self,
        inline_message_id: I,
    ) -> StopInlineMessageLiveLocation
    where
        I: Into<String>,
    {
        StopInlineMessageLiveLocation::new(self.clone(), inline_message_id)
    }

    /// Use this method to send information about a venue.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendvenue).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///  - `latitude`: Latitude of the venue.
    ///  - `longitude`: Longitude of the venue.
    ///  - `title`: Name of the venue.
    ///  - `address`: Address of the venue.
    pub fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f32,
        longitude: f32,
        title: T,
        address: A,
    ) -> SendVenue
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        SendVenue::new(self.clone(), chat_id, latitude, longitude, title, address)
    }

    /// Use this method to send phone contacts.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendcontact).
    ///
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `phone_number`: Contact's phone number.
    ///   - `first_name`: Contact's first name.
    pub fn send_contact<C, P, F>(&self, chat_id: C, phone_number: P, first_name: F) -> SendContact
    where
        C: Into<ChatId>,
        P: Into<String>,
        F: Into<String>,
    {
        SendContact::new(self.clone(), chat_id, phone_number, first_name)
    }

    /// Use this method to send a native poll. A native poll can't be sent to a
    /// private chat.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendpoll).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `question`: Poll question, 1-255 characters.
    ///   - `options`: List of answer options, 2-10 strings 1-100 characters
    ///     each.
    ///
    /// # Notes
    /// Uses [a default parse mode] ([`SendPoll::explanation_parse_mode`]) if
    /// specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    /// [`SendPoll::explanation_parse_mode`]:
    /// [`SendPoll::explanation_parse_mode`]:
    /// crate::types::SendPoll::explanation_parse_mode
    pub fn send_poll<C, Q, O>(&self, chat_id: C, question: Q, options: O) -> SendPoll
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: Into<Vec<String>>,
    {
        SendPoll::new(self.clone(), chat_id, question, options)
    }

    /// Use this method when you need to tell the user that something is
    /// happening on the bot's side.
    ///
    /// The status is set for 5 seconds or less (when a message arrives from
    /// your bot, Telegram clients clear its typing status).
    ///
    /// ## Note
    /// Example: The [ImageBot] needs some time to process a request and upload
    /// the image. Instead of sending a text message along the lines of
    /// “Retrieving image, please wait…”, the bot may use
    /// [`Bot::send_chat_action`] with `action = upload_photo`. The user
    /// will see a `sending photo` status for the bot.
    ///
    /// We only recommend using this method when a response from the bot will
    /// take a **noticeable** amount of time to arrive.
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///
    /// [ImageBot]: https://t.me/imagebot
    /// [`Bot::send_chat_action`]: crate::Bot::send_chat_action
    pub fn send_chat_action<C>(&self, chat_id: C, action: SendChatActionKind) -> SendChatAction
    where
        C: Into<ChatId>,
    {
        SendChatAction::new(self.clone(), chat_id, action)
    }

    /// Use this method to get a list of profile pictures for a user.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getuserprofilephotos).
    ///
    /// # Params
    ///   - `user_id`: Unique identifier of the target user.
    pub fn get_user_profile_photos(&self, user_id: i32) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(self.clone(), user_id)
    }

    /// Use this method to get basic info about a file and prepare it for
    /// downloading.
    ///
    /// For the moment, bots can download files of up to `20MB` in size.
    ///
    /// The file can then be downloaded via the link
    /// `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>`
    /// is taken from the response. It is guaranteed that the link will be valid
    /// for at least `1` hour. When the link expires, a new one can be requested
    /// by calling [`GetFile`] again.
    ///
    /// **Note**: This function may not preserve the original file name and MIME
    /// type. You should save the file's MIME type and name (if available) when
    /// the [`File`] object is received.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getfile).
    ///
    /// # Params
    ///   - `file_id`: File identifier to get info about.
    ///
    /// [`File`]: crate::types::File
    /// [`GetFile`]: self::GetFile
    pub fn get_file<F>(&self, file_id: F) -> GetFile
    where
        F: Into<String>,
    {
        GetFile::new(self.clone(), file_id)
    }

    /// Use this method to kick a user from a group, a supergroup or a channel.
    ///
    /// In the case of supergroups and channels, the user will not be able to
    /// return to the group on their own using invite links, etc., unless
    /// [unbanned] first. The bot must be an administrator in the chat for
    /// this to work and must have the appropriate admin rights.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#kickchatmember).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `user_id`: Unique identifier of the target user.
    ///
    /// [unbanned]: crate::Bot::unban_chat_member
    pub fn kick_chat_member<C>(&self, chat_id: C, user_id: i32) -> KickChatMember
    where
        C: Into<ChatId>,
    {
        KickChatMember::new(self.clone(), chat_id, user_id)
    }

    /// Use this method to unban a previously kicked user in a supergroup or
    /// channel. The user will **not** return to the group or channel
    /// automatically, but will be able to join via link, etc. The bot must
    /// be an administrator for this to work.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#unbanchatmember).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `user_id`: Unique identifier of the target user.
    pub fn unban_chat_member<C>(&self, chat_id: C, user_id: i32) -> UnbanChatMember
    where
        C: Into<ChatId>,
    {
        UnbanChatMember::new(self.clone(), chat_id, user_id)
    }

    /// Use this method to restrict a user in a supergroup.
    ///
    /// The bot must be an administrator in the supergroup for this to work and
    /// must have the appropriate admin rights. Pass `true` for all
    /// permissions to lift restrictions from a user.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#restrictchatmember).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `user_id`: Unique identifier of the target user.
    ///   - `permissions`: New user permissions.
    pub fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> RestrictChatMember
    where
        C: Into<ChatId>,
    {
        RestrictChatMember::new(self.clone(), chat_id, user_id, permissions)
    }

    /// Use this method to promote or demote a user in a supergroup or a
    /// channel.
    ///
    /// The bot must be an administrator in the chat for this to work and must
    /// have the appropriate admin rights. Pass False for all boolean
    /// parameters to demote a user.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#promotechatmember).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `user_id`: Unique identifier of the target user.
    pub fn promote_chat_member<C>(&self, chat_id: C, user_id: i32) -> PromoteChatMember
    where
        C: Into<ChatId>,
    {
        PromoteChatMember::new(self.clone(), chat_id, user_id)
    }

    /// Use this method to set default chat permissions for all members.
    ///
    /// The bot must be an administrator in the group or a supergroup for this
    /// to work and must have the can_restrict_members admin rights.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setchatpermissions).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `permissions`: New default chat permissions.
    pub fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> SetChatPermissions
    where
        C: Into<ChatId>,
    {
        SetChatPermissions::new(self.clone(), chat_id, permissions)
    }

    /// Use this method to generate a new invite link for a chat; any previously
    /// generated link is revoked.
    ///
    /// The bot must be an administrator in the chat for this to work and must
    /// have the appropriate admin rights.
    ///
    /// # Note
    /// Each administrator in a chat generates their own invite links. Bots
    /// can't use invite links generated by other administrators. If you
    /// want your bot to work with invite links, it will need to generate
    /// its own link using [`Bot::export_chat_invite_link`] – after this the
    /// link will become available to the bot via the [`Bot::get_chat`]
    /// method. If your bot needs to generate a new invite link replacing
    /// its previous one, use [`Bot::export_chat_invite_link`] again.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#exportchatinvitelink).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///
    /// [`Bot::export_chat_invite_link`]: crate::Bot::export_chat_invite_link
    /// [`Bot::get_chat`]: crate::Bot::get_chat
    pub fn export_chat_invite_link<C>(&self, chat_id: C) -> ExportChatInviteLink
    where
        C: Into<ChatId>,
    {
        ExportChatInviteLink::new(self.clone(), chat_id)
    }

    /// Use this method to set a new profile photo for the chat.
    ///
    /// Photos can't be changed for private chats. The bot must be an
    /// administrator in the chat for this to work and must have the
    /// appropriate admin rights.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setchatphoto).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `photo`: New chat photo, uploaded using `multipart/form-data`.
    pub fn set_chat_photo<C>(&self, chat_id: C, photo: InputFile) -> SetChatPhoto
    where
        C: Into<ChatId>,
    {
        SetChatPhoto::new(self.clone(), chat_id, photo)
    }

    /// Use this method to delete a chat photo. Photos can't be changed for
    /// private chats. The bot must be an administrator in the chat for this
    /// to work and must have the appropriate admin rights.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#deletechatphoto).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    pub fn delete_chat_photo<C>(&self, chat_id: C) -> DeleteChatPhoto
    where
        C: Into<ChatId>,
    {
        DeleteChatPhoto::new(self.clone(), chat_id)
    }

    /// Use this method to change the title of a chat.
    ///
    /// Titles can't be changed for private chats. The bot must be an
    /// administrator in the chat for this to work and must have the
    /// appropriate admin rights.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setchattitle).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `title`: New chat title, 1-255 characters.
    pub fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> SetChatTitle
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SetChatTitle::new(self.clone(), chat_id, title)
    }

    /// Use this method to change the description of a group, a supergroup or a
    /// channel.
    ///
    /// The bot must be an administrator in the chat for this to work and must
    /// have the appropriate admin rights.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setchatdescription).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    pub fn set_chat_description<C>(&self, chat_id: C) -> SetChatDescription
    where
        C: Into<ChatId>,
    {
        SetChatDescription::new(self.clone(), chat_id)
    }

    /// Use this method to pin a message in a group, a supergroup, or a channel.
    ///
    /// The bot must be an administrator in the chat for this to work and must
    /// have the `can_pin_messages` admin right in the supergroup or
    /// `can_edit_messages` admin right in the channel.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#pinchatmessage).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `message_id`: Identifier of a message to pin.
    pub fn pin_chat_message<C>(&self, chat_id: C, message_id: i32) -> PinChatMessage
    where
        C: Into<ChatId>,
    {
        PinChatMessage::new(self.clone(), chat_id, message_id)
    }

    /// Use this method to unpin a message in a group, a supergroup, or a
    /// channel.
    ///
    /// The bot must be an administrator in the chat for this to work and must
    /// have the `can_pin_messages` admin right in the supergroup or
    /// `can_edit_messages` admin right in the channel.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#unpinchatmessage).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    pub fn unpin_chat_message<C>(&self, chat_id: C) -> UnpinChatMessage
    where
        C: Into<ChatId>,
    {
        UnpinChatMessage::new(self.clone(), chat_id)
    }

    /// Use this method for your bot to leave a group, supergroup or channel.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#leavechat).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    pub fn leave_chat<C>(&self, chat_id: C) -> LeaveChat
    where
        C: Into<ChatId>,
    {
        LeaveChat::new(self.clone(), chat_id)
    }

    /// Use this method to get up to date information about the chat (current
    /// name of the user for one-on-one conversations, current username of a
    /// user, group or channel, etc.).
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getchat).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    pub fn get_chat<C>(&self, chat_id: C) -> GetChat
    where
        C: Into<ChatId>,
    {
        GetChat::new(self.clone(), chat_id)
    }

    /// Use this method to get a list of administrators in a chat.
    ///
    /// If the chat is a group or a supergroup and no administrators were
    /// appointed, only the creator will be returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getchatadministrators).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    pub fn get_chat_administrators<C>(&self, chat_id: C) -> GetChatAdministrators
    where
        C: Into<ChatId>,
    {
        GetChatAdministrators::new(self.clone(), chat_id)
    }

    /// Use this method to get the number of members in a chat.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getchatmemberscount).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    pub fn get_chat_members_count<C>(&self, chat_id: C) -> GetChatMembersCount
    where
        C: Into<ChatId>,
    {
        GetChatMembersCount::new(self.clone(), chat_id)
    }

    /// Use this method to get information about a member of a chat.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getchatmember).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup or channel (in the format `@channelusername`).
    ///   - `user_id`: Unique identifier of the target user.
    pub fn get_chat_member<C>(&self, chat_id: C, user_id: i32) -> GetChatMember
    where
        C: Into<ChatId>,
    {
        GetChatMember::new(self.clone(), chat_id, user_id)
    }

    /// Use this method to set a new group sticker set for a supergroup.
    ///
    /// The bot must be an administrator in the chat for this to work and must
    /// have the appropriate admin rights. Use the field can_set_sticker_set
    /// optionally returned in getChat requests to check if the bot can use
    /// this method.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setchatstickerset).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup (in the format `@supergroupusername`).
    ///   - `sticker_set_name`: Name of the sticker set to be set as the group
    ///     sticker set.
    pub fn set_chat_sticker_set<C, S>(&self, chat_id: C, sticker_set_name: S) -> SetChatStickerSet
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        SetChatStickerSet::new(self.clone(), chat_id, sticker_set_name)
    }

    /// Use this method to delete a group sticker set from a supergroup.
    ///
    /// The bot must be an administrator in the chat for this to work and must
    /// have the appropriate admin rights. Use the field
    /// `can_set_sticker_set` optionally returned in [`Bot::get_chat`]
    /// requests to check if the bot can use this method.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#deletechatstickerset).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target supergroup (in the format `@supergroupusername`).
    ///
    /// [`Bot::get_chat`]: crate::Bot::get_chat
    pub fn delete_chat_sticker_set<C>(&self, chat_id: C) -> DeleteChatStickerSet
    where
        C: Into<ChatId>,
    {
        DeleteChatStickerSet::new(self.clone(), chat_id)
    }

    /// Use this method to send answers to callback queries sent from [inline
    /// keyboards].
    ///
    /// The answer will be displayed to the user as a notification at
    /// the top of the chat screen or as an alert.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#answercallbackquery).
    ///
    /// # Params
    ///   - `callback_query_id`: Unique identifier for the query to be answered.
    ///
    /// [inline keyboards]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn answer_callback_query<C>(&self, callback_query_id: C) -> AnswerCallbackQuery
    where
        C: Into<String>,
    {
        AnswerCallbackQuery::new(self.clone(), callback_query_id)
    }

    /// Use this method to edit text and game messages.
    ///
    /// On success, the edited [`Message`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagetext).
    ///
    /// [`Message`]: crate::types::Message
    ///
    /// # Params
    ///
    ///  - `chat_id`: Unique identifier for the target chat or username of the
    ///    target channel (in the format `@channelusername`).
    ///  - `message_id`: Identifier of the message to edit.
    ///  - `text`: New text of the message.
    ///
    /// # Notes
    ///
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn edit_message_text<C, T>(&self, chat_id: C, message_id: i32, text: T) -> EditMessageText
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        EditMessageText::new(self.clone(), chat_id, message_id, text)
    }

    /// Use this method to edit text and game messages sent via the bot.
    ///
    /// On success, [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagetext).
    ///
    /// [`True`]: crate::types::True
    ///
    /// # Params
    ///
    ///  - `inline_message_id`: Identifier of the inline message.
    ///  - `text`: New text of the message.
    ///
    /// # Notes
    ///
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn edit_inline_message_text<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> EditInlineMessageText
    where
        I: Into<String>,
        T: Into<String>,
    {
        EditInlineMessageText::new(self.clone(), inline_message_id, text)
    }

    /// Use this method to edit captions of messages sent via the bot.
    ///
    /// On success, [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagecaption).
    ///
    /// [`True`]: crate::types::True
    ///
    /// # Notes
    ///
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn edit_message_caption<C>(&self, chat_id: C, message_id: i32) -> EditMessageCaption
    where
        C: Into<ChatId>,
    {
        EditMessageCaption::new(self.clone(), chat_id, message_id)
    }

    /// Use this method to edit captions of messages sent via the bot.
    ///
    /// On success, [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagecaption).
    ///
    /// [`True`]: crate::types::True
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn edit_inline_message_caption<I>(&self, inline_message_id: I) -> EditInlineMessageCaption
    where
        I: Into<String>,
    {
        EditInlineMessageCaption::new(self.clone(), inline_message_id)
    }

    /// Use this method to edit animation, audio, document, photo, or video
    /// messages.
    ///
    /// If a message is a part of a message album, then it can be edited only to
    /// a photo or a video. Otherwise, message type can be changed
    /// arbitrarily. On success, the edited [`Message`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagemedia).
    ///
    /// [`Message`]: crate::types::Message
    pub fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: i32,
        media: InputMedia,
    ) -> EditMessageMedia
    where
        C: Into<ChatId>,
    {
        EditMessageMedia::new(self.clone(), chat_id, message_id, media)
    }

    /// Use this method to edit animation, audio, document, photo, or video
    /// messages sent via the bot.
    ///
    /// If a message is a part of a message album, then it can be edited only to
    /// a photo or a video. Otherwise, message type can be changed
    /// arbitrarily. When this method is used, new file can't be uploaded.
    /// Use previously uploaded file via its `file_id` or specify a URL. On
    /// success, [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagemedia).
    ///
    /// [`True`]: crate::types::True
    pub fn edit_inline_message_media<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> EditInlineMessageMedia
    where
        I: Into<String>,
    {
        EditInlineMessageMedia::new(self.clone(), inline_message_id, media)
    }

    /// Use this method to edit only the reply markup of messages.
    ///
    /// On success, the edited [`Message`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagereplymarkup).
    ///
    /// [`Message`]: crate::types::Message
    pub fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> EditMessageReplyMarkup
    where
        C: Into<ChatId>,
    {
        EditMessageReplyMarkup::new(self.clone(), chat_id, message_id)
    }

    /// Use this method to edit only the reply markup of messages sent via the
    /// bot.
    ///
    /// On success, [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagereplymarkup).
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    pub fn edit_inline_message_reply_markup<I>(
        &self,
        inline_message_id: I,
    ) -> EditInlineMessageReplyMarkup
    where
        I: Into<String>,
    {
        EditInlineMessageReplyMarkup::new(self.clone(), inline_message_id)
    }

    /// Use this method to stop a poll which was sent by the bot.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#stoppoll).
    ///
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target channel (in the format `@channelusername`).
    ///   - `message_id`: Identifier of the original message with the poll.
    pub fn stop_poll<C>(&self, chat_id: C, message_id: i32) -> StopPoll
    where
        C: Into<ChatId>,
    {
        StopPoll::new(self.clone(), chat_id, message_id)
    }

    /// Use this method to delete a message, including service messages.
    ///
    /// The limitations are:
    ///  - A message can only be deleted if it was sent less than 48 hours ago.
    ///  - Bots can delete outgoing messages in private chats, groups, and
    ///    supergroups.
    ///  - Bots can delete incoming messages in private chats.
    ///  - Bots granted can_post_messages permissions can delete outgoing
    ///    messages in channels.
    ///  - If the bot is an administrator of a group, it can delete any message
    ///    there.
    ///  - If the bot has can_delete_messages permission in a supergroup or a
    ///    channel, it can delete any message there.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#deletemessage).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target channel (in the format `@channelusername`).
    ///   - `message_id`: Identifier of the message to delete.
    pub fn delete_message<C>(&self, chat_id: C, message_id: i32) -> DeleteMessage
    where
        C: Into<ChatId>,
    {
        DeleteMessage::new(self.clone(), chat_id, message_id)
    }

    /// Use this method to send static .WEBP or [animated] .TGS stickers.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendsticker).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target channel (in the format `@channelusername`).
    ///   - `sticker`: Sticker to send.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on the Telegram
    /// servers (recommended), pass an [`InputFile::Url`] for Telegram to get a
    /// .webp file from the Internet, or upload a new one using
    /// [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [animated]: https://telegram.org/blog/animated-stickers
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> SendSticker
    where
        C: Into<ChatId>,
    {
        SendSticker::new(self.clone(), chat_id, sticker)
    }

    /// Use this method to get a sticker set.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getstickerset).
    ///
    /// # Params
    ///   - `name`: Name of the sticker set.
    pub fn get_sticker_set<N>(&self, name: N) -> GetStickerSet
    where
        N: Into<String>,
    {
        GetStickerSet::new(self.clone(), name)
    }

    /// Use this method to upload a .png file with a sticker for later use in
    /// [`Bot::create_new_sticker_set`] and [`Bot::add_sticker_to_set`] methods
    /// (can be used multiple times).
    ///
    /// [The official docs](https://core.telegram.org/bots/api#uploadstickerfile).
    ///
    /// # Params
    ///   - `user_id`: User identifier of sticker file owner.
    ///   - `png_sticker`: **Png** image with the sticker, must be up to 512
    ///     kilobytes in size, dimensions must not exceed 512px, and either
    ///     width or height must be exactly 512px. [More info on Sending Files
    ///     »].
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    /// [`Bot::create_new_sticker_set`]: crate::Bot::create_new_sticker_set
    /// [`Bot::add_sticker_to_set`]: crate::Bot::add_sticker_to_set
    pub fn upload_sticker_file(&self, user_id: i32, png_sticker: InputFile) -> UploadStickerFile {
        UploadStickerFile::new(self.clone(), user_id, png_sticker)
    }

    /// Use this method to create new sticker set owned by a user. The bot will
    /// be able to edit the created sticker set.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#createnewstickerset).
    ///
    /// # Params
    ///   - `user_id`: User identifier of created sticker set owner.
    ///   - `name`: Short name of sticker set, to be used in `t.me/addstickers/`
    ///     URLs (e.g., animals). Can contain only english letters, digits and
    ///     underscores.
    ///
    /// Must begin with a letter, can't contain consecutive underscores and must
    /// end in `_by_<bot username>`. `<bot_username>` is case insensitive. 1-64
    /// characters.
    ///   - `title`: Sticker set title, 1-64 characters.
    pub fn create_new_sticker_set<N, T, E>(
        &self,
        user_id: i32,
        name: N,
        title: T,
        sticker_type: InputSticker,
        emojis: E,
    ) -> CreateNewStickerSet
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        CreateNewStickerSet::new(self.clone(), user_id, name, title, sticker_type, emojis)
    }

    /// Use this method to add a new sticker to a set created by the bot.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#addstickertoset).
    ///
    /// # Params
    ///   - `user_id`: User identifier of sticker set owner.
    ///   - `name`: Sticker set name.
    ///   - `emojis`: One or more emoji corresponding to the sticker.
    pub fn add_sticker_to_set<N, E>(
        &self,
        user_id: i32,
        name: N,
        sticker_type: InputSticker,
        emojis: E,
    ) -> AddStickerToSet
    where
        N: Into<String>,
        E: Into<String>,
    {
        AddStickerToSet::new(self.clone(), user_id, name, sticker_type, emojis)
    }

    /// Use this method to move a sticker in a set created by the bot to a
    /// specific position.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setstickerpositioninset).
    ///
    /// # Params
    ///   - `sticker`: File identifier of the sticker.
    ///   - `position`: New sticker position in the set, zero-based.
    pub fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: i32,
    ) -> SetStickerPositionInSet
    where
        S: Into<String>,
    {
        SetStickerPositionInSet::new(self.clone(), sticker, position)
    }

    /// Use this method to delete a sticker from a set created by the bot.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#deletestickerfromset).
    ///
    /// # Params
    ///   - `sticker`: File identifier of the sticker.
    pub fn delete_sticker_from_set<S>(&self, sticker: S) -> DeleteStickerFromSet
    where
        S: Into<String>,
    {
        DeleteStickerFromSet::new(self.clone(), sticker)
    }

    /// Use this method to send answers to an inline query.
    ///
    /// No more than **50** results per query are allowed.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#answerinlinequery).
    ///
    /// # Params
    ///   - `inline_query_id`: Unique identifier for the answered query.
    ///   - `results`: A JSON-serialized array of results for the inline query.
    pub fn answer_inline_query<I, R>(&self, inline_query_id: I, results: R) -> AnswerInlineQuery
    where
        I: Into<String>,
        R: Into<Vec<InlineQueryResult>>,
    {
        AnswerInlineQuery::new(self.clone(), inline_query_id, results)
    }

    /// Use this method to send invoices.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendinvoice).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target private chat.
    ///   - `title`: Product name, 1-32 characters.
    ///   - `description`: Product description, 1-255 characters.
    ///   - `payload`: Bot-defined invoice payload, 1-128 bytes. This will not
    ///     be displayed to the user, use for your internal processes.
    ///   - `provider_token`: Payments provider token, obtained via
    ///     [@Botfather].
    ///   - `start_parameter`: Unique deep-linking parameter that can be used to
    ///     generate this invoice when used as a start parameter.
    ///   - `currency`: Three-letter ISO 4217 currency code, see [more on
    ///     currencies].
    ///   - `prices`: Price breakdown, a list of components (e.g. product price,
    ///     tax, discount, delivery cost, delivery tax, bonus, etc.).
    ///
    /// [more on currencies]: https://core.telegram.org/bots/payments#supported-currencies
    /// [@Botfather]: https://t.me/botfather
    #[allow(clippy::too_many_arguments)]
    pub fn send_invoice<T, D, Pl, Pt, S, C, Pr>(
        &self,
        chat_id: i32,
        title: T,
        description: D,
        payload: Pl,
        provider_token: Pt,
        start_parameter: S,
        currency: C,
        prices: Pr,
    ) -> SendInvoice
    where
        T: Into<String>,
        D: Into<String>,
        Pl: Into<String>,
        Pt: Into<String>,
        S: Into<String>,
        C: Into<String>,
        Pr: Into<Vec<LabeledPrice>>,
    {
        SendInvoice::new(
            self.clone(),
            chat_id,
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
        )
    }

    /// Once the user has confirmed their payment and shipping details, the Bot
    /// API sends the final confirmation in the form of an [`Update`] with
    /// the field `pre_checkout_query`. Use this method to respond to such
    /// pre-checkout queries. Note: The Bot API must receive an answer
    /// within 10 seconds after the pre-checkout query was sent.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#answerprecheckoutquery).
    ///
    /// # Params
    ///   - `shipping_query_id`: Unique identifier for the query to be answered.
    ///   - `ok`: Specify `true` if delivery to the specified address is
    ///     possible and `false` if there are any problems (for example, if
    ///     delivery to the specified address is not possible).
    ///
    /// [`Update`]: crate::types::Update
    pub fn answer_shipping_query<S>(&self, shipping_query_id: S, ok: bool) -> AnswerShippingQuery
    where
        S: Into<String>,
    {
        AnswerShippingQuery::new(self.clone(), shipping_query_id, ok)
    }

    /// Once the user has confirmed their payment and shipping details, the Bot
    /// API sends the final confirmation in the form of an [`Update`] with
    /// the field `pre_checkout_query`. Use this method to respond to such
    /// pre-checkout queries. Note: The Bot API must receive an answer
    /// within 10 seconds after the pre-checkout query was sent.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#answerprecheckoutquery).
    ///
    /// # Params
    ///   - `pre_checkout_query_id`: Unique identifier for the query to be
    ///     answered.
    ///   - `ok`: Specify `true` if everything is alright (goods are available,
    ///     etc.) and the bot is ready to proceed with the order. Use False if
    ///     there are any problems.
    ///
    /// [`Update`]: crate::types::Update
    pub fn answer_pre_checkout_query<P>(
        &self,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> AnswerPreCheckoutQuery
    where
        P: Into<String>,
    {
        AnswerPreCheckoutQuery::new(self.clone(), pre_checkout_query_id, ok)
    }

    /// Use this method to send a game.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#sendgame).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat.
    ///   - `game_short_name`: Short name of the game, serves as the unique
    ///     identifier for the game. Set up your games via [@Botfather].
    ///
    /// [@Botfather]: https://t.me/botfather
    pub fn send_game<G>(&self, chat_id: i32, game_short_name: G) -> SendGame
    where
        G: Into<String>,
    {
        SendGame::new(self.clone(), chat_id, game_short_name)
    }

    /// Use this method to set the score of the specified user in a game.
    ///
    /// On success, if the message was sent by the bot, returns the edited
    /// [`Message`], otherwise returns [`True`]. Returns an error, if the new
    /// score is not greater than the user's current score in the chat and
    /// force is `false`.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setgamescore).
    ///
    /// # Params
    ///   - `target`: Target message, either chat id and message id or inline
    ///     message id.
    ///   - `user_id`: User identifier.
    ///   - `score`: New score, must be non-negative.
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    pub fn set_game_score<T>(&self, target: T, user_id: i32, score: i32) -> SetGameScore
    where
        T: Into<TargetMessage>,
    {
        SetGameScore::new(self.clone(), target, user_id, score)
    }

    /// Use this method to get data for high score tables.
    ///
    /// Will return the score of the specified user and several of his neighbors
    /// in a game.
    ///
    /// # Note
    /// This method will currently return scores for the target user, plus two
    /// of his closest neighbors on each side. Will also return the top
    /// three users if the user and his neighbors are not among them. Please
    /// note that this behavior is subject to change.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getgamehighscores).
    ///
    /// # Params
    ///   - `target`: Target message, either chat id and message id or inline
    ///     message id.
    ///   - `user_id`: Target user id.
    pub fn get_game_high_scores<T>(&self, target: T, user_id: i32) -> GetGameHighScores
    where
        T: Into<TargetMessage>,
    {
        GetGameHighScores::new(self.clone(), target, user_id)
    }

    /// Use this method to set a custom title for an administrator in a
    /// supergroup promoted by the bot.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setchatadministratorcustomtitle).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target channel (in the format `@channelusername`).
    ///   - `user_id`: Unique identifier of the target user.
    ///   - `custom_title`: New custom title for the administrator; 0-16
    ///     characters, emoji are not allowed.
    pub fn set_chat_administrator_custom_title<C, CT>(
        &self,
        chat_id: C,
        user_id: i32,
        custom_title: CT,
    ) -> SetChatAdministratorCustomTitle
    where
        C: Into<ChatId>,
        CT: Into<String>,
    {
        SetChatAdministratorCustomTitle::new(self.clone(), chat_id, user_id, custom_title)
    }

    /// Use this method to send an animated emoji that will display a random
    /// value.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#senddice).
    ///
    /// # Params
    ///   - `chat_id`: Unique identifier for the target chat or username of the
    ///     target channel  (in the format `@channelusername`).
    pub fn send_dice<C>(&self, chat_id: C) -> SendDice
    where
        C: Into<ChatId>,
    {
        SendDice::new(self.clone(), chat_id)
    }

    /// Use this method to get the current list of the bot's commands.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#getmycommands).
    pub fn get_my_commands(&self) -> GetMyCommands {
        GetMyCommands::new(self.clone())
    }

    /// Use this method to change the list of the bot's commands.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setmycommands).
    ///
    /// # Params
    ///    - `commands`: A JSON-serialized list of bot commands to be set as the
    ///      list of the bot's commands. At most 100 commands can be specified.
    pub fn set_my_commands<C>(&self, commands: C) -> SetMyCommands
    where
        C: Into<Vec<BotCommand>>,
    {
        SetMyCommands::new(self.clone(), commands)
    }

    /// Use this method to set the thumbnail of a sticker set. Animated
    /// thumbnails can be set for animated sticker sets only.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#setstickersetthumb).
    ///
    /// # Params
    ///    - `name`: Sticker set name.
    ///    - `user_id`: User identifier of the sticker set owner.
    pub fn set_sticker_set_thumb<S>(&self, name: S, user_id: i32) -> SetStickerSetThumb
    where
        S: Into<String>,
    {
        SetStickerSetThumb::new(self.clone(), name, user_id)
    }
}

impl Requester for Bot {
    type Err = crate::errors::RequestError;

    type GetUpdates = JsonRequest<payloads::GetUpdates>;

    fn get_updates(&self) -> Self::GetUpdates {
        Self::GetUpdates::new(self.clone(), payloads::GetUpdates::new())
    }

    type SetWebhook = JsonRequest<payloads::SetWebhook>;

    fn set_webhook<U, A>(&self, url: U, allowed_updates: A) -> Self::SetWebhook
    where
        U: Into<String>,
        A: IntoIterator<Item = crate::types::AllowedUpdate>,
    {
        Self::SetWebhook::new(self.clone(), payloads::SetWebhook::new(url, allowed_updates))
    }

    type DeleteWebhook = JsonRequest<payloads::DeleteWebhook>;

    fn delete_webhook(&self) -> Self::DeleteWebhook {
        Self::DeleteWebhook::new(self.clone(), payloads::DeleteWebhook::new())
    }

    type GetWebhookInfo = JsonRequest<payloads::GetWebhookInfo>;

    fn get_webhook_info(&self) -> Self::GetWebhookInfo {
        Self::GetWebhookInfo::new(self.clone(), payloads::GetWebhookInfo::new())
    }

    type GetMe = JsonRequest<payloads::GetMe>;

    fn get_me(&self) -> Self::GetMe {
        Self::GetMe::new(self.clone(), payloads::GetMe::new())
    }

    type SendMessage = JsonRequest<payloads::SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::SendMessage::new(self.clone(), payloads::SendMessage::new(chat_id, text))
    }

    type ForwardMessage = JsonRequest<payloads::ForwardMessage>;

    fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: i32,
    ) -> Self::ForwardMessage
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        Self::ForwardMessage::new(
            self.clone(),
            payloads::ForwardMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type SendPhoto = MultipartRequest<payloads::SendPhoto>;

    fn send_photo<Ch, Ca>(&self, chat_id: Ch, photo: InputFile, caption: Ca) -> Self::SendPhoto
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendPhoto::new(self.clone(), payloads::SendPhoto::new(chat_id, photo, caption))
    }

    type SendAudio = MultipartRequest<payloads::SendAudio>;

    fn send_audio<Ch, Ca>(&self, chat_id: Ch, audio: InputFile, caption: Ca) -> Self::SendAudio
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendAudio::new(self.clone(), payloads::SendAudio::new(chat_id, audio, caption))
    }

    type SendDocument = MultipartRequest<payloads::SendDocument>;

    fn send_document<Ch, Ca>(
        &self,
        chat_id: Ch,
        document: InputFile,
        caption: Ca,
    ) -> Self::SendDocument
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendDocument::new(
            self.clone(),
            payloads::SendDocument::new(chat_id, document, caption),
        )
    }

    type SendVideo = MultipartRequest<payloads::SendVideo>;

    fn send_video<Ch, Ca>(&self, chat_id: Ch, video: InputFile, caption: Ca) -> Self::SendVideo
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendVideo::new(self.clone(), payloads::SendVideo::new(chat_id, video, caption))
    }

    type SendAnimation = MultipartRequest<payloads::SendAnimation>;

    fn send_animation<Ch, Ca>(
        &self,
        chat_id: Ch,
        animation: InputFile,
        caption: Ca,
    ) -> Self::SendAnimation
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendAnimation::new(
            self.clone(),
            payloads::SendAnimation::new(chat_id, animation, caption),
        )
    }

    type SendVoice = MultipartRequest<payloads::SendVoice>;

    fn send_voice<Ch, Ca>(&self, chat_id: Ch, voice: InputFile, caption: Ca) -> Self::SendVoice
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::SendVoice::new(self.clone(), payloads::SendVoice::new(chat_id, voice, caption))
    }

    type SendVideoNote = MultipartRequest<payloads::SendVideoNote>;

    fn send_video_note<C>(&self, chat_id: C, video_note: InputFile) -> Self::SendVideoNote
    where
        C: Into<ChatId>,
    {
        Self::SendVideoNote::new(self.clone(), payloads::SendVideoNote::new(chat_id, video_note))
    }

    type SendMediaGroup = MultipartRequest<payloads::SendMediaGroup>;

    fn send_media_group<C, M>(&self, chat_id: C, media: M) -> Self::SendMediaGroup
    where
        C: Into<ChatId>,
        M: IntoIterator<Item = InputMedia>,
    {
        Self::SendMediaGroup::new(self.clone(), payloads::SendMediaGroup::new(chat_id, media))
    }

    type SendLocation = JsonRequest<payloads::SendLocation>;

    fn send_location<C>(
        &self,
        chat_id: C,
        latitude: f64,
        longitude: f64,
        live_period: u32,
    ) -> Self::SendLocation
    where
        C: Into<ChatId>,
    {
        Self::SendLocation::new(
            self.clone(),
            payloads::SendLocation::new(chat_id, latitude, longitude, live_period),
        )
    }

    type EditMessageLiveLocation = JsonRequest<payloads::EditMessageLiveLocation>;

    fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        Self::EditMessageLiveLocation::new(
            self.clone(),
            payloads::EditMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocationInline = JsonRequest<payloads::EditMessageLiveLocationInline>;

    fn edit_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::EditMessageLiveLocationInline::new(
            self.clone(),
            payloads::EditMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocation = JsonRequest<payloads::StopMessageLiveLocation>;

    fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        Self::StopMessageLiveLocation::new(
            self.clone(),
            payloads::StopMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocationInline = JsonRequest<payloads::StopMessageLiveLocationInline>;

    fn stop_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::StopMessageLiveLocationInline::new(
            self.clone(),
            payloads::StopMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type SendVenue = JsonRequest<payloads::SendVenue>;

    fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f64,
        longitude: f64,
        title: T,
        address: A,
    ) -> Self::SendVenue
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        Self::SendVenue::new(
            self.clone(),
            payloads::SendVenue::new(chat_id, latitude, longitude, title, address),
        )
    }

    type SendContact = JsonRequest<payloads::SendContact>;

    fn send_contact<C>(&self, chat_id: C, phone_number: f64, first_name: f64) -> Self::SendContact
    where
        C: Into<ChatId>,
    {
        Self::SendContact::new(
            self.clone(),
            payloads::SendContact::new(chat_id, phone_number, first_name),
        )
    }

    type SendPoll = JsonRequest<payloads::SendPoll>;

    fn send_poll<C, Q, O>(
        &self,
        chat_id: C,
        question: Q,
        options: O,
        type_: crate::types::PollType,
    ) -> Self::SendPoll
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: IntoIterator<Item = String>,
    {
        Self::SendPoll::new(
            self.clone(),
            payloads::SendPoll::new(chat_id, question, options, type_),
        )
    }

    type SendDice = JsonRequest<payloads::SendDice>;

    fn send_dice<C>(&self, chat_id: C, emoji: crate::types::DiceEmoji) -> Self::SendDice
    where
        C: Into<ChatId>,
    {
        Self::SendDice::new(self.clone(), payloads::SendDice::new(chat_id, emoji))
    }

    type SendChatAction = JsonRequest<payloads::SendChatAction>;

    fn send_chat_action<C>(
        &self,
        chat_id: C,
        action: crate::types::ChatAction,
    ) -> Self::SendChatAction
    where
        C: Into<ChatId>,
    {
        Self::SendChatAction::new(self.clone(), payloads::SendChatAction::new(chat_id, action))
    }

    type GetUserProfilePhotos = JsonRequest<payloads::GetUserProfilePhotos>;

    fn get_user_profile_photos(&self, user_id: i32) -> Self::GetUserProfilePhotos {
        Self::GetUserProfilePhotos::new(self.clone(), payloads::GetUserProfilePhotos::new(user_id))
    }

    type GetFile = JsonRequest<payloads::GetFile>;

    fn get_file<F>(&self, file_id: F) -> Self::GetFile
    where
        F: Into<String>,
    {
        Self::GetFile::new(self.clone(), payloads::GetFile::new(file_id))
    }

    type KickChatMember = JsonRequest<payloads::KickChatMember>;

    fn kick_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::KickChatMember
    where
        C: Into<ChatId>,
    {
        Self::KickChatMember::new(self.clone(), payloads::KickChatMember::new(chat_id, user_id))
    }

    type UnbanChatMember = JsonRequest<payloads::UnbanChatMember>;

    fn unban_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::UnbanChatMember
    where
        C: Into<ChatId>,
    {
        Self::UnbanChatMember::new(self.clone(), payloads::UnbanChatMember::new(chat_id, user_id))
    }

    type RestrictChatMember = JsonRequest<payloads::RestrictChatMember>;

    fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> Self::RestrictChatMember
    where
        C: Into<ChatId>,
    {
        Self::RestrictChatMember::new(
            self.clone(),
            payloads::RestrictChatMember::new(chat_id, user_id, permissions),
        )
    }

    type PromoteChatMember = JsonRequest<payloads::PromoteChatMember>;

    fn promote_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::PromoteChatMember
    where
        C: Into<ChatId>,
    {
        Self::PromoteChatMember::new(
            self.clone(),
            payloads::PromoteChatMember::new(chat_id, user_id),
        )
    }

    type SetChatAdministratorCustomTitle = JsonRequest<payloads::SetChatAdministratorCustomTitle>;

    fn set_chat_administrator_custom_title<Ch, Cu>(
        &self,
        chat_id: Ch,
        user_id: i32,
        custom_title: Cu,
    ) -> Self::SetChatAdministratorCustomTitle
    where
        Ch: Into<ChatId>,
        Cu: Into<String>,
    {
        Self::SetChatAdministratorCustomTitle::new(
            self.clone(),
            payloads::SetChatAdministratorCustomTitle::new(chat_id, user_id, custom_title),
        )
    }

    type SetChatPermissions = JsonRequest<payloads::SetChatPermissions>;

    fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> Self::SetChatPermissions
    where
        C: Into<ChatId>,
    {
        Self::SetChatPermissions::new(
            self.clone(),
            payloads::SetChatPermissions::new(chat_id, permissions),
        )
    }

    type ExportChatInviteLink = JsonRequest<payloads::ExportChatInviteLink>;

    fn export_chat_invite_link<C>(&self, chat_id: C) -> Self::ExportChatInviteLink
    where
        C: Into<ChatId>,
    {
        Self::ExportChatInviteLink::new(self.clone(), payloads::ExportChatInviteLink::new(chat_id))
    }

    type SetChatPhoto = MultipartRequest<payloads::SetChatPhoto>;

    fn set_chat_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SetChatPhoto
    where
        C: Into<ChatId>,
    {
        Self::SetChatPhoto::new(self.clone(), payloads::SetChatPhoto::new(chat_id, photo))
    }

    type DeleteChatPhoto = JsonRequest<payloads::DeleteChatPhoto>;

    fn delete_chat_photo<C>(&self, chat_id: C) -> Self::DeleteChatPhoto
    where
        C: Into<ChatId>,
    {
        Self::DeleteChatPhoto::new(self.clone(), payloads::DeleteChatPhoto::new(chat_id))
    }

    type SetChatTitle = JsonRequest<payloads::SetChatTitle>;

    fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> Self::SetChatTitle
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::SetChatTitle::new(self.clone(), payloads::SetChatTitle::new(chat_id, title))
    }

    type SetChatDescription = JsonRequest<payloads::SetChatDescription>;

    fn set_chat_description<C>(&self, chat_id: C) -> Self::SetChatDescription
    where
        C: Into<ChatId>,
    {
        Self::SetChatDescription::new(self.clone(), payloads::SetChatDescription::new(chat_id))
    }

    type PinChatMessage = JsonRequest<payloads::PinChatMessage>;

    fn pin_chat_message<C>(&self, chat_id: C, message_id: i32) -> Self::PinChatMessage
    where
        C: Into<ChatId>,
    {
        Self::PinChatMessage::new(self.clone(), payloads::PinChatMessage::new(chat_id, message_id))
    }

    type UnpinChatMessage = JsonRequest<payloads::UnpinChatMessage>;

    fn unpin_chat_message<C>(&self, chat_id: C) -> Self::UnpinChatMessage
    where
        C: Into<ChatId>,
    {
        Self::UnpinChatMessage::new(self.clone(), payloads::UnpinChatMessage::new(chat_id))
    }

    type LeaveChat = JsonRequest<payloads::LeaveChat>;

    fn leave_chat<C>(&self, chat_id: C) -> Self::LeaveChat
    where
        C: Into<ChatId>,
    {
        Self::LeaveChat::new(self.clone(), payloads::LeaveChat::new(chat_id))
    }

    type GetChat = JsonRequest<payloads::GetChat>;

    fn get_chat<C>(&self, chat_id: C) -> Self::GetChat
    where
        C: Into<ChatId>,
    {
        Self::GetChat::new(self.clone(), payloads::GetChat::new(chat_id))
    }

    type GetChatAdministrators = JsonRequest<payloads::GetChatAdministrators>;

    fn get_chat_administrators<C>(&self, chat_id: C) -> Self::GetChatAdministrators
    where
        C: Into<ChatId>,
    {
        Self::GetChatAdministrators::new(
            self.clone(),
            payloads::GetChatAdministrators::new(chat_id),
        )
    }

    type GetChatMembersCount = JsonRequest<payloads::GetChatMembersCount>;

    fn get_chat_members_count<C>(&self, chat_id: C) -> Self::GetChatMembersCount
    where
        C: Into<ChatId>,
    {
        Self::GetChatMembersCount::new(self.clone(), payloads::GetChatMembersCount::new(chat_id))
    }

    type GetChatMember = JsonRequest<payloads::GetChatMember>;

    fn get_chat_member<C>(&self, chat_id: C, user_id: i32) -> Self::GetChatMember
    where
        C: Into<ChatId>,
    {
        Self::GetChatMember::new(self.clone(), payloads::GetChatMember::new(chat_id, user_id))
    }

    type SetChatStickerSet = JsonRequest<payloads::SetChatStickerSet>;

    fn set_chat_sticker_set<C, S>(&self, chat_id: C, sticker_set_name: S) -> Self::SetChatStickerSet
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        Self::SetChatStickerSet::new(
            self.clone(),
            payloads::SetChatStickerSet::new(chat_id, sticker_set_name),
        )
    }

    type DeleteChatStickerSet = JsonRequest<payloads::DeleteChatStickerSet>;

    fn delete_chat_sticker_set<C>(&self, chat_id: C) -> Self::DeleteChatStickerSet
    where
        C: Into<ChatId>,
    {
        Self::DeleteChatStickerSet::new(self.clone(), payloads::DeleteChatStickerSet::new(chat_id))
    }

    type AnswerCallbackQuery = JsonRequest<payloads::AnswerCallbackQuery>;

    fn answer_callback_query<C>(&self, callback_query_id: C) -> Self::AnswerCallbackQuery
    where
        C: Into<String>,
    {
        Self::AnswerCallbackQuery::new(
            self.clone(),
            payloads::AnswerCallbackQuery::new(callback_query_id),
        )
    }

    type SetMyCommands = JsonRequest<payloads::SetMyCommands>;

    fn set_my_commands<C>(&self, commands: C) -> Self::SetMyCommands
    where
        C: IntoIterator<Item = BotCommand>,
    {
        Self::SetMyCommands::new(self.clone(), payloads::SetMyCommands::new(commands))
    }

    type GetMyCommands = JsonRequest<payloads::GetMyCommands>;

    fn get_my_commands(&self) -> Self::GetMyCommands {
        Self::GetMyCommands::new(self.clone(), payloads::GetMyCommands::new())
    }

    type AnswerInlineQuery = JsonRequest<payloads::AnswerInlineQuery>;

    fn answer_inline_query<I, R>(&self, inline_query_id: I, results: R) -> Self::AnswerInlineQuery
    where
        I: Into<String>,
        R: IntoIterator<Item = InlineQueryResult>,
    {
        Self::AnswerInlineQuery::new(
            self.clone(),
            payloads::AnswerInlineQuery::new(inline_query_id, results),
        )
    }

    type EditMessageText = JsonRequest<payloads::EditMessageText>;

    fn edit_message_text<C, T>(&self, chat_id: C, message_id: i32, text: T) -> Self::EditMessageText
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::EditMessageText::new(
            self.clone(),
            payloads::EditMessageText::new(chat_id, message_id, text),
        )
    }

    type EditMessageTextInline = JsonRequest<payloads::EditMessageTextInline>;

    fn edit_message_text_inline<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> Self::EditMessageTextInline
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self::EditMessageTextInline::new(
            self.clone(),
            payloads::EditMessageTextInline::new(inline_message_id, text),
        )
    }

    type EditMessageCaption = JsonRequest<payloads::EditMessageCaption>;

    fn edit_message_caption<Ch, Ca>(
        &self,
        chat_id: Ch,
        message_id: i32,
        caption: Ca,
    ) -> Self::EditMessageCaption
    where
        Ch: Into<ChatId>,
        Ca: Into<String>,
    {
        Self::EditMessageCaption::new(
            self.clone(),
            payloads::EditMessageCaption::new(chat_id, message_id, caption),
        )
    }

    type EditMessageCaptionInline = JsonRequest<payloads::EditMessageCaptionInline>;

    fn edit_message_caption_inline<I, C>(
        &self,
        inline_message_id: I,
        caption: C,
    ) -> Self::EditMessageCaptionInline
    where
        I: Into<String>,
        C: Into<String>,
    {
        Self::EditMessageCaptionInline::new(
            self.clone(),
            payloads::EditMessageCaptionInline::new(inline_message_id, caption),
        )
    }

    type EditMessageMedia = MultipartRequest<payloads::EditMessageMedia>;

    fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: i32,
        media: InputMedia,
    ) -> Self::EditMessageMedia
    where
        C: Into<ChatId>,
    {
        Self::EditMessageMedia::new(
            self.clone(),
            payloads::EditMessageMedia::new(chat_id, message_id, media),
        )
    }

    type EditMessageMediaInline = MultipartRequest<payloads::EditMessageMediaInline>;

    fn edit_message_media_inline<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> Self::EditMessageMediaInline
    where
        I: Into<String>,
    {
        Self::EditMessageMediaInline::new(
            self.clone(),
            payloads::EditMessageMediaInline::new(inline_message_id, media),
        )
    }

    type EditMessageReplyMarkup = JsonRequest<payloads::EditMessageReplyMarkup>;

    fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> Self::EditMessageReplyMarkup
    where
        C: Into<ChatId>,
    {
        Self::EditMessageReplyMarkup::new(
            self.clone(),
            payloads::EditMessageReplyMarkup::new(chat_id, message_id),
        )
    }

    type EditMessageReplyMarkupInline = JsonRequest<payloads::EditMessageReplyMarkupInline>;

    fn edit_message_reply_markup_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::EditMessageReplyMarkupInline
    where
        I: Into<String>,
    {
        Self::EditMessageReplyMarkupInline::new(
            self.clone(),
            payloads::EditMessageReplyMarkupInline::new(inline_message_id),
        )
    }

    type StopPoll = JsonRequest<payloads::StopPoll>;

    fn stop_poll<C>(&self, chat_id: C, message_id: i32) -> Self::StopPoll
    where
        C: Into<ChatId>,
    {
        Self::StopPoll::new(self.clone(), payloads::StopPoll::new(chat_id, message_id))
    }

    type DeleteMessage = JsonRequest<payloads::DeleteMessage>;

    fn delete_message<C>(&self, chat_id: C, message_id: i32) -> Self::DeleteMessage
    where
        C: Into<ChatId>,
    {
        Self::DeleteMessage::new(self.clone(), payloads::DeleteMessage::new(chat_id, message_id))
    }

    type SendSticker = MultipartRequest<payloads::SendSticker>;

    fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> Self::SendSticker
    where
        C: Into<ChatId>,
    {
        Self::SendSticker::new(self.clone(), payloads::SendSticker::new(chat_id, sticker))
    }

    type GetStickerSet = JsonRequest<payloads::GetStickerSet>;

    fn get_sticker_set<N>(&self, name: N) -> Self::GetStickerSet
    where
        N: Into<String>,
    {
        Self::GetStickerSet::new(self.clone(), payloads::GetStickerSet::new(name))
    }

    type UploadStickerFile = MultipartRequest<payloads::UploadStickerFile>;

    fn upload_sticker_file(&self, user_id: i32, png_sticker: InputFile) -> Self::UploadStickerFile where
    {
        Self::UploadStickerFile::new(
            self.clone(),
            payloads::UploadStickerFile::new(user_id, png_sticker),
        )
    }

    type CreateNewStickerSet = JsonRequest<payloads::CreateNewStickerSet>;

    fn create_new_sticker_set<N, T, E>(
        &self,
        user_id: i32,
        name: N,
        title: T,
        emojis: E,
    ) -> Self::CreateNewStickerSet
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        Self::CreateNewStickerSet::new(
            self.clone(),
            payloads::CreateNewStickerSet::new(user_id, name, title, emojis),
        )
    }

    type AddStickerToSet = MultipartRequest<payloads::AddStickerToSet>;

    fn add_sticker_to_set<N, E>(
        &self,
        user_id: i32,
        name: N,
        sticker: InputSticker,
        emojis: E,
    ) -> Self::AddStickerToSet
    where
        N: Into<String>,
        E: Into<String>,
    {
        Self::AddStickerToSet::new(
            self.clone(),
            payloads::AddStickerToSet::new(user_id, name, sticker, emojis),
        )
    }

    type SetStickerPositionInSet = JsonRequest<payloads::SetStickerPositionInSet>;

    fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: u32,
    ) -> Self::SetStickerPositionInSet
    where
        S: Into<String>,
    {
        Self::SetStickerPositionInSet::new(
            self.clone(),
            payloads::SetStickerPositionInSet::new(sticker, position),
        )
    }

    type DeleteStickerFromSet = JsonRequest<payloads::DeleteStickerFromSet>;

    fn delete_sticker_from_set<S>(&self, sticker: S) -> Self::DeleteStickerFromSet
    where
        S: Into<String>,
    {
        Self::DeleteStickerFromSet::new(self.clone(), payloads::DeleteStickerFromSet::new(sticker))
    }

    type SetStickerSetThumb = MultipartRequest<payloads::SetStickerSetThumb>;

    fn set_sticker_set_thumb<N>(&self, name: N, user_id: i32) -> Self::SetStickerSetThumb
    where
        N: Into<String>,
    {
        Self::SetStickerSetThumb::new(
            self.clone(),
            payloads::SetStickerSetThumb::new(name, user_id),
        )
    }

    type SendInvoice = JsonRequest<payloads::SendInvoice>;

    fn send_invoice<T, D, Pa, P, S, C, Pri>(
        &self,
        chat_id: i32,
        title: T,
        description: D,
        payload: Pa,
        provider_token: P,
        start_parameter: S,
        currency: C,
        prices: Pri,
    ) -> Self::SendInvoice
    where
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        P: Into<String>,
        S: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice>,
    {
        Self::SendInvoice::new(
            self.clone(),
            payloads::SendInvoice::new(
                chat_id,
                title,
                description,
                payload,
                provider_token,
                start_parameter,
                currency,
                prices,
            ),
        )
    }

    type AnswerShippingQuery = JsonRequest<payloads::AnswerShippingQuery>;

    fn answer_shipping_query<S>(&self, shipping_query_id: S, ok: bool) -> Self::AnswerShippingQuery
    where
        S: Into<String>,
    {
        Self::AnswerShippingQuery::new(
            self.clone(),
            payloads::AnswerShippingQuery::new(shipping_query_id, ok),
        )
    }

    type AnswerPreCheckoutQuery = JsonRequest<payloads::AnswerPreCheckoutQuery>;

    fn answer_pre_checkout_query<P>(
        &self,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> Self::AnswerPreCheckoutQuery
    where
        P: Into<String>,
    {
        Self::AnswerPreCheckoutQuery::new(
            self.clone(),
            payloads::AnswerPreCheckoutQuery::new(pre_checkout_query_id, ok),
        )
    }

    type SetPassportDataErrors = JsonRequest<payloads::SetPassportDataErrors>;

    fn set_passport_data_errors<E>(&self, user_id: i32, errors: E) -> Self::SetPassportDataErrors
    where
        E: IntoIterator<Item = crate::types::PassportElementError>,
    {
        Self::SetPassportDataErrors::new(
            self.clone(),
            payloads::SetPassportDataErrors::new(user_id, errors),
        )
    }
}
