use crate::{
    requests::{
        AddStickerToSet, AnswerCallbackQuery, AnswerInlineQuery, AnswerPreCheckoutQuery,
        AnswerShippingQuery, CreateNewStickerSet, DeleteChatPhoto, DeleteChatStickerSet,
        DeleteMessage, DeleteStickerFromSet, DeleteWebhook, EditMessageCaption,
        EditMessageLiveLocation, EditMessageMedia, EditMessageReplyMarkup, EditMessageText,
        ExportChatInviteLink, ForwardMessage, GetChat, GetChatAdministrators, GetChatMember,
        GetChatMembersCount, GetFile, GetGameHighScores, GetMe, GetMyCommands, GetStickerSet,
        GetUpdates, GetUserProfilePhotos, GetWebhookInfo, KickChatMember, LeaveChat,
        PinChatMessage, PromoteChatMember, RestrictChatMember, SendAnimation, SendAudio,
        SendChatAction, SendChatActionKind, SendContact, SendDice, SendDocument, SendGame,
        SendInvoice, SendLocation, SendMediaGroup, SendMessage, SendPhoto, SendPoll, SendSticker,
        SendVenue, SendVideo, SendVideoNote, SendVoice, SetChatAdministratorCustomTitle,
        SetChatDescription, SetChatPermissions, SetChatPhoto, SetChatStickerSet, SetChatTitle,
        SetGameScore, SetMyCommands, SetStickerPositionInSet, SetStickerSetThumb, SetWebhook,
        StopMessageLiveLocation, StopPoll, UnbanChatMember, UnpinChatMessage, UploadStickerFile,
    },
    types::{
        BotCommand, ChatId, ChatOrInlineMessage, ChatPermissions, InlineQueryResult, InputFile,
        InputMedia, LabeledPrice, ParseMode, StickerType,
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
        self.with_default_parse_mode_if_specified(
            SendMessage::new(self.clone(), chat_id, text),
            SendMessage::parse_mode,
        )
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
        self.with_default_parse_mode_if_specified(
            SendPhoto::new(self.clone(), chat_id, photo),
            SendPhoto::parse_mode,
        )
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
        self.with_default_parse_mode_if_specified(
            SendAudio::new(self.clone(), chat_id, audio),
            SendAudio::parse_mode,
        )
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
        self.with_default_parse_mode_if_specified(
            SendDocument::new(self.clone(), chat_id, document),
            SendDocument::parse_mode,
        )
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
        self.with_default_parse_mode_if_specified(
            SendVideo::new(self.clone(), chat_id, video),
            SendVideo::parse_mode,
        )
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
        self.with_default_parse_mode_if_specified(
            SendAnimation::new(self.clone(), chat_id, animation),
            SendAnimation::parse_mode,
        )
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
        self.with_default_parse_mode_if_specified(
            SendVoice::new(self.clone(), chat_id, voice),
            SendVoice::parse_mode,
        )
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
    /// explicitly disabled by a call to stopMessageLiveLocation. On success, if
    /// the edited message was sent by the bot, the edited [`Message`] is
    /// returned, otherwise [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagelivelocation).
    ///
    /// # Params
    ///   - `latitude`: Latitude of new location.
    ///   - `longitude`: Longitude of new location.
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    pub fn edit_message_live_location(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
        latitude: f32,
        longitude: f32,
    ) -> EditMessageLiveLocation {
        EditMessageLiveLocation::new(self.clone(), chat_or_inline_message, latitude, longitude)
    }

    /// Use this method to stop updating a live location message before
    /// `live_period` expires.
    ///
    /// On success, if the message was sent by the bot, the sent [`Message`] is
    /// returned, otherwise [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#stopmessagelivelocation).
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    pub fn stop_message_live_location(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
    ) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new(self.clone(), chat_or_inline_message)
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
        self.with_default_parse_mode_if_specified(
            SendPoll::new(self.clone(), chat_id, question, options),
            SendPoll::explanation_parse_mode,
        )
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
    /// On success, if edited message is sent by the bot, the edited [`Message`]
    /// is returned, otherwise [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagetext).
    ///
    /// # Params
    ///   - New text of the message.
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn edit_message_text<T>(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
        text: T,
    ) -> EditMessageText
    where
        T: Into<String>,
    {
        match self.parse_mode {
            None => EditMessageText::new(self.clone(), chat_or_inline_message, text),
            Some(parse_mode) => EditMessageText::new(self.clone(), chat_or_inline_message, text)
                .parse_mode(parse_mode),
        }
    }

    /// Use this method to edit captions of messages.
    ///
    /// On success, if edited message is sent by the bot, the edited [`Message`]
    /// is returned, otherwise [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagecaption).
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    ///
    /// # Notes
    /// Uses [a default parse mode] if specified in [`BotBuilder`].
    ///
    /// [a default parse mode]: crate::BotBuilder::parse_mode
    /// [`BotBuilder`]: crate::BotBuilder
    pub fn edit_message_caption(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
    ) -> EditMessageCaption {
        match self.parse_mode {
            None => EditMessageCaption::new(self.clone(), chat_or_inline_message),
            Some(parse_mode) => {
                EditMessageCaption::new(self.clone(), chat_or_inline_message).parse_mode(parse_mode)
            }
        }
    }

    /// Use this method to edit animation, audio, document, photo, or video
    /// messages.
    ///
    /// If a message is a part of a message album, then it can be edited only to
    /// a photo or a video. Otherwise, message type can be changed
    /// arbitrarily. When inline message is edited, new file can't be
    /// uploaded. Use previously uploaded file via its `file_id` or specify
    /// a URL. On success, if the edited message was sent by the bot, the
    /// edited [`Message`] is returned, otherwise [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagemedia).
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    pub fn edit_message_media(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
        media: InputMedia,
    ) -> EditMessageMedia {
        EditMessageMedia::new(self.clone(), chat_or_inline_message, media)
    }

    /// Use this method to edit only the reply markup of messages.
    ///
    /// On success, if edited message is sent by the bot, the edited [`Message`]
    /// is returned, otherwise [`True`] is returned.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#editmessagereplymarkup).
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    pub fn edit_message_reply_markup(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
    ) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::new(self.clone(), chat_or_inline_message)
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
        sticker_type: StickerType,
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
        sticker_type: StickerType,
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
    ///   - `user_id`: User identifier.
    ///   - `score`: New score, must be non-negative.
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    pub fn set_game_score(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
        user_id: i32,
        score: i32,
    ) -> SetGameScore {
        SetGameScore::new(self.clone(), chat_or_inline_message, user_id, score)
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
    ///   - `user_id`: Target user id.
    pub fn get_game_high_scores(
        &self,
        chat_or_inline_message: ChatOrInlineMessage,
        user_id: i32,
    ) -> GetGameHighScores {
        GetGameHighScores::new(self.clone(), chat_or_inline_message, user_id)
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

    fn with_default_parse_mode_if_specified<Builder>(
        &self,
        builder: Builder,
        f: fn(Builder, ParseMode) -> Builder,
    ) -> Builder {
        match self.parse_mode {
            None => builder,
            Some(parse_mode) => f(builder, parse_mode),
        }
    }
}
