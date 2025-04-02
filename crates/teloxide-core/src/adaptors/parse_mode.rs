use std::future::IntoFuture;

use url::Url;

use crate::{
    payloads::{
        AnswerInlineQuery, AnswerWebAppQuery, CopyMessage, EditMessageCaption,
        EditMessageCaptionInline, EditMessageMedia, EditMessageMediaInline, EditMessageText,
        EditMessageTextInline, SendAnimation, SendAudio, SendDocument, SendMediaGroup, SendMessage,
        SendPhoto, SendPoll, SendVideo, SendVoice,
    },
    prelude::Requester,
    requests::{HasPayload, Output, Request},
    types::*,
};

/// Default parse mode adaptor, see
/// [`RequesterExt::parse_mode`](crate::requests::RequesterExt::parse_mode).
#[derive(Clone, Debug)]
pub struct DefaultParseMode<B> {
    bot: B,
    mode: ParseMode,
}

/// Request returned by [`DefaultParseMode`] methods.
#[derive(Clone)]
pub struct DefaultParseModeRequest<R> {
    req: R,
    mode: ParseMode,
}

impl<B> DefaultParseMode<B> {
    /// Creates new [`DefaultParseMode`].
    ///
    /// Note: it's recommended to use [`RequesterExt::parse_mode`] instead.
    ///
    /// [`RequesterExt::parse_mode`]: crate::requests::RequesterExt::parse_mode
    pub fn new(bot: B, parse_mode: ParseMode) -> Self {
        Self { bot, mode: parse_mode }
    }

    /// Allows to access the inner bot.
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps the inner bot.
    pub fn into_inner(self) -> B {
        self.bot
    }

    /// Returns currently used [`ParseMode`].
    pub fn parse_mode(&self) -> ParseMode {
        self.mode
    }
}

impl<R> Request for DefaultParseModeRequest<R>
where
    R: Request + Clone,
    R::Payload: VisitParseModes,
{
    type Err = R::Err;
    type Send = R::Send;
    type SendRef = R::Send;

    // Required methods
    fn send(mut self) -> Self::Send {
        self.req.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(self.mode));
        self.req.send()
    }

    fn send_ref(&self) -> Self::SendRef {
        // There is no other way to change the payload, given a `&self` :(
        self.clone().send()
    }
}

impl<R> IntoFuture for DefaultParseModeRequest<R>
where
    Self: Request,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

impl<R> HasPayload for DefaultParseModeRequest<R>
where
    R: Request,
{
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.req.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.req.payload_ref()
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        {
            let req = $this.inner().$m($($arg),*);
            DefaultParseModeRequest { req, mode: $this.mode }
        }
    };
}

macro_rules! fty {
    ($T:ident) => {
        DefaultParseModeRequest<B::$T>
    };
}

macro_rules! ftyid {
    ($T:ident) => {
        B::$T
    };
}

macro_rules! fid {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

impl<B> Requester for DefaultParseMode<B>
where
    B: Requester,
    B::SendMessage: Clone,
    B::SendPhoto: Clone,
    B::SendVideo: Clone,
    B::SendAudio: Clone,
    B::SendDocument: Clone,
    B::SendAnimation: Clone,
    B::SendVoice: Clone,
    B::EditMessageText: Clone,
    B::EditMessageTextInline: Clone,
    B::EditMessageCaption: Clone,
    B::EditMessageCaptionInline: Clone,
    B::SendPoll: Clone,
    B::CopyMessage: Clone,
    B::AnswerInlineQuery: Clone,
    B::AnswerWebAppQuery: Clone,
    B::EditMessageMedia: Clone,
    B::EditMessageMediaInline: Clone,
    B::SendMediaGroup: Clone,
{
    type Err = B::Err;

    requester_forward! {
        send_message,
        send_photo,
        send_video,
        send_audio,
        send_document,
        send_animation,
        send_voice,
        send_poll,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline,
        copy_message,
        answer_inline_query,
        answer_web_app_query,
        send_media_group,
        edit_message_media,
        edit_message_media_inline,
        => f, fty
    }

    requester_forward! {
        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        forward_messages,
        copy_messages,
        send_video_note,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_venue,
        send_contact,
        send_dice,
        send_chat_action,
        set_message_reaction,
        get_user_profile_photos,
        get_file,
        kick_chat_member,
        ban_chat_member,
        unban_chat_member,
        restrict_chat_member,
        promote_chat_member,
        set_chat_administrator_custom_title,
        ban_chat_sender_chat,
        unban_chat_sender_chat,
        set_chat_permissions,
        export_chat_invite_link,
        create_chat_invite_link,
        edit_chat_invite_link,
        revoke_chat_invite_link,
        set_chat_photo,
        delete_chat_photo,
        set_chat_title,
        set_chat_description,
        pin_chat_message,
        unpin_chat_message,
        unpin_all_chat_messages,
        leave_chat,
        get_chat,
        get_chat_administrators,
        get_chat_members_count,
        get_chat_member_count,
        get_chat_member,
        set_chat_sticker_set,
        delete_chat_sticker_set,
        get_forum_topic_icon_stickers,
        create_forum_topic,
        edit_forum_topic,
        close_forum_topic,
        reopen_forum_topic,
        delete_forum_topic,
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_general_forum_topic_messages,
        answer_callback_query,
        get_user_chat_boosts,
        set_my_commands,
        get_business_connection,
        get_my_commands,
        set_my_name,
        get_my_name,
        set_my_description,
        get_my_description,
        set_my_short_description,
        get_my_short_description,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        delete_message,
        delete_messages,
        send_sticker,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        replace_sticker_in_set,
        set_sticker_set_thumbnail,
        set_custom_emoji_sticker_set_thumbnail,
        set_sticker_set_title,
        delete_sticker_set,
        set_sticker_emoji_list,
        set_sticker_keywords,
        set_sticker_mask_position,
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        get_star_transactions,
        refund_star_payment,
        set_passport_data_errors,
        send_game,
        set_game_score,
        set_game_score_inline,
        get_game_high_scores,
        approve_chat_join_request,
        decline_chat_join_request
        => fid, ftyid
    }
}

download_forward! {
    B
    DefaultParseMode<B>
    { this => this.inner() }
}

trait VisitParseModes {
    fn visit_parse_modes(&mut self, visitor: impl FnMut(&mut Option<ParseMode>));
}

macro_rules! impl_visit_parse_modes {
    (
        $(
            $T:ty => [
                $(
                    $field:ident
                ),*
            ]
            ,
        )*
    ) => {
        $(
            impl VisitParseModes for $T {
                fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
                    $(
                        visitor(&mut self.$field);
                    )*
                }
            }
        )*
    }
}

