use crate::{
    dispatching::{core::Guard, handlers::messages::message_parser::MessageParser},
    types,
    types::Message,
};

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn with_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.id))
    }

    pub fn with_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.date))
    }

    pub fn with_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.chat))
    }

    pub fn with_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.chat.id))
    }

    pub fn with_via_bot(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.with_guard(move |message: &Message| match &message.via_bot {
            Some(bot) => guard.check(bot),
            None => false,
        })
    }

    pub fn with_from(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn with_forward_from(self, guard: impl Guard<types::ForwardedFrom> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn with_forward_from_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_from_chat() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_forward_from_message_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_from_message_id() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_forward_signature(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_signature() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_forward_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_date() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_reply_to_message(self, guard: impl Guard<Message> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.reply_to_message() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_edit_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.edit_date() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_media_group_id(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.media_group_id() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_text(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.text() {
            Some(text) => guard.check(text),
            None => false,
        })
    }

    pub fn with_entities(self, guard: impl Guard<[types::MessageEntity]> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.entities() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_any_entity(self, guard: impl Guard<types::MessageEntity> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.entities() {
            Some(x) => x.iter().any(|ent| guard.check(ent)),
            None => false,
        })
    }

    pub fn with_caption_entities(self, guard: impl Guard<[types::MessageEntity]> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.caption_entities() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_any_caption_entity(self, guard: impl Guard<types::MessageEntity> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.caption_entities() {
            Some(x) => x.iter().any(|ent| guard.check(ent)),
            None => false,
        })
    }

    pub fn with_any_entity_or_caption_entity(self, guard: impl Guard<types::MessageEntity> + Clone + 'static) -> Self {
        self.with_any_entity(guard.clone())
            .or_with_any_caption_entity(guard)
    }

    pub fn with_audio(self, guard: impl Guard<types::Audio> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.audio() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_document(self, guard: impl Guard<types::Document> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.document() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_animation(self, guard: impl Guard<types::Animation> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.animation() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_game(self, guard: impl Guard<types::Game> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.game() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_photo(self, guard: impl Guard<[types::PhotoSize]> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.photo() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_sticker(self, guard: impl Guard<types::Sticker> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.sticker() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_video(self, guard: impl Guard<types::Video> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.video() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_voice(self, guard: impl Guard<types::Voice> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.voice() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_video_note(self, guard: impl Guard<types::VideoNote> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.video_note() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_caption(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.caption() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_text_or_caption(self, guard: impl Guard<str> + Clone + 'static) -> Self {
        self.with_text(guard.clone())
            .or_with_caption(guard)
    }

    pub fn with_contact(self, guard: impl Guard<types::Contact> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.contact() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_location(self, guard: impl Guard<types::Location> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.location() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_venue(self, guard: impl Guard<types::Venue> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.venue() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_poll(self, guard: impl Guard<types::Poll> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.poll() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_new_chat_members(self, guard: impl Guard<[types::User]> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.new_chat_members() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_left_chat_member(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.left_chat_member() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_new_chat_title(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.new_chat_title() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_new_chat_photo(self, guard: impl Guard<[types::PhotoSize]> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.new_chat_photo() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_delete_chat_photo(self, is_deleted: bool) -> Self {
        self.with_guard(move |message: &Message| match message.delete_chat_photo() {
            Some(_) => is_deleted == true,
            None => is_deleted == false,
        })
    }

    pub fn with_group_chat_created(self, is_created: bool) -> Self {
        self.with_guard(move |message: &Message| match message.group_chat_created() {
            Some(_) => is_created == true,
            None => is_created == false,
        })
    }

    pub fn with_super_group_chat_created(self, is_created: bool) -> Self {
        self.with_guard(move |message: &Message| match message.super_group_chat_created() {
            Some(_) => is_created == true,
            None => is_created == false,
        })
    }

    pub fn with_channel_chat_created(self, is_created: bool) -> Self {
        self.with_guard(move |message: &Message| match message.channel_chat_created() {
            Some(_) => is_created == true,
            None => is_created == false,
        })
    }

    pub fn with_migrate_to_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.migrate_to_chat_id() {
            Some(x) => guard.check(&x),
            None => false,
        })
    }

    pub fn with_migrate_from_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.migrate_from_chat_id() {
            Some(x) => guard.check(&x),
            None => false,
        })
    }

    pub fn with_pinned_message(self, guard: impl Guard<Message> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.pinned_message() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_invoice(self, guard: impl Guard<types::Invoice> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.invoice() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_successful_payment(self, guard: impl Guard<types::SuccessfulPayment> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.successful_payment() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_connected_website(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.connected_website() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn with_passport_data(self, guard: impl Guard<types::PassportData> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.passport_data() {
            Some(x) => guard.check(x),
            None => false,
        })
    }
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn or_with_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| guard.check(&message.id))
    }

    pub fn or_with_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| guard.check(&message.date))
    }

    pub fn or_with_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| guard.check(&message.chat))
    }

    pub fn or_with_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| guard.check(&message.chat.id))
    }

    pub fn or_with_via_bot(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match &message.via_bot {
            Some(bot) => guard.check(bot),
            None => false,
        })
    }

    pub fn or_with_from(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn or_with_forward_from(self, guard: impl Guard<types::ForwardedFrom> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.forward_from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn or_with_forward_from_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.forward_from_chat() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_forward_from_message_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.forward_from_message_id() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_forward_signature(self, guard: impl Guard<str> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.forward_signature() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_forward_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.forward_date() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_reply_to_message(self, guard: impl Guard<Message> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.reply_to_message() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_edit_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.edit_date() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_media_group_id(self, guard: impl Guard<str> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.media_group_id() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_text(self, guard: impl Guard<str> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.text() {
            Some(text) => guard.check(text),
            None => false,
        })
    }

    pub fn or_with_entities(self, guard: impl Guard<[types::MessageEntity]> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.entities() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_any_entity(self, guard: impl Guard<types::MessageEntity> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.entities() {
            Some(x) => x.iter().any(|ent| guard.check(ent)),
            None => false,
        })
    }

    pub fn or_with_caption_entities(self, guard: impl Guard<[types::MessageEntity]> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.caption_entities() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_any_caption_entity(self, guard: impl Guard<types::MessageEntity> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.caption_entities() {
            Some(x) => x.iter().any(|ent| guard.check(ent)),
            None => false,
        })
    }

    pub fn or_with_any_entity_or_caption_entity(self, guard: impl Guard<types::MessageEntity> + Clone + 'static) -> Self {
        self.or_with_any_entity(guard.clone())
            .or_with_any_caption_entity(guard)
    }

    pub fn or_with_audio(self, guard: impl Guard<types::Audio> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.audio() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_document(self, guard: impl Guard<types::Document> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.document() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_animation(self, guard: impl Guard<types::Animation> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.animation() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_game(self, guard: impl Guard<types::Game> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.game() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_photo(self, guard: impl Guard<[types::PhotoSize]> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.photo() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_sticker(self, guard: impl Guard<types::Sticker> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.sticker() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_video(self, guard: impl Guard<types::Video> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.video() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_voice(self, guard: impl Guard<types::Voice> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.voice() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_video_note(self, guard: impl Guard<types::VideoNote> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.video_note() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_caption(self, guard: impl Guard<str> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.caption() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_text_or_caption(self, guard: impl Guard<str> + Clone + 'static) -> Self {
        self.or_with_text(guard.clone())
            .or_with_caption(guard)
    }

    pub fn or_with_contact(self, guard: impl Guard<types::Contact> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.contact() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_location(self, guard: impl Guard<types::Location> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.location() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_venue(self, guard: impl Guard<types::Venue> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.venue() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_poll(self, guard: impl Guard<types::Poll> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.poll() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_new_chat_members(self, guard: impl Guard<[types::User]> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.new_chat_members() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_left_chat_member(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.left_chat_member() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_new_chat_title(self, guard: impl Guard<str> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.new_chat_title() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_new_chat_photo(self, guard: impl Guard<[types::PhotoSize]> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.new_chat_photo() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_delete_chat_photo(self, is_deleted: bool) -> Self {
        self.or_with_guard(move |message: &Message| match message.delete_chat_photo() {
            Some(_) => is_deleted == true,
            None => is_deleted == false,
        })
    }

    pub fn or_with_group_chat_created(self, is_created: bool) -> Self {
        self.or_with_guard(move |message: &Message| match message.group_chat_created() {
            Some(_) => is_created == true,
            None => is_created == false,
        })
    }

    pub fn or_with_super_group_chat_created(self, is_created: bool) -> Self {
        self.or_with_guard(move |message: &Message| match message.super_group_chat_created() {
            Some(_) => is_created == true,
            None => is_created == false,
        })
    }

    pub fn or_with_channel_chat_created(self, is_created: bool) -> Self {
        self.or_with_guard(move |message: &Message| match message.channel_chat_created() {
            Some(_) => is_created == true,
            None => is_created == false,
        })
    }

    pub fn or_with_migrate_to_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.migrate_to_chat_id() {
            Some(x) => guard.check(&x),
            None => false,
        })
    }

    pub fn or_with_migrate_from_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.migrate_from_chat_id() {
            Some(x) => guard.check(&x),
            None => false,
        })
    }

    pub fn or_with_pinned_message(self, guard: impl Guard<Message> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.pinned_message() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_invoice(self, guard: impl Guard<types::Invoice> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.invoice() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_successful_payment(self, guard: impl Guard<types::SuccessfulPayment> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.successful_payment() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_connected_website(self, guard: impl Guard<str> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.connected_website() {
            Some(x) => guard.check(x),
            None => false,
        })
    }

    pub fn or_with_passport_data(self, guard: impl Guard<types::PassportData> + 'static) -> Self {
        self.or_with_guard(move |message: &Message| match message.passport_data() {
            Some(x) => guard.check(x),
            None => false,
        })
    }
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn has_via_bot(self) -> Self {
        self.with_guard(move |message: &Message| message.via_bot.is_some())
    }

    pub fn has_from(self) -> Self {
        self.with_guard(|message: &Message| message.from().is_some())
    }

    pub fn has_forward_from(self) -> Self {
        self.with_guard(|message: &Message| message.forward_from().is_some())
    }

    pub fn has_forward_from_chat(self) -> Self {
        self.with_guard(|message: &Message| message.forward_from_chat().is_some())
    }

    pub fn has_forward_from_message_id(self) -> Self {
        self.with_guard(move |message: &Message| message.forward_from_message_id().is_some())
    }

    pub fn has_forward_signature(self) -> Self {
        self.with_guard(|message: &Message| message.forward_signature().is_some())
    }

    pub fn has_forward_date(self) -> Self {
        self.with_guard(move |message: &Message| message.forward_date().is_some())
    }

    pub fn has_reply_to_message(self) -> Self {
        self.with_guard(|message: &Message| message.reply_to_message().is_some())
    }

    pub fn has_edit_date(self) -> Self {
        self.with_guard(move |message: &Message| message.edit_date().is_some())
    }

    pub fn has_media_group_id(self) -> Self {
        self.with_guard(|message: &Message| message.media_group_id().is_some())
    }

    pub fn has_text(self) -> Self {
        self.with_guard(|message: &Message| message.text().is_some())
    }

    pub fn has_entities(self) -> Self {
        self.with_guard(move |message: &Message| message.entities().is_some())
    }

    pub fn has_caption_entities(self) -> Self {
        self.with_guard(move |message: &Message| message.caption_entities().is_some())
    }

    pub fn has_entities_or_caption_entities(self) -> Self {
        self.has_entities()
            .or_has_caption_entities()
    }

    pub fn has_audio(self) -> Self {
        self.with_guard(|message: &Message| message.audio().is_some())
    }

    pub fn has_document(self) -> Self {
        self.with_guard(|message: &Message| message.document().is_some())
    }

    pub fn has_animation(self) -> Self {
        self.with_guard(|message: &Message| message.animation().is_some())
    }

    pub fn has_game(self) -> Self {
        self.with_guard(|message: &Message| message.game().is_some())
    }

    pub fn has_photo(self) -> Self {
        self.with_guard(move |message: &Message| message.photo().is_some())
    }

    pub fn has_sticker(self) -> Self {
        self.with_guard(|message: &Message| message.sticker().is_some())
    }

    pub fn has_video(self) -> Self {
        self.with_guard(|message: &Message| message.video().is_some())
    }

    pub fn has_voice(self) -> Self {
        self.with_guard(|message: &Message| message.voice().is_some())
    }

    pub fn has_video_note(self) -> Self {
        self.with_guard(|message: &Message| message.video_note().is_some())
    }

    pub fn has_caption(self) -> Self {
        self.with_guard(|message: &Message| message.caption().is_some())
    }

    pub fn has_text_or_caption(self) -> Self {
        self.has_text()
            .or_has_caption()
    }

    pub fn has_contact(self) -> Self {
        self.with_guard(|message: &Message| message.contact().is_some())
    }

    pub fn has_location(self) -> Self {
        self.with_guard(|message: &Message| message.location().is_some())
    }

    pub fn has_venue(self) -> Self {
        self.with_guard(|message: &Message| message.venue().is_some())
    }

    pub fn has_poll(self) -> Self {
        self.with_guard(|message: &Message| message.poll().is_some())
    }

    pub fn has_new_chat_members(self) -> Self {
        self.with_guard(move |message: &Message| message.new_chat_members().is_some())
    }

    pub fn has_left_chat_member(self) -> Self {
        self.with_guard(|message: &Message| message.left_chat_member().is_some())
    }

    pub fn has_new_chat_title(self) -> Self {
        self.with_guard(|message: &Message| message.new_chat_title().is_some())
    }

    pub fn has_new_chat_photo(self) -> Self {
        self.with_guard(move |message: &Message| message.new_chat_photo().is_some())
    }

    pub fn has_delete_chat_photo(self) -> Self {
        self.with_guard(|message: &Message| message.delete_chat_photo().is_some())
    }

    pub fn has_group_chat_created(self) -> Self {
        self.with_guard(|message: &Message| message.group_chat_created().is_some())
    }

    pub fn has_super_group_chat_created(self) -> Self {
        self.with_guard(|message: &Message| message.super_group_chat_created().is_some())
    }

    pub fn has_channel_chat_created(self) -> Self {
        self.with_guard(|message: &Message| message.channel_chat_created().is_some())
    }

    pub fn has_migrate_to_chat_id(self) -> Self {
        self.with_guard(move |message: &Message| message.migrate_to_chat_id().is_some())
    }

    pub fn has_migrate_from_chat_id(self) -> Self {
        self.with_guard(move |message: &Message| message.migrate_from_chat_id().is_some())
    }

    pub fn has_pinned_message(self) -> Self {
        self.with_guard(|message: &Message| message.pinned_message().is_some())
    }

    pub fn has_invoice(self) -> Self {
        self.with_guard(|message: &Message| message.invoice().is_some())
    }

    pub fn has_successful_payment(self) -> Self {
        self.with_guard(|message: &Message| message.successful_payment().is_some())
    }

    pub fn has_connected_website(self) -> Self {
        self.with_guard(|message: &Message| message.connected_website().is_some())
    }

    pub fn has_passport_data(self) -> Self {
        self.with_guard(|message: &Message| message.passport_data().is_some())
    }
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn or_has_via_bot(self) -> Self {
        self.or_with_guard(move |message: &Message| message.via_bot.is_some())
    }

    pub fn or_has_from(self) -> Self {
        self.or_with_guard(|message: &Message| message.from().is_some())
    }

    pub fn or_has_forward_from(self) -> Self {
        self.or_with_guard(|message: &Message| message.forward_from().is_some())
    }

    pub fn or_has_forward_from_chat(self) -> Self {
        self.or_with_guard(|message: &Message| message.forward_from_chat().is_some())
    }

    pub fn or_has_forward_from_message_id(self) -> Self {
        self.or_with_guard(move |message: &Message| message.forward_from_message_id().is_some())
    }

    pub fn or_has_forward_signature(self) -> Self {
        self.or_with_guard(|message: &Message| message.forward_signature().is_some())
    }

    pub fn or_has_forward_date(self) -> Self {
        self.or_with_guard(move |message: &Message| message.forward_date().is_some())
    }

    pub fn or_has_reply_to_message(self) -> Self {
        self.or_with_guard(|message: &Message| message.reply_to_message().is_some())
    }

    pub fn or_has_edit_date(self) -> Self {
        self.or_with_guard(move |message: &Message| message.edit_date().is_some())
    }

    pub fn or_has_media_group_id(self) -> Self {
        self.or_with_guard(|message: &Message| message.media_group_id().is_some())
    }

    pub fn or_has_text(self) -> Self {
        self.or_with_guard(|message: &Message| message.text().is_some())
    }

    pub fn or_has_entities(self) -> Self {
        self.or_with_guard(move |message: &Message| message.entities().is_some())
    }

    pub fn or_has_caption_entities(self) -> Self {
        self.or_with_guard(move |message: &Message| message.caption_entities().is_some())
    }

    pub fn or_has_entities_or_caption_entities(self) -> Self {
        self.or_has_entities()
            .or_has_caption_entities()
    }

    pub fn or_has_audio(self) -> Self {
        self.or_with_guard(|message: &Message| message.audio().is_some())
    }

    pub fn or_has_document(self) -> Self {
        self.or_with_guard(|message: &Message| message.document().is_some())
    }

    pub fn or_has_animation(self) -> Self {
        self.or_with_guard(|message: &Message| message.animation().is_some())
    }

    pub fn or_has_game(self) -> Self {
        self.or_with_guard(|message: &Message| message.game().is_some())
    }

    pub fn or_has_photo(self) -> Self {
        self.or_with_guard(move |message: &Message| message.photo().is_some())
    }

    pub fn or_has_sticker(self) -> Self {
        self.or_with_guard(|message: &Message| message.sticker().is_some())
    }

    pub fn or_has_video(self) -> Self {
        self.or_with_guard(|message: &Message| message.video().is_some())
    }

    pub fn or_has_voice(self) -> Self {
        self.or_with_guard(|message: &Message| message.voice().is_some())
    }

    pub fn or_has_video_note(self) -> Self {
        self.or_with_guard(|message: &Message| message.video_note().is_some())
    }

    pub fn or_has_caption(self) -> Self {
        self.or_with_guard(|message: &Message| message.caption().is_some())
    }

    pub fn or_has_text_or_caption(self) -> Self {
        self.or_has_text()
            .or_has_caption()
    }

    pub fn or_has_contact(self) -> Self {
        self.or_with_guard(|message: &Message| message.contact().is_some())
    }

    pub fn or_has_location(self) -> Self {
        self.or_with_guard(|message: &Message| message.location().is_some())
    }

    pub fn or_has_venue(self) -> Self {
        self.or_with_guard(|message: &Message| message.venue().is_some())
    }

    pub fn or_has_poll(self) -> Self {
        self.or_with_guard(|message: &Message| message.poll().is_some())
    }

    pub fn or_has_new_chat_members(self) -> Self {
        self.or_with_guard(move |message: &Message| message.new_chat_members().is_some())
    }

    pub fn or_has_left_chat_member(self) -> Self {
        self.or_with_guard(|message: &Message| message.left_chat_member().is_some())
    }

    pub fn or_has_new_chat_title(self) -> Self {
        self.or_with_guard(|message: &Message| message.new_chat_title().is_some())
    }

    pub fn or_has_new_chat_photo(self) -> Self {
        self.or_with_guard(move |message: &Message| message.new_chat_photo().is_some())
    }

    pub fn or_has_delete_chat_photo(self) -> Self {
        self.or_with_guard(|message: &Message| message.delete_chat_photo().is_some())
    }

    pub fn or_has_group_chat_created(self) -> Self {
        self.or_with_guard(|message: &Message| message.group_chat_created().is_some())
    }

    pub fn or_has_super_group_chat_created(self) -> Self {
        self.or_with_guard(|message: &Message| message.super_group_chat_created().is_some())
    }

    pub fn or_has_channel_chat_created(self) -> Self {
        self.or_with_guard(|message: &Message| message.channel_chat_created().is_some())
    }

    pub fn or_has_migrate_to_chat_id(self) -> Self {
        self.or_with_guard(move |message: &Message| message.migrate_to_chat_id().is_some())
    }

    pub fn or_has_migrate_from_chat_id(self) -> Self {
        self.or_with_guard(move |message: &Message| message.migrate_from_chat_id().is_some())
    }

    pub fn or_has_pinned_message(self) -> Self {
        self.or_with_guard(|message: &Message| message.pinned_message().is_some())
    }

    pub fn or_has_invoice(self) -> Self {
        self.or_with_guard(|message: &Message| message.invoice().is_some())
    }

    pub fn or_has_successful_payment(self) -> Self {
        self.or_with_guard(|message: &Message| message.successful_payment().is_some())
    }

    pub fn or_has_connected_website(self) -> Self {
        self.or_with_guard(|message: &Message| message.connected_website().is_some())
    }

    pub fn or_has_passport_data(self) -> Self {
        self.or_with_guard(|message: &Message| message.passport_data().is_some())
    }
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn no_has_via_bot(self) -> Self {
        self.with_guard(move |message: &Message| message.via_bot.is_none())
    }

    pub fn no_has_from(self) -> Self {
        self.with_guard(|message: &Message| message.from().is_none())
    }

    pub fn no_has_forward_from(self) -> Self {
        self.with_guard(|message: &Message| message.forward_from().is_none())
    }

    pub fn no_has_forward_from_chat(self) -> Self {
        self.with_guard(|message: &Message| message.forward_from_chat().is_none())
    }

    pub fn no_has_forward_from_message_id(self) -> Self {
        self.with_guard(move |message: &Message| message.forward_from_message_id().is_none())
    }

    pub fn no_has_forward_signature(self) -> Self {
        self.with_guard(|message: &Message| message.forward_signature().is_none())
    }

    pub fn no_has_forward_date(self) -> Self {
        self.with_guard(move |message: &Message| message.forward_date().is_none())
    }

    pub fn no_has_reply_to_message(self) -> Self {
        self.with_guard(|message: &Message| message.reply_to_message().is_none())
    }

    pub fn no_has_edit_date(self) -> Self {
        self.with_guard(move |message: &Message| message.edit_date().is_none())
    }

    pub fn no_has_media_group_id(self) -> Self {
        self.with_guard(|message: &Message| message.media_group_id().is_none())
    }

    pub fn no_has_text(self) -> Self {
        self.with_guard(|message: &Message| message.text().is_none())
    }

    pub fn no_has_entities(self) -> Self {
        self.with_guard(move |message: &Message| message.entities().is_none())
    }

    pub fn no_has_caption_entities(self) -> Self {
        self.with_guard(move |message: &Message| message.caption_entities().is_none())
    }

    pub fn no_has_entities_or_caption_entities(self) -> Self {
        self.no_has_entities()
            .or_no_has_caption_entities()
    }

    pub fn no_has_audio(self) -> Self {
        self.with_guard(|message: &Message| message.audio().is_none())
    }

    pub fn no_has_document(self) -> Self {
        self.with_guard(|message: &Message| message.document().is_none())
    }

    pub fn no_has_animation(self) -> Self {
        self.with_guard(|message: &Message| message.animation().is_none())
    }

    pub fn no_has_game(self) -> Self {
        self.with_guard(|message: &Message| message.game().is_none())
    }

    pub fn no_has_photo(self) -> Self {
        self.with_guard(move |message: &Message| message.photo().is_none())
    }

    pub fn no_has_sticker(self) -> Self {
        self.with_guard(|message: &Message| message.sticker().is_none())
    }

    pub fn no_has_video(self) -> Self {
        self.with_guard(|message: &Message| message.video().is_none())
    }

    pub fn no_has_voice(self) -> Self {
        self.with_guard(|message: &Message| message.voice().is_none())
    }

    pub fn no_has_video_note(self) -> Self {
        self.with_guard(|message: &Message| message.video_note().is_none())
    }

    pub fn no_has_caption(self) -> Self {
        self.with_guard(|message: &Message| message.caption().is_none())
    }

    pub fn no_has_text_or_caption(self) -> Self {
        self.no_has_text()
            .or_no_has_caption()
    }

    pub fn no_has_contact(self) -> Self {
        self.with_guard(|message: &Message| message.contact().is_none())
    }

    pub fn no_has_location(self) -> Self {
        self.with_guard(|message: &Message| message.location().is_none())
    }

    pub fn no_has_venue(self) -> Self {
        self.with_guard(|message: &Message| message.venue().is_none())
    }

    pub fn no_has_poll(self) -> Self {
        self.with_guard(|message: &Message| message.poll().is_none())
    }

    pub fn no_has_new_chat_members(self) -> Self {
        self.with_guard(move |message: &Message| message.new_chat_members().is_none())
    }

    pub fn no_has_left_chat_member(self) -> Self {
        self.with_guard(|message: &Message| message.left_chat_member().is_none())
    }

    pub fn no_has_new_chat_title(self) -> Self {
        self.with_guard(|message: &Message| message.new_chat_title().is_none())
    }

    pub fn no_has_new_chat_photo(self) -> Self {
        self.with_guard(move |message: &Message| message.new_chat_photo().is_none())
    }

    pub fn no_has_delete_chat_photo(self) -> Self {
        self.with_guard(|message: &Message| message.delete_chat_photo().is_none())
    }

    pub fn no_has_group_chat_created(self) -> Self {
        self.with_guard(|message: &Message| message.group_chat_created().is_none())
    }

    pub fn no_has_super_group_chat_created(self) -> Self {
        self.with_guard(|message: &Message| message.super_group_chat_created().is_none())
    }

    pub fn no_has_channel_chat_created(self) -> Self {
        self.with_guard(|message: &Message| message.channel_chat_created().is_none())
    }

    pub fn no_has_migrate_to_chat_id(self) -> Self {
        self.with_guard(move |message: &Message| message.migrate_to_chat_id().is_none())
    }

    pub fn no_has_migrate_from_chat_id(self) -> Self {
        self.with_guard(move |message: &Message| message.migrate_from_chat_id().is_none())
    }

    pub fn no_has_pinned_message(self) -> Self {
        self.with_guard(|message: &Message| message.pinned_message().is_none())
    }

    pub fn no_has_invoice(self) -> Self {
        self.with_guard(|message: &Message| message.invoice().is_none())
    }

    pub fn no_has_successful_payment(self) -> Self {
        self.with_guard(|message: &Message| message.successful_payment().is_none())
    }

    pub fn no_has_connected_website(self) -> Self {
        self.with_guard(|message: &Message| message.connected_website().is_none())
    }

    pub fn no_has_passport_data(self) -> Self {
        self.with_guard(|message: &Message| message.passport_data().is_none())
    }
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn or_no_has_via_bot(self) -> Self {
        self.or_with_guard(move |message: &Message| message.via_bot.is_none())
    }

    pub fn or_no_has_from(self) -> Self {
        self.or_with_guard(|message: &Message| message.from().is_none())
    }

    pub fn or_no_has_forward_from(self) -> Self {
        self.or_with_guard(|message: &Message| message.forward_from().is_none())
    }

    pub fn or_no_has_forward_from_chat(self) -> Self {
        self.or_with_guard(|message: &Message| message.forward_from_chat().is_none())
    }

    pub fn or_no_has_forward_from_message_id(self) -> Self {
        self.or_with_guard(move |message: &Message| message.forward_from_message_id().is_none())
    }

    pub fn or_no_has_forward_signature(self) -> Self {
        self.or_with_guard(|message: &Message| message.forward_signature().is_none())
    }

    pub fn or_no_has_forward_date(self) -> Self {
        self.or_with_guard(move |message: &Message| message.forward_date().is_none())
    }

    pub fn or_no_has_reply_to_message(self) -> Self {
        self.or_with_guard(|message: &Message| message.reply_to_message().is_none())
    }

    pub fn or_no_has_edit_date(self) -> Self {
        self.or_with_guard(move |message: &Message| message.edit_date().is_none())
    }

    pub fn or_no_has_media_group_id(self) -> Self {
        self.or_with_guard(|message: &Message| message.media_group_id().is_none())
    }

    pub fn or_no_has_text(self) -> Self {
        self.or_with_guard(|message: &Message| message.text().is_none())
    }

    pub fn or_no_has_entities(self) -> Self {
        self.or_with_guard(move |message: &Message| message.entities().is_none())
    }

    pub fn or_no_has_caption_entities(self) -> Self {
        self.or_with_guard(move |message: &Message| message.caption_entities().is_none())
    }

    pub fn or_no_has_entities_or_caption_entities(self) -> Self {
        self.or_no_has_entities()
            .or_no_has_caption_entities()
    }

    pub fn or_no_has_audio(self) -> Self {
        self.or_with_guard(|message: &Message| message.audio().is_none())
    }

    pub fn or_no_has_document(self) -> Self {
        self.or_with_guard(|message: &Message| message.document().is_none())
    }

    pub fn or_no_has_animation(self) -> Self {
        self.or_with_guard(|message: &Message| message.animation().is_none())
    }

    pub fn or_no_has_game(self) -> Self {
        self.or_with_guard(|message: &Message| message.game().is_none())
    }

    pub fn or_no_has_photo(self) -> Self {
        self.or_with_guard(move |message: &Message| message.photo().is_none())
    }

    pub fn or_no_has_sticker(self) -> Self {
        self.or_with_guard(|message: &Message| message.sticker().is_none())
    }

    pub fn or_no_has_video(self) -> Self {
        self.or_with_guard(|message: &Message| message.video().is_none())
    }

    pub fn or_no_has_voice(self) -> Self {
        self.or_with_guard(|message: &Message| message.voice().is_none())
    }

    pub fn or_no_has_video_note(self) -> Self {
        self.or_with_guard(|message: &Message| message.video_note().is_none())
    }

    pub fn or_no_has_caption(self) -> Self {
        self.or_with_guard(|message: &Message| message.caption().is_none())
    }

    pub fn or_no_has_text_or_caption(self) -> Self {
        self.or_no_has_text()
            .or_no_has_caption()
    }

    pub fn or_no_has_contact(self) -> Self {
        self.or_with_guard(|message: &Message| message.contact().is_none())
    }

    pub fn or_no_has_location(self) -> Self {
        self.or_with_guard(|message: &Message| message.location().is_none())
    }

    pub fn or_no_has_venue(self) -> Self {
        self.or_with_guard(|message: &Message| message.venue().is_none())
    }

    pub fn or_no_has_poll(self) -> Self {
        self.or_with_guard(|message: &Message| message.poll().is_none())
    }

    pub fn or_no_has_new_chat_members(self) -> Self {
        self.or_with_guard(move |message: &Message| message.new_chat_members().is_none())
    }

    pub fn or_no_has_left_chat_member(self) -> Self {
        self.or_with_guard(|message: &Message| message.left_chat_member().is_none())
    }

    pub fn or_no_has_new_chat_title(self) -> Self {
        self.or_with_guard(|message: &Message| message.new_chat_title().is_none())
    }

    pub fn or_no_has_new_chat_photo(self) -> Self {
        self.or_with_guard(move |message: &Message| message.new_chat_photo().is_none())
    }

    pub fn or_no_has_delete_chat_photo(self) -> Self {
        self.or_with_guard(|message: &Message| message.delete_chat_photo().is_none())
    }

    pub fn or_no_has_group_chat_created(self) -> Self {
        self.or_with_guard(|message: &Message| message.group_chat_created().is_none())
    }

    pub fn or_no_has_super_group_chat_created(self) -> Self {
        self.or_with_guard(|message: &Message| message.super_group_chat_created().is_none())
    }

    pub fn or_no_has_channel_chat_created(self) -> Self {
        self.or_with_guard(|message: &Message| message.channel_chat_created().is_none())
    }

    pub fn or_no_has_migrate_to_chat_id(self) -> Self {
        self.or_with_guard(move |message: &Message| message.migrate_to_chat_id().is_none())
    }

    pub fn or_no_has_migrate_from_chat_id(self) -> Self {
        self.or_with_guard(move |message: &Message| message.migrate_from_chat_id().is_none())
    }

    pub fn or_no_has_pinned_message(self) -> Self {
        self.or_with_guard(|message: &Message| message.pinned_message().is_none())
    }

    pub fn or_no_has_invoice(self) -> Self {
        self.or_with_guard(|message: &Message| message.invoice().is_none())
    }

    pub fn or_no_has_successful_payment(self) -> Self {
        self.or_with_guard(|message: &Message| message.successful_payment().is_none())
    }

    pub fn or_no_has_connected_website(self) -> Self {
        self.or_with_guard(|message: &Message| message.connected_website().is_none())
    }

    pub fn or_no_has_passport_data(self) -> Self {
        self.or_with_guard(|message: &Message| message.passport_data().is_none())
    }
}