impl_visit_parse_modes! {
    SendMessage => [parse_mode],
    SendPhoto => [parse_mode],
    SendVideo => [parse_mode],
    SendAudio => [parse_mode],
    SendDocument => [parse_mode],
    SendAnimation => [parse_mode],
    SendVoice => [parse_mode],
    EditMessageText => [parse_mode],
    EditMessageTextInline => [parse_mode],
    EditMessageCaption => [parse_mode],
    EditMessageCaptionInline => [parse_mode],
    // FIXME: check if `parse_mode` changes anything if `.caption` is not set
    //        (and if it does, maybe not call visitor if `self.caption.is_none()`)
    CopyMessage => [parse_mode],
    SendPoll => [explanation_parse_mode],
}

impl VisitParseModes for AnswerInlineQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.results
            .iter_mut()
            .for_each(|result| visit_parse_modes_in_inline_query_result(result, &mut visitor))
    }
}

impl VisitParseModes for AnswerWebAppQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SendMediaGroup {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.media
            .iter_mut()
            .for_each(|media| visit_parse_modes_in_input_media(media, &mut visitor))
    }
}

impl VisitParseModes for EditMessageMedia {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

impl VisitParseModes for EditMessageMediaInline {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

fn visit_parse_modes_in_inline_query_result(
    result: &mut InlineQueryResult,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InlineQueryResult::*;

    let parse_mode = match result {
        // Simply contain `parse_mode`
        CachedAudio(r) => &mut r.parse_mode,
        CachedDocument(r) => &mut r.parse_mode,
        CachedGif(r) => &mut r.parse_mode,
        CachedMpeg4Gif(r) => &mut r.parse_mode,
        CachedPhoto(r) => &mut r.parse_mode,
        CachedVideo(r) => &mut r.parse_mode,
        CachedVoice(r) => &mut r.parse_mode,
        Audio(r) => &mut r.parse_mode,
        Document(r) => &mut r.parse_mode,
        Gif(r) => &mut r.parse_mode,
        Mpeg4Gif(r) => &mut r.parse_mode,
        Photo(r) => &mut r.parse_mode,
        Video(r) => &mut r.parse_mode,
        Voice(r) => &mut r.parse_mode,

        // Can contain parse mode if `InputMessageContent::Text`
        CachedSticker(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },
        Article(r) => match &mut r.input_message_content {
            InputMessageContent::Text(t) => &mut t.parse_mode,
            _ => return,
        },
        Contact(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },
        Location(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },
        Venue(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },

        // Can't contain `parse_mode` at all
        Game(_r) => return,
    };

    visitor(parse_mode);
}

fn visit_parse_modes_in_input_media(
    media: &mut InputMedia,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InputMedia::*;

    let parse_mode = match media {
        Photo(m) => &mut m.parse_mode,
        Video(m) => &mut m.parse_mode,
        Animation(m) => &mut m.parse_mode,
        Audio(m) => &mut m.parse_mode,
        Document(m) => &mut m.parse_mode,
    };

    visitor(parse_mode);
}
